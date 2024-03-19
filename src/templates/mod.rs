use askama::Template;

use crate::{config::Themes, document::RenderedPost};

#[derive(Template)]
#[template(path = "post.html")]
pub struct PostTemplate<'a> {
    pub title: &'a str,
    pub date: &'a str,
    pub content: &'a str,
}

#[derive(Template)]
#[template(path = "draft.html")]
pub struct DraftTemplate<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub blog_name: &'a str,
    pub blog_subtitle: &'a str,
    pub posts: &'a [RenderedPost],
}

#[derive(Template)]
#[template(path = "base.html")]
pub struct PageTemplate<'a> {
    pub title: &'a str,
    pub og_type: &'a str,
    pub url: &'a str,
    pub blog_name: &'a str,
    pub og_description: &'a str,
    pub feed_file: &'a str,
    pub style: &'a str,
    pub body: &'a str,
    pub github_url: &'a str,
    pub year: &'a str,
    pub author: &'a str,
    pub analytics_tag: &'a str,
}

#[derive(Template)]
#[template(path = "style.css", escape = "none")]
pub struct StyleTemplate<'a> {
    pub themes: &'a Themes,
}
