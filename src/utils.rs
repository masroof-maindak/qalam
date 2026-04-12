use anyhow::{Context, Result, bail};
use std::fs;

use crate::{home, posts, projects};

pub fn write_html(html: String, out_path: &str) -> Result<()> {
    // TODO: sanitize HTML via ammonia?

    fs::write(out_path, html).with_context(|| format!("Failed to write HTML to {out_path}"))
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
