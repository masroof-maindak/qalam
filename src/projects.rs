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

pub fn create_html_str(pp: &ProjectPage, footer_text: &Option<String>) -> String {
    let markup = html! {
        (utils::page_header(&pp.page_title, &".."))

        div #projects-page {
            (utils::goto_home_link())

            h1 {(pp.title)}
            p {(pp.desc)}

            @for proj in &pp.projects {
                article .project-item {
                    div .project-line {
                        a .project-name href=(proj.url) target="_blank" rel="noreferrer noopener" {(proj.name)}
                        span .project-desc {(proj.desc)}
                    }
                    p .project-tags {
                        @for tag in &proj.tags {
                            span { "#" (tag) }
                        }
                    }
                }
            }
        }

        (utils::page_footer(footer_text))
    };

    markup.into_string()
}
