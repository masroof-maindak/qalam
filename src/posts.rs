use anyhow::{Result, bail};
use maud::{DOCTYPE, html};
use pulldown_cmark::{Parser, html};
use serde::Deserialize;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub const IN_POSTS_CFG_PATH: &str = "posts.toml";
pub const OUT_POSTS_PATH: &str = "build/posts/index.html";
const IN_POSTS_DIR: &str = "posts/";
pub const OUT_POSTS_DIR: &str = "build/posts/";

#[derive(Deserialize, Debug)]

pub struct PostsPage {
    page_title: String,
    title: String,
    desc: String,
}

pub fn get_files_from_posts_dir() -> Result<Vec<PathBuf>> {
    let mut post_fpaths: Vec<PathBuf> = vec![];
    let posts_dir = Path::new(IN_POSTS_DIR);

    if !posts_dir.is_dir() {
        bail!("{IN_POSTS_DIR} doesn't exist or isn't a directory.");
    }

    for entry in fs::read_dir(posts_dir)? {
        let entry = entry?;
        let fpath = entry.path();
        if fpath.extension().and_then(|s| s.to_str()) == Some("md") {
            post_fpaths.push(fpath)
        }
    }

    Ok(post_fpaths)
}

pub fn convert_all_posts_to_html(post_fpaths: Vec<PathBuf>) -> Result<()> {
    let out_dir = Path::new(OUT_POSTS_DIR);

    for fpath in post_fpaths {
        if !fpath.is_file() {
            continue;
        }

        let stem = match fpath.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s,
            None => {
                eprintln!("[POSTS] Failed to extract fname from path: {:#?}", fpath);
                continue;
            }
        };

        let mut out_fpath = out_dir.join(stem);
        out_fpath.set_extension("html");
        let out_file = File::create(out_fpath)?;

        let markdown_input = fs::read_to_string(fpath)?;
        let parser = Parser::new(markdown_input.as_str());
        html::write_html_io(out_file, parser)?;
    }

    Ok(())
}

pub fn create_html_str(pp: &PostsPage) -> String {
    // TODO: Assign CSS classes!

    let markup = html! {
        (DOCTYPE)
        html {
            meta charset="utf-8";
            title {(pp.page_title)}
        }

        h1 {(pp.title)}
        p {(pp.desc)}

        // TODO: create post sections via loop...
    };

    markup.into_string()
}
