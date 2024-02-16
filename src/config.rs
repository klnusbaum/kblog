use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub domain: String,
    pub blog_name: String,
    pub blog_subtitle: String,
    pub author: String,
}

impl Config {
    pub fn from_toml(file: PathBuf) -> Result<Config> {
        let content = fs::read_to_string(file)?;
        Ok(toml::from_str(&content)?)
    }
}
