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
}

impl FeedCreator {
    pub fn new<P, D>(out_dir: P, now: D) -> FeedCreator
    where
        P: AsRef<Path>,
        D: Into<FixedDateTime>,
    {
        let feed_file = out_dir.as_ref().join(FEED_FILE);
        let now = now.into();
        FeedCreator { feed_file, now }
    }

    pub fn render_feed(&self, posts: &[Post]) -> Result<()> {
        let feed = self.create_feed(&posts);
        self.write_feed(&feed)
    }

    fn create_feed(&self, posts: &[Post]) -> Feed {
        let entries = create_entries(&posts);
        let latest_update = entries.latest_update(self.now);
        let rights = rights(&self.now);

        FeedBuilder::default()
            .id("tag:knusbaum.org")
            .title(feed_title())
            .subtitle(feed_subtitle())
            .updated(latest_update)
            .links(feed_links())
            .author(kurtis())
            .rights(rights)
            .entries(entries)
            .build()
    }

    fn write_feed(&self, feed: &Feed) -> Result<()> {
        let file = File::create(&self.feed_file)?;
        feed.write_to(file)?;
        Ok(())
    }
}

fn feed_title() -> Text {
    plain_text("knusbaum.org")
}

fn feed_subtitle() -> Text {
    plain_text("The Blog of Kurtis Nusbaum")
}

fn feed_links() -> Vec<Link> {
    let self_link = LinkBuilder::default()
        .href(format!("https://knusbaum.org/{}", FEED_FILE))
        .rel("self")
        .mime_type("application/atom+xml".to_string())
        .build();
    let alt_link = LinkBuilder::default()
        .href("https://knusbaum.org/")
        .rel("alternate")
        .mime_type("text/html".to_string())
        .build();
    vec![self_link, alt_link]
}

fn create_entries(posts: &[Post]) -> Vec<Entry> {
    posts.iter().map(|p| create_entry(p)).collect()
}

fn create_entry(post: &Post) -> Entry {
    EntryBuilder::default()
        .id(entry_id(post))
        .title(plain_text(&post.title))
        .published(post.date)
        .updated(post.date)
        .link(entry_link(post))
        .author(kurtis())
        .build()
}

fn entry_id(post: &Post) -> String {
    format!(
        "tag:knusbaum.org,{}:{}",
        post.date.format("%Y-%m-%d"),
        post.id
    )
}

fn entry_link(post: &Post) -> Link {
    LinkBuilder::default()
        .href(format!("https://knusbaum.org/posts/{}", post.id))
        .rel("alternate")
        .mime_type("text/html".to_string())
        .build()
}

fn rights(now: &FixedDateTime) -> Text {
    plain_text(&format!("© {} Kurtis Nusbaum", now.year()))
}

fn kurtis() -> Person {
    PersonBuilder::default()
        .name("Kurtis Nusbaum")
        .uri("https://knusbaum.org".to_string())
        .build()
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
