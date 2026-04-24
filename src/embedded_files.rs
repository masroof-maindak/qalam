use anyhow::{Context, Result, bail};
use camino::Utf8Path;
use rust_embed::Embed;
use std::fs::{self, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::str;
use syntect::highlighting::{Theme, ThemeSet};

#[derive(Embed)]
#[folder = "themes/"]
struct ThemesDir;

#[derive(Embed)]
#[folder = "syntax-themes/"]
struct SyntaxThemesDir;

const OUT_CSS_PATH: &str = "build/themes/styles.css";
const OVERRIDE_CSS_PATH: &str = "themes/override.css";
const THEMES_PATH: &str = "syntax-themes/";

pub fn generate_css_with_override() -> Result<()> {
    let styles_css =
        ThemesDir::get("styles.css").expect("Failed to find styles.css file in ThemesDir");
    let styles_css_text = str::from_utf8(&styles_css.data)?;

    let mut stream = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(OUT_CSS_PATH)?,
    );

    stream.write_all(styles_css_text.as_bytes())?;

    let override_css_path = Path::new(OVERRIDE_CSS_PATH);
    if override_css_path.exists() {
        let override_css = fs::read_to_string(override_css_path)?;
        stream.write_all(override_css.as_bytes())?;
    }

    stream.flush()?;

    Ok(())
}

pub fn generate_theme_set() -> Result<ThemeSet> {
    let rel_theme_path = Utf8Path::new(THEMES_PATH);
    let mut theme_set = ThemeSet::new();

    // Load 'official' themes
    for path in SyntaxThemesDir::iter() {
        let path = Utf8Path::new(path.as_ref());

        let theme_file = SyntaxThemesDir::get(path.as_str());
        match theme_file {
            None => {
                bail!("Failed to load syntax theme from {path}");
            }

            Some(ef) => {
                let mut theme_cursor = std::io::Cursor::new(str::from_utf8(&ef.data)?);
                let theme = ThemeSet::load_from_reader(&mut theme_cursor)?;
                if let Some(name) = path.file_stem() {
                    theme_set.themes.insert(name.to_string(), theme);
                } else {
                    bail!("Failed to extract fname from path: {path}");
                }
            }
        }
    }

    // Load user themes, if the folder exists
    if Path::new(rel_theme_path).exists() {
        theme_set
            .add_from_folder(rel_theme_path)
            .with_context(|| format!("Failed to load themes from {rel_theme_path}"))?;
    }

    Ok(theme_set)
}

pub fn load_syntax_theme(theme_set: ThemeSet, theme_name: &str) -> Result<Theme> {
    if let Some(theme) = theme_set.themes.get(theme_name) {
        Ok(theme.clone())
    } else {
        bail!("Theme {theme_name} not found.");
    }
}
