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

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, value_name = "DIR", default_value = "content")]
    in_dir: PathBuf,

    #[arg(short, long, value_name = "DIR", default_value = "gen")]
    out_dir: PathBuf,

    #[arg(short, long, value_name = "FILE", default_value = "config.toml")]
    config: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let now = Utc::now();
    let year = format!("{}", now.year());
    let syntax_set = SyntaxSet::load_defaults_newlines();
    let md = Markdowner::new(syntax_set);
    let theme_set = ThemeSet::load_defaults();
    let css_creator = CSSCreator::new(theme_set, &args.out_dir);
    let config = Config::from_toml(args.config)?;
    let feed_creator = FeedCreator::new(
        &args.out_dir,
        now,
        config.domain.clone(),
        config.blog_name.clone(),
    );
    let renderer = Renderer::new(
        args.in_dir,
        args.out_dir,
        md,
        css_creator,
        feed_creator,
        config.domain.clone(),
        config.blog_name.clone(),
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
