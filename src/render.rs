use crate::css::CSSCreator;
use crate::document::{Draft, Post};
use crate::feed::FeedCreator;
use crate::markdown::Markdowner;
use crate::{css, feed, templates};
use anyhow::{anyhow, bail, Error, Result};
use std::cmp::Ordering;
use std::fs;
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
    year: String,
    analytics_tag: String,
}

impl Renderer {
    pub fn new(
        in_dir: PathBuf,
        out_dir: PathBuf,
        markdowner: Markdowner,
        css_creator: CSSCreator,
        feed_creator: FeedCreator,
        year: String,
        analytics_tag: String,
    ) -> Renderer {
        let posts_in_dir = in_dir.join("posts");
        let drafts_in_dir = in_dir.join("drafts");
        let posts_out_dir = out_dir.join("posts");
        let drafts_out_dir = out_dir.join("drafts");
        Renderer {
            posts_in_dir,
            drafts_in_dir,
            out_dir,
            posts_out_dir,
            drafts_out_dir,
            markdowner,
            css_creator,
            feed_creator,
            year,
            analytics_tag,
        }
    }

    pub fn render(&self) -> Result<()> {
        self.reset_out_dir()?;
        let posts = self.create_posts()?;
        let drafts = self.create_drafts()?;
        for post in &posts {
            self.render_post(&post)?;
        }
        for draft in &drafts {
            self.render_draft(&draft)?;
        }
        self.render_index(&posts)?;
        self.render_feed(&posts)?;
        self.render_css()?;
        Ok(())
    }

    fn reset_out_dir(&self) -> Result<()> {
        if self.out_dir.exists() {
            fs::remove_dir_all(&self.out_dir)?;
        }
        fs::create_dir(&self.out_dir)?;
        Ok(())
    }

    fn create_posts(&self) -> Result<Vec<Post>> {
        if !self.posts_in_dir.exists() {
            bail!(missing_posts_dir(&self.posts_in_dir))
        }

        fs::create_dir(&self.posts_out_dir)?;

        let mut posts: Vec<Post> = self
            .posts_in_dir
            .read_dir()?
            .map(|entry| Post::new(&entry?.path()))
            .collect::<Result<Vec<Post>>>()?;
        posts.sort_by(Self::order_posts);
        Ok(posts)
    }

    fn order_posts(p1: &Post, p2: &Post) -> Ordering {
        match p2.date.cmp(&p1.date) {
            Ordering::Equal => p2.title.cmp(&p1.title),
            order => order,
        }
    }

    fn create_drafts(&self) -> Result<Vec<Draft>> {
        if !self.drafts_in_dir.exists() {
            return Ok(vec![]);
        }

        fs::create_dir(&self.drafts_out_dir)?;

        self.drafts_in_dir
            .read_dir()?
            .map(|entry| Draft::new(&entry?.path()))
            .collect::<Result<Vec<Draft>>>()
    }

    fn render_post(&self, post: &Post) -> Result<()> {
        let html = self.markdowner.to_html(&post.markdown)?;
        let formatted_date = format!("{}", &post.date.format("%Y-%m-%d"));
        let rendered_post = templates::POST_TEMPLATE
            .replace(templates::TOKEN_TITLE, &post.title)
            .replace(templates::TOKEN_DATE, &formatted_date)
            .replace(templates::TOKEN_CONTENT, &html);
        let post_dir = self.posts_out_dir.join(&post.id);
        fs::create_dir(&post_dir)?;
        self.render_page(
            &post_dir.join("index.html"),
            &post.title,
            &rendered_post,
            &post_dir.to_og_url()?,
            OG_TYPE_ARTICLE,
        )
    }

    fn render_draft(&self, draft: &Draft) -> Result<()> {
        let html = self.markdowner.to_html(&draft.markdown)?;
        let rendered_draft = templates::DRAFT_TEMPLATE
            .replace(templates::TOKEN_TITLE, &draft.title)
            .replace(templates::TOKEN_CONTENT, &html);
        let draft_dir = self.drafts_out_dir.join(&draft.id);
        fs::create_dir(&draft_dir)?;
        self.render_page(
            &draft_dir.join("index.html"),
            &draft.title,
            &rendered_draft,
            &draft_dir.to_og_url()?,
            OG_TYPE_ARTICLE,
        )
    }

    fn render_index(&self, posts: &[Post]) -> Result<()> {
        let list: String = posts
            .iter()
            .map(|post| {
                format!(
                    "<li>{} <a href=\"/posts/{}\">{}</a></li>\n",
                    post.date.format("%Y-%m-%d"),
                    post.id,
                    post.title
                )
            })
            .collect();
        let index = templates::INDEX_TEMPLATE.replace(templates::TOKEN_POST_LIST, &list);
        self.render_page(
            &self.out_dir.join("index.html"),
            "knusbaum.org",
            &index,
            "https://knusbaum.org/",
            OG_TYPE_WEBSITE,
        )
    }

    fn render_page(
        &self,
        path: &Path,
        title: &str,
        body: &str,
        url: &str,
        og_type: &str,
    ) -> Result<()> {
        let rendered_page = templates::PAGE_TEMPLATE
            .replace(templates::TOKEN_STYLES, css::STYLE_FILE)
            .replace(templates::TOKEN_RSS_FEED, feed::FEED_FILE)
            .replace(templates::TOKEN_TITLE, title)
            .replace(templates::TOKEN_BODY, body)
            .replace(templates::TOKEN_YEAR, &self.year)
            .replace(templates::TOKEN_ANALYTICS_TAG, &self.analytics_tag)
            .replace(templates::TOKEN_URL, url)
            .replace(templates::TOKEN_OG_TYPE, og_type.as_ref());
        Ok(fs::write(path, &rendered_page)?)
    }

    fn render_css(&self) -> Result<()> {
        self.css_creator.write_styles()
    }

    fn render_feed(&self, posts: &[Post]) -> Result<()> {
        self.feed_creator.render_feed(posts)
    }
}

fn missing_posts_dir(path: &Path) -> Error {
    anyhow!(
        "\"posts\" directory not found. expected at {}",
        path.display()
    )
}

trait ToOgUrl {
    fn to_og_url(&self) -> Result<String>;
}

impl ToOgUrl for PathBuf {
    fn to_og_url(&self) -> Result<String> {
        let url_path = self
            .to_str()
            .ok_or(anyhow!("Non utf8 file name {}", self.display()))?;
        Ok(format!("https://knusbaum.org/{}", url_path))
    }
}
