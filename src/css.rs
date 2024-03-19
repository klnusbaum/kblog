use anyhow::Result;
use askama::Template;
use std::fs;
use std::path::{Path, PathBuf};
use syntect::highlighting::ThemeSet;
use syntect::html::{css_for_theme_with_class_style, ClassStyle};

use crate::config::Themes;
use crate::templates;

pub const STYLE_FILE: &'static str = "style.css";

pub struct CSSCreator {
    out_dir: PathBuf,
    main_themes: Themes,
    code_theme_set: ThemeSet,
}

impl CSSCreator {
    pub fn new<P: AsRef<Path>>(
        out_dir: P,
        main_themes: Themes,
        code_theme_set: ThemeSet,
    ) -> CSSCreator {
        let out_dir = out_dir.as_ref().to_path_buf();
        CSSCreator {
            out_dir,
            main_themes,
            code_theme_set,
        }
    }

    pub fn write_styles(&self) -> Result<()> {
        self.write_dark_code_style()?;
        self.write_light_code_style()?;
        self.write_main_style()?;
        Ok(())
    }

    fn write_main_style(&self) -> Result<()> {
        let css = templates::StyleTemplate {
            themes: &self.main_themes,
        }
        .render()?;
        self.write_css(STYLE_FILE, &css)
    }

    fn write_light_code_style(&self) -> Result<()> {
        let css = self.load_theme_css("Solarized (light)")?;
        self.write_css("code-theme-light.css", &css)
    }

    fn write_dark_code_style(&self) -> Result<()> {
        let css = self.load_theme_css("Solarized (dark)")?;
        self.write_css("code-theme-dark.css", &css)
    }

    fn load_theme_css(&self, name: &str) -> Result<String> {
        let theme = &self.code_theme_set.themes[name];
        let css = css_for_theme_with_class_style(theme, ClassStyle::Spaced)?;
        Ok(css)
    }

    fn write_css(&self, style_file: &str, css: &str) -> Result<()> {
        let css_file = self.out_dir.join(style_file);
        Ok(fs::write(css_file, css)?)
    }
}
