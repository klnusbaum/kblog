use crate::config::Metadata;
use crate::css::CSSCreator;
use crate::document::{RawDraft, RawPost, RenderedDraft, RenderedPost};
use crate::feed::FeedCreator;
use crate::markdown::Markdowner;
use crate::{css, feed, templates};
use anyhow::{anyhow, bail, Context, Error, Result};
use askama::Template;
use std::cmp::Ordering;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

const OG_TYPE_ARTICLE: &'static str = "article";
const OG_TYPE_WEBSITE: &'static str = "website";

pub struct Renderer {
    posts_in_dir: PathBuf,
    drafts_in_dir: PathBuf,
    out_dir: PathBuf,
    posts_out_dir: PathBuf,
    drafts_out_dir: PathBuf,
    markdowner: Markdowner,
    css_creator: CSSCreator,
    feed_creator: FeedCreator,
    metadata: Metadata,
    year: String,
    analytics_tag: String,
}

impl Renderer {
    pub fn new<P>(
        in_dir: P,
        out_dir: P,
        markdowner: Markdowner,
        css_creator: CSSCreator,
        feed_creator: FeedCreator,
        metadata: Metadata,
        year: String,
        analytics_tag: String,
    ) -> Renderer
    where
        P: AsRef<Path>,
    {
        let posts_in_dir = in_dir.as_ref().join("posts");
        let drafts_in_dir = in_dir.as_ref().join("drafts");
        let posts_out_dir = out_dir.as_ref().join("posts");
        let drafts_out_dir = out_dir.as_ref().join("drafts");
        let out_dir = out_dir.as_ref().to_path_buf();
        Renderer {
            posts_in_dir,
            drafts_in_dir,
            out_dir,
            posts_out_dir,
            drafts_out_dir,
            markdowner,
            css_creator,
            feed_creator,
            metadata,
            year,
            analytics_tag,
        }
    }

    pub fn render(&self) -> Result<()> {
        let posts = self.render_posts()?;
        let drafts = self.render_drafts()?;
        self.reset_out_dir()?;
        self.output_posts(&posts)?;
        self.output_drafts(&drafts)?;
        self.output_index(&posts)?;
        self.output_feed(&posts)?;
        self.output_css()?;
        Ok(())
    }

    fn reset_out_dir(&self) -> Result<()> {
        if self.out_dir.exists() {
            fs::remove_dir_all(&self.out_dir)?;
        }
        fs::create_dir(&self.out_dir)?;
        Ok(())
    }

    fn render_posts(&self) -> Result<Vec<RenderedPost>> {
        if !self.posts_in_dir.exists() {
            bail!(missing_posts_dir(&self.posts_in_dir))
        }

        let mut posts: Vec<RenderedPost> = self
            .posts_in_dir
            .read_dir()?
            .map(|entry| RawPost::new(&entry?.path()))
            .map(|raw_post| self.render_post(raw_post?))
            .collect::<Result<Vec<RenderedPost>>>()?;
        posts.sort_by(Self::order_posts);
        Ok(posts)
    }

    fn order_posts(p1: &RenderedPost, p2: &RenderedPost) -> Ordering {
        match p2.date.cmp(&p1.date) {
            Ordering::Equal => p2.title.cmp(&p1.title),
            order => order,
        }
    }

    fn render_post(&self, post: RawPost) -> Result<RenderedPost> {
        let html = self.markdowner.to_html(&post.markdown)?;
        let summary = extract_summary(&html)
            .with_context(|| format!("Failed to extract summary for \"{}\"", post.title))?;
        Ok(RenderedPost {
            id: post.id,
            title: post.title,
            date: post.date,
            summary,
            html,
        })
    }

    fn output_posts(&self, posts: &[RenderedPost]) -> Result<()> {
        fs::create_dir(&self.posts_out_dir)?;

        for post in posts {
            self.output_post(&post)?;
        }

        Ok(())
    }

    fn output_post(&self, post: &RenderedPost) -> Result<()> {
        let formatted_date = format!("{}", &post.date.format("%Y-%m-%d"));
        let full_html = templates::PostTemplate {
            title: &post.title,
            date: &formatted_date,
            content: &post.html,
        }
        .render()?;
        let post_dir = self.posts_out_dir.join(&post.id);
        fs::create_dir(&post_dir)?;
        self.render_page(
            &post_dir.join("index.html"),
            &post.title,
            &full_html,
            &self.to_og_url(&format!("posts/{}", post.id))?,
            &post.summary,
            OG_TYPE_ARTICLE,
        )
    }

    fn render_drafts(&self) -> Result<Vec<RenderedDraft>> {
        if !self.drafts_in_dir.exists() {
            return Ok(vec![]);
        }

        self.drafts_in_dir
            .read_dir()?
            .map(|entry| RawDraft::new(&entry?.path()))
            .map(|raw_post| self.render_draft(raw_post?))
            .collect::<Result<Vec<RenderedDraft>>>()
    }

    fn render_draft(&self, draft: RawDraft) -> Result<RenderedDraft> {
        let html = self.markdowner.to_html(&draft.markdown)?;
        let summary = format!("{} draft post", &draft.title);
        Ok(RenderedDraft {
            id: draft.id,
            title: draft.title,
            summary,
            html,
        })
    }

    fn output_drafts(&self, drafts: &[RenderedDraft]) -> Result<()> {
        fs::create_dir(&self.drafts_out_dir)?;

        for draft in drafts {
            self.output_draft(&draft)?;
        }
        Ok(())
    }

    fn output_draft(&self, draft: &RenderedDraft) -> Result<()> {
        let full_html = templates::DraftTemplate {
            title: &draft.title,
            content: &draft.html,
        }
        .render()?;
        let draft_dir = self.drafts_out_dir.join(&draft.id);
        fs::create_dir(&draft_dir)?;
        self.render_page(
            &draft_dir.join("index.html"),
            &draft.title,
            &full_html,
            &format!("drafts/{}", draft.id),
            &format!("{} draft post", draft.title),
            OG_TYPE_ARTICLE,
        )
    }

    fn output_index(&self, posts: &[RenderedPost]) -> Result<()> {
        let index = templates::IndexTemplate {
            blog_name: &self.metadata.blog_name,
            blog_subtitle: &self.metadata.blog_subtitle,
            posts,
        }
        .render()?;
        self.render_page(
            &self.out_dir.join("index.html"),
            &self.metadata.blog_name,
            &index,
            &format!("https://{}/", &self.metadata.domain),
            &self.metadata.blog_subtitle,
            OG_TYPE_WEBSITE,
        )
    }

    fn render_page(
        &self,
        path: &Path,
        title: &str,
        body: &str,
        url: &str,
        og_description: &str,
        og_type: &str,
    ) -> Result<()> {
        let mut file = File::create(path)?;
        templates::PageTemplate {
            title,
            og_type,
            url,
            blog_name: &self.metadata.blog_name,
            og_description,
            feed_file: feed::FEED_FILE,
            style: css::STYLE_FILE,
            body,
            links: &self.metadata.links,
            year: &self.year,
            author: &self.metadata.author,
            analytics_tag: &self.analytics_tag,
        }
        .write_into(&mut file)?;
        Ok(())
    }

    fn output_css(&self) -> Result<()> {
        self.css_creator.write_styles()
    }

    fn output_feed(&self, posts: &[RenderedPost]) -> Result<()> {
        self.feed_creator.render_feed(posts)
    }

    fn to_og_url(&self, path: &str) -> Result<String> {
        Ok(format!("https://{}/{}", self.metadata.domain, path))
    }
}

fn extract_summary(summary_html: &str) -> Result<String> {
    let raw_text = match summary_html.split_once("</p>") {
        Some((first_p, _)) => Ok(strip_html(first_p)),
        None => Err(missing_summary()),
    }?;
    let formatted = raw_text.replace("\n", " ");
    Ok(formatted)
}

fn strip_html(html: &str) -> String {
    let mut result = String::from("");
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            next if !in_tag => result.push(next),
            _ => continue,
        }
    }
    result
}

fn missing_posts_dir(path: &Path) -> Error {
    anyhow!(
        "\"posts\" directory not found. expected at {}",
        path.display()
    )
}

fn missing_summary() -> Error {
    anyhow!("no summary found")
}
