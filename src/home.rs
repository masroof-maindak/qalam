use maud::{DOCTYPE, html};
use serde::{Deserialize, Serialize};

pub const IN_HOME_CFG_PATH: &str = "index.toml";
pub const OUT_HOME_CFG_PATH: &str = "build/index.html";

#[derive(Serialize, Deserialize, Debug)]
pub struct HomePage {
    page_title: String,
    name: String,
    username: String,
    bio: String,
    email: String,
    github: String,
    desc: String,
    num_recent_posts: u8,
    css_file_path: Option<String>,
}

pub fn create_html_str(hp: &HomePage) -> String {
    // TODO: Assign CSS classes!

    let markup = html! {
        (DOCTYPE)
        html {
            meta charset="utf-8";
            title {(hp.page_title)}
        }

        section #home-info {
            img {}
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

        section #home-post-list {
            // TODO: loop over hp.num_recent_posts posts

            a href="/posts/" {"more »"}
        }
    };

    markup.into_string()
}
