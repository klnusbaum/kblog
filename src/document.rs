use anyhow::{anyhow, Error, Result};
use chrono::prelude::*;
use std::fs;
use std::path::Path;

pub struct RenderedPost {
    pub id: String,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub summary: String,
    pub html: String,
}

pub struct RawPost {
    pub id: String,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub markdown: String,
}

impl RawPost {
    pub fn new(path: &Path) -> Result<RawPost> {
        let (id, date) = id_and_date(path)?;
        let (title, markdown) = title_and_markdown(path)?;

        Ok(RawPost {
            id,
            title,
            date,
            markdown,
        })
    }
}

pub struct RenderedDraft {
    pub id: String,
    pub title: String,
    pub html: String,
}

pub struct RawDraft {
    pub id: String,
    pub title: String,
    pub markdown: String,
}

impl RawDraft {
    pub fn new(path: &Path) -> Result<RawDraft> {
        let id = file_name(path)?.to_string();
        let (title, markdown) = title_and_markdown(path)?;
        Ok(RawDraft {
            id,
            title,
            markdown,
        })
    }
}

fn id_and_date(path: &Path) -> Result<(String, DateTime<FixedOffset>)> {
    let file_name = file_name(path)?;
    let mut parts = file_name.split("_");
    let date = parts.next().ok_or(missing_date(path))?;
    let id = parts.next().ok_or(missing_id(path))?;
    Ok((id.to_string(), parse_date(date)?))
}

fn parse_date(date: &str) -> Result<DateTime<FixedOffset>> {
    let parsed = date
        .parse::<NaiveDate>()?
        .and_time(NaiveTime::default())
        .and_local_timezone(Utc)
        .unwrap()
        .into();
    Ok(parsed)
}

fn title_and_markdown(path: &Path) -> Result<(String, String)> {
    let all_content = fs::read_to_string(path)?;
    let mut parts = all_content.splitn(2, "\n\n");
    let title = parts.next().ok_or(missing_title(path))?;
    let markdown = parts.next().ok_or(missing_markdown(path))?;
    Ok((title.to_string(), markdown.to_string()))
}

fn file_name<'a>(path: &'a Path) -> Result<&'a str> {
    Ok(path
        .file_name()
        .ok_or(missing_file_name(path))?
        .to_str()
        .ok_or(bad_file_name(path))?
        .trim_end_matches(".md"))
}

fn missing_file_name(path: &Path) -> Error {
    anyhow!("couldn't parse filename of {}", path.display())
}

fn bad_file_name(path: &Path) -> Error {
    anyhow!("bad file name {}", path.display())
}

fn missing_date(path: &Path) -> Error {
    anyhow!("post missing date in filename {}", path.display())
}

fn missing_id(path: &Path) -> Error {
    anyhow!("post missing id in filename {}", path.display())
}

fn missing_title(path: &Path) -> Error {
    anyhow!("post missing title {}", path.display())
}

fn missing_markdown(path: &Path) -> Error {
    anyhow!("post missing markdown {}", path.display())
}
