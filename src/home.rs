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
}

pub fn create_html_str(hp: &HomePage) -> String {
    // TODO: Assign CSS classes!

    let markup = html! {
        (DOCTYPE)
        html {
            meta charset="utf-8";
            title {(hp.page_title)}
        }

        h1 {(hp.name)}
        p {(hp.desc)}

    };

    markup.into_string()
}
