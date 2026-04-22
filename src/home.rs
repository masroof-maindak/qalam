use maud::html;
use serde::{Deserialize, Serialize};

use crate::utils;

pub const IN_HOME_CFG_PATH: &str = "index.toml";
pub const OUT_HOME_CFG_PATH: &str = "build/index.html";
pub const AVATAR_PATH: &str = "img/avatar.webp";

#[derive(Serialize, Deserialize, Debug)]
pub struct HomePage {
    page_title: String,
    name: String,
    username: String,
    bio: String,
    email: String,
    github: String,
    desc: String,
}

pub fn create_html_str(hp: &HomePage) -> String {
    let markup = html! {
        (utils::page_header(&hp.page_title))

        section #home-info {
            img src=(AVATAR_PATH) alt={"Avatar"};
            div {
                span #name {(hp.name)}
                span #username {(hp.username)}
            }
            div #bio {(hp.bio)}
        }

        section #home-links {
            div #left {
                a .home-link href="/posts/index.html" {"posts"};
                a .home-link href="/projects/index.html" {"projects"};
            }
            div #right {
                // mail svg & link
                // a href="mailto:"{(hp.email)}
                // GH svg & link
            }
        }

        section #home-brief {
            p {(hp.desc)}
        }
    };

    markup.into_string()
}
