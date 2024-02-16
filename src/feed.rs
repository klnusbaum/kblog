use crate::document::Post;
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
    domain: String,
    blog_name: String,
}

impl FeedCreator {
    pub fn new<P, D>(out_dir: P, now: D, domain: String, blog_name: String) -> FeedCreator
    where
        P: AsRef<Path>,
        D: Into<FixedDateTime>,
    {
        let feed_file = out_dir.as_ref().join(FEED_FILE);
        let now = now.into();
        FeedCreator {
            feed_file,
            now,
            domain,
            blog_name,
        }
    }

    pub fn render_feed(&self, posts: &[Post]) -> Result<()> {
        let feed = self.create_feed(&posts);
        self.write_feed(&feed)
    }

    fn create_feed(&self, posts: &[Post]) -> Feed {
        let entries = self.create_entries(&posts);
        let latest_update = entries.latest_update(self.now);

        FeedBuilder::default()
            .id(format!("tag:{}", self.domain))
            .title(self.feed_title())
            .subtitle(self.feed_subtitle())
            .updated(latest_update)
            .links(self.feed_links())
            .author(self.kurtis())
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
        plain_text(&self.blog_name)
    }

    fn feed_subtitle(&self) -> Text {
        plain_text("The Blog of Kurtis Nusbaum")
    }

    fn feed_links(&self) -> Vec<Link> {
        let self_link = LinkBuilder::default()
            .href(format!("https://{}/{}", &self.domain, FEED_FILE))
            .rel("self")
            .mime_type("application/atom+xml".to_string())
            .build();
        let alt_link = LinkBuilder::default()
            .href(format!("https://{}/", self.domain))
            .rel("alternate")
            .mime_type("text/html".to_string())
            .build();
        vec![self_link, alt_link]
    }

    fn create_entries(&self, posts: &[Post]) -> Vec<Entry> {
        posts.iter().map(|p| self.create_entry(p)).collect()
    }

    fn create_entry(&self, post: &Post) -> Entry {
        EntryBuilder::default()
            .id(self.entry_id(post))
            .title(plain_text(&post.title))
            .published(post.date)
            .updated(post.date)
            .link(self.entry_link(post))
            .author(self.kurtis())
            .build()
    }

    fn entry_id(&self, post: &Post) -> String {
        format!(
            "tag:{},{}:{}",
            self.domain,
            post.date.format("%Y-%m-%d"),
            post.id
        )
    }

    fn entry_link(&self, post: &Post) -> Link {
        LinkBuilder::default()
            .href(format!("https://{}/posts/{}", self.domain, post.id))
            .rel("alternate")
            .mime_type("text/html".to_string())
            .build()
    }

    fn rights(&self) -> Text {
        plain_text(&format!("© {} Kurtis Nusbaum", self.now.year()))
    }

    fn kurtis(&self) -> Person {
        PersonBuilder::default()
            .name("Kurtis Nusbaum")
            .uri(format!("https://{}/", self.domain))
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
