use maud::html;
use serde::Deserialize;

use crate::utils;

pub const IN_PROJS_CFG_PATH: &str = "projects.toml";
pub const OUT_PROJ_PATH: &str = "build/projects/index.html";
pub const OUT_PROJ_DIR: &str = "build/projects/";

#[derive(Deserialize, Debug)]
struct Project {
    name: String,
    desc: String,
    url: String,
    tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectPage {
    page_title: String,
    title: String,
    desc: String,
    projects: Vec<Project>,
}

pub fn create_html_str(pp: &ProjectPage) -> String {
    // TODO: Assign CSS classes!

    let markup = html! {
        (utils::page_header(&pp.page_title))

        h1 {(pp.title)}
        p {(pp.desc)}

        @for proj in &pp.projects {
            h3 {(proj.name) ", -- " (proj.url)}
            p {(proj.desc)}
            p {
                @for tag in &proj.tags {
                    {"#" (tag) " "}
                }
            }
        }
    };

    markup.into_string()
}
