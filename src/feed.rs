use crate::config::Metadata;
use crate::document::RenderedPost;
use anyhow::Result;
use atom_syndication::{
    Entry, EntryBuilder, Feed, FeedBuilder, FixedDateTime, Link, LinkBuilder, Person,
    PersonBuilder, Text,
};
use chrono::Datelike;
use std::fs::File;
use std::path::{Path, PathBuf};

pub const FEED_FILE: &'static str = "atom.xml";

pub struct FeedCreator {
    feed_file: PathBuf,
    now: FixedDateTime,
    metadata: Metadata,
}

impl FeedCreator {
    pub fn new<P, D>(out_dir: P, now: D, metadata: Metadata) -> FeedCreator
    where
        P: AsRef<Path>,
        D: Into<FixedDateTime>,
    {
        let feed_file = out_dir.as_ref().join(FEED_FILE);
        let now = now.into();
        FeedCreator {
            feed_file,
            now,
            metadata,
        }
    }

    pub fn render_feed(&self, posts: &[RenderedPost]) -> Result<()> {
        let feed = self.create_feed(&posts);
        self.write_feed(&feed)
    }

    fn create_feed(&self, posts: &[RenderedPost]) -> Feed {
        let entries = self.create_entries(&posts);
        let latest_update = entries.latest_update(self.now);

        FeedBuilder::default()
            .id(format!("tag:{}", self.metadata.domain))
            .title(self.feed_title())
            .subtitle(self.feed_subtitle())
            .updated(latest_update)
            .links(self.feed_links())
            .author(self.author())
            .rights(self.rights())
            .entries(entries)
            .build()
    }

    fn write_feed(&self, feed: &Feed) -> Result<()> {
        let file = File::create(&self.feed_file)?;
        feed.write_to(file)?;
        Ok(())
    }

    fn feed_title(&self) -> Text {
        plain_text(&self.metadata.blog_name)
    }

    fn feed_subtitle(&self) -> Text {
        plain_text(&self.metadata.blog_subtitle)
    }

    fn feed_links(&self) -> Vec<Link> {
        let self_link = LinkBuilder::default()
            .href(format!("https://{}/{}", &self.metadata.domain, FEED_FILE))
            .rel("self")
            .mime_type("application/atom+xml".to_string())
            .build();
        let alt_link = LinkBuilder::default()
            .href(format!("https://{}/", self.metadata.domain))
            .rel("alternate")
            .mime_type("text/html".to_string())
            .build();
        vec![self_link, alt_link]
    }

    fn create_entries(&self, posts: &[RenderedPost]) -> Vec<Entry> {
        posts.iter().map(|p| self.create_entry(p)).collect()
    }

    fn create_entry(&self, post: &RenderedPost) -> Entry {
        EntryBuilder::default()
            .id(self.entry_id(post))
            .title(plain_text(&post.title))
            .summary(plain_text(&post.summary))
            .published(post.date)
            .updated(post.date)
            .link(self.entry_link(post))
            .author(self.author())
            .build()
    }

    fn entry_id(&self, post: &RenderedPost) -> String {
        format!(
            "tag:{},{}:{}",
            self.metadata.domain,
            post.date.format("%Y-%m-%d"),
            post.id
        )
    }

    fn entry_link(&self, post: &RenderedPost) -> Link {
        LinkBuilder::default()
            .href(format!(
                "https://{}/posts/{}",
                self.metadata.domain, post.id
            ))
            .rel("alternate")
            .mime_type("text/html".to_string())
            .build()
    }

    fn rights(&self) -> Text {
        plain_text(&format!("© {} {}", self.now.year(), self.metadata.author))
    }

    fn author(&self) -> Person {
        PersonBuilder::default()
            .name(&self.metadata.author)
            .uri(format!("https://{}/", self.metadata.domain))
            .build()
    }
}

fn plain_text(text: &str) -> Text {
    Text::plain(text)
}

trait Chronological {
    fn latest_update(&self, now: FixedDateTime) -> FixedDateTime;
}

impl Chronological for Vec<Entry> {
    fn latest_update(&self, now: FixedDateTime) -> FixedDateTime {
        match self.iter().map(|e| e.updated).max() {
            Some(max) => max,
            None => now,
        }
    }
}
