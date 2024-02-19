mod config;
mod css;
mod document;
mod feed;
mod markdown;
mod render;
mod templates;

use crate::css::CSSCreator;
use crate::feed::FeedCreator;
use crate::markdown::Markdowner;
use crate::render::Renderer;
use anyhow::Result;
use chrono::{Datelike, Utc};
use clap::Parser;
use config::Config;
use std::env;
use std::path::PathBuf;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

const DEFAULT_IN_DIR: &'static str = "content";
const DEFAULT_OUT_DIR: &'static str = "gen";
const CONFIG_FILE_NAME: &'static str = "config.toml";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Optional directory from which the program should run.
    #[arg(short = 'C', value_name = "DIR")]
    working_directory: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    if let Some(dir) = args.working_directory {
        env::set_current_dir(dir)?
    };

    let now = Utc::now();
    let year = format!("{}", now.year());
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let md = Markdowner::new(syntax_set);
    let theme_set = ThemeSet::load_defaults();
    let config = Config::from_toml(PathBuf::from(CONFIG_FILE_NAME))?;
    let css_creator = CSSCreator::new(DEFAULT_OUT_DIR, config.themes.clone(), theme_set);
    let feed_creator = FeedCreator::new(
        DEFAULT_OUT_DIR,
        now,
        config.domain.clone(),
        config.blog_name.clone(),
        config.blog_subtitle.clone(),
        config.author.clone(),
    );
    let renderer = Renderer::new(
        PathBuf::from(DEFAULT_IN_DIR),
        PathBuf::from(DEFAULT_OUT_DIR),
        md,
        css_creator,
        feed_creator,
        config.domain.clone(),
        config.blog_name.clone(),
        config.blog_subtitle.clone(),
        config.author,
        year,
        env_or_default("ANALYTICS_TAG", "dev_tag"),
    );

    renderer.render().map_err(|e| {
        eprintln!("{}", e);
        e
    })
}

fn env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(String::from(default))
}
