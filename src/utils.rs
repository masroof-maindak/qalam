use anyhow::{Context, Result, bail};
use camino::Utf8Path;
use maud::{DOCTYPE, Markup, html};
use std::fs;
use std::path::Path;

use crate::{home, posts, projects};

pub const CSS_PATH: &str = "themes/styles.css";
const PROJECT_URL: &str = "https://github.com/masroof-maindak/qalam";

pub fn write_html(html: &str, out_path: &dyn AsRef<Path>) -> Result<()> {
    fs::write(out_path, html)
        .with_context(|| format!("Failed to write HTML to {:#?}", out_path.as_ref()))?;

    Ok(())
}

#[derive(Debug)]
pub enum TomlFileType {
    Proj,
    Home,
    Posts,
}

#[derive(Debug)]
pub enum TomlCfg {
    Proj(projects::ProjectPage),
    Home(home::HomePage),
    Post(posts::PostsPage),
}

impl TomlCfg {
    pub fn into_proj(self) -> Result<projects::ProjectPage> {
        if let TomlCfg::Proj(p) = self {
            Ok(p)
        } else {
            bail!("expected Proj config")
        }
    }

    pub fn into_home(self) -> Result<home::HomePage> {
        if let TomlCfg::Home(h) = self {
            Ok(h)
        } else {
            bail!("expected Home config")
        }
    }

    pub fn into_post(self) -> Result<posts::PostsPage> {
        if let TomlCfg::Post(p) = self {
            Ok(p)
        } else {
            bail!("expected Post config")
        }
    }
}

pub fn parse_toml_file(tf: TomlFileType, path: &str) -> Result<TomlCfg> {
    let toml_str =
        fs::read_to_string(path).with_context(|| format!("[{:?}] Failed to read {path}.", tf))?;

    match tf {
        TomlFileType::Proj => Ok(TomlCfg::Proj(toml::from_str::<projects::ProjectPage>(
            &toml_str,
        )?)),
        TomlFileType::Home => Ok(TomlCfg::Home(toml::from_str::<home::HomePage>(&toml_str)?)),
        TomlFileType::Posts => Ok(TomlCfg::Post(toml::from_str::<posts::PostsPage>(
            &toml_str,
        )?)),
    }
}

pub fn page_header(page_title: &str, css_path_base_dir: &dyn AsRef<Path>) -> Markup {
    let css_path = css_path_base_dir.as_ref().join(CSS_PATH);

    html! {
        (DOCTYPE)
        html {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            title { (page_title) }
        }
        link rel="stylesheet" type="text/css" href=(css_path.display());

        // Favicon stuff
        link rel="icon" type="image/png" href="/favicons/favicon-96x96.png" sizes="96x96";
        link rel="icon" type="image/svg+xml" href="/favicons/favicon.svg";
        link rel="shortcut icon" href="/favicons/favicon.ico";
        link rel="apple-touch-icon" sizes="180x180" href="/favicons/apple-touch-icon.png";
        link rel="manifest" href="/favicons/site.webmanifest";
    }
}

pub fn goto_home_link() -> Markup {
    html! {
        a #back-text href="/" { "← Back" }
    }
}

pub fn goto_posts_link() -> Markup {
    html! {
        a #back-text href="/posts" { "← Back" }
    }
}

pub fn page_footer(copyright_str: &Option<String>) -> Markup {
    html! {
        footer {
            @if let Some(s) = copyright_str {
                span {(s) "  |  "}
            }

            span {"Built w/ " a .footer-link href=(PROJECT_URL) {"Qalam."}};

            // TODO: right-aligned theme colour switcher
        }
    }
}

pub fn copy_images_to_build(src_path: &str, dst_path: &dyn AsRef<Path>) -> Result<()> {
    for entry in Utf8Path::new(src_path).read_dir_utf8()? {
        let entry = entry?;
        let entry = entry.path();

        if let Some(fname) = entry.file_name()
            && entry.is_file()
        {
            let dst = dst_path.as_ref().join(fname);
            fs::copy(entry, &dst).with_context(|| format!("Moving {entry} to {:#?}", dst))?;
        }
    }

    Ok(())
}
