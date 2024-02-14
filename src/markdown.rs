use anyhow::{anyhow, Result};
use pulldown_cmark::CodeBlockKind::Fenced;
use pulldown_cmark::Event::{End, Html, Start, Text};
use pulldown_cmark::Tag::{CodeBlock, FootnoteDefinition, Paragraph};
use pulldown_cmark::{html, CowStr, Event, Options, Parser};
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

pub struct Markdowner {
    syntax_set: SyntaxSet,
}

impl Markdowner {
    pub fn new(syntax_set: SyntaxSet) -> Markdowner {
        Markdowner { syntax_set }
    }

    pub fn to_html(&self, markdown: &str) -> Result<String> {
        let mut rendered_content = String::new();
        let parser = Parser::new_ext(
            markdown,
            Options::ENABLE_TABLES | Options::ENABLE_FOOTNOTES | Options::ENABLE_STRIKETHROUGH,
        );
        let mut code_handler = CodeHandler::new(&self.syntax_set);
        let mut footnote_handler = FootnoteHandler::new();
        let events = parser
            .flat_map(|e| footnote_handler.handle_event(e))
            .map(|e| code_handler.handle_event(e))
            .collect::<Result<Vec<_>>>()?
            .into_iter();
        html::push_html(&mut rendered_content, events);
        Ok(rendered_content)
    }
}

struct CodeHandler<'a> {
    syntax_set: &'a SyntaxSet,
    current_lang: Option<String>,
}

impl<'a> CodeHandler<'a> {
    fn new(syntax_set: &'a SyntaxSet) -> CodeHandler<'a> {
        CodeHandler {
            syntax_set,
            current_lang: None,
        }
    }

    fn handle_event<'e>(&mut self, event: Event<'e>) -> Result<Event<'e>> {
        let event = match event {
            Start(CodeBlock(Fenced(lang))) => self.start_fenced_code(lang),
            End(CodeBlock(Fenced(_))) => self.end_fenced_code(),
            Text(text) => self.handle_text(text)?,
            _ => event,
        };
        Ok(event)
    }

    fn start_fenced_code<'e>(&mut self, lang: CowStr<'e>) -> Event<'e> {
        self.current_lang = Some(lang.to_string());
        Html("<pre class=\"code\"><code>".into())
    }

    fn end_fenced_code<'e>(&mut self) -> Event<'e> {
        self.current_lang = None;
        Html("</code></pre>".into())
    }

    fn handle_text<'e>(&self, text: CowStr<'e>) -> Result<Event<'e>> {
        if let Some(lang) = &self.current_lang {
            self.code_html(&text, lang)
        } else {
            Ok(Event::Text(text))
        }
    }

    fn code_html<'e>(&self, code: &str, lang: &str) -> Result<Event<'e>> {
        let formatted_code = self.format_code(code, lang)?;
        Ok(Html(formatted_code.into()))
    }

    fn format_code(&self, code: &str, lang: &str) -> Result<String> {
        let mut html_generator = self.html_generator(lang)?;
        for line in LinesWithEndings::from(code) {
            html_generator.parse_html_for_line_which_includes_newline(line)?;
        }
        Ok(html_generator.finalize())
    }

    fn html_generator(&self, lang: &str) -> Result<ClassedHTMLGenerator> {
        let generator = ClassedHTMLGenerator::new_with_class_style(
            self.syntax_for_lang(lang)?,
            self.syntax_set,
            ClassStyle::Spaced,
        );
        Ok(generator)
    }

    fn syntax_for_lang(&self, lang: &str) -> Result<&SyntaxReference> {
        self.syntax_set
            .find_syntax_by_extension(lang)
            .ok_or(anyhow!("unknown language extension {lang}"))
    }
}

struct FootnoteHandler {
    footnote_number_on_next_paragraph: bool,
    footnote_counter: u32,
}

impl FootnoteHandler {
    fn new() -> FootnoteHandler {
        FootnoteHandler {
            footnote_number_on_next_paragraph: false,
            footnote_counter: 1,
        }
    }

    fn handle_event<'e>(&mut self, event: Event<'e>) -> Vec<Event<'e>> {
        match event {
            Start(FootnoteDefinition(label)) => self.start_footnote(label),
            End(FootnoteDefinition(_)) => self.end_footnote(),
            Start(Paragraph) => self.start_paragraph(),
            _ => vec![event],
        }
    }

    fn start_footnote<'e>(&mut self, label: CowStr<'e>) -> Vec<Event<'e>> {
        self.footnote_number_on_next_paragraph = true;
        let open_div = format!("<div id=\"{}\" class=\"footnote\">", label);
        vec![Html(CowStr::from(open_div))]
    }

    fn end_footnote<'e>(&mut self) -> Vec<Event<'e>> {
        vec![Event::Html(CowStr::from("</div>\n"))]
    }

    fn start_paragraph<'e>(&mut self) -> Vec<Event<'e>> {
        let mut events = vec![Start(Paragraph)];
        if self.footnote_number_on_next_paragraph {
            events.push(self.footnote_number());
        }
        events
    }

    fn footnote_number<'e>(&mut self) -> Event<'e> {
        let text = format!("[{}]: ", self.footnote_counter);
        self.footnote_counter += 1;
        self.footnote_number_on_next_paragraph = false;
        Text(CowStr::from(text))
    }
}
