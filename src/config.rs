use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    pub metadata: Metadata,
    pub themes: Themes,
}

#[derive(Deserialize, Clone)]
pub struct Metadata {
    pub domain: String,
    pub blog_name: String,
    pub blog_subtitle: String,
    pub author: String,
}

#[derive(Deserialize, Clone)]
pub struct Themes {
    pub light: Theme,
    pub dark: Theme,
}

#[derive(Deserialize, Clone)]
pub struct Theme {
    pub background_color: String,
    pub text_color: String,
    pub link_color: String,
    pub footer_color: String,
}

impl Config {
    pub fn from_toml<P: AsRef<Path>>(file: P) -> Result<Config> {
        let content = fs::read_to_string(file)?;
        Ok(toml::from_str(&content)?)
    }
}
