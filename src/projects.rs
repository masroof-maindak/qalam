use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

const IN_PROJS_FPATH: &str = "projects.toml";
const OUT_PROJ_FPATH: &str = "build/projects.html";

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    desc: String,
    url: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectPage {
    title: String,
    desc: String,
    projects: Vec<Project>,
}

pub fn parse_projs_index_file() -> Result<ProjectPage> {
    let projs_page_toml = fs::read_to_string(IN_PROJS_FPATH)
        .with_context(|| format!("[PROJ] Failed to read {IN_PROJS_FPATH}."))?;

    let projs_page_cfg: ProjectPage =
        toml::from_str(&projs_page_toml).context("[PROJ] Failed to parse projects file.")?;

    Ok(projs_page_cfg)
}

pub fn generate_projs_page_html(_p: &ProjectPage) -> Result<String> {
    todo!()
}

pub fn write_projs_html_page(projs_page_html: String) -> Result<()> {
    fs::write(OUT_PROJ_FPATH, projs_page_html)
        .with_context(|| format!("[PROJ] Failed to write HTML to {OUT_PROJ_FPATH}"))
}
