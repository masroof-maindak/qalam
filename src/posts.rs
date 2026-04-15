use anyhow::{Context, Result, anyhow, bail};
use chrono::NaiveDate;
use gray_matter::{Matter, engine::TOML};
use maud::html;
use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;
use std::fs;
use std::iter::zip;
use std::path::{Path, PathBuf};

use crate::utils;

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

#[derive(Deserialize, Debug)]
struct FrontMatter {
    title: String,
    date: String, // toml::value::Date doesn't work for some reason. At least not for my desired format (YYYY-MM-DD), which is all I tested.
}

#[derive(Debug)]
struct NoteMetadata {
    // Generated using frontmatter
    title: String,
    date: chrono::NaiveDate,
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

pub fn generate_html_str(fpath: &Path, out_path: &Path) -> Result<String> {
    // Extract frontmatter
    let markdown_input = fs::read_to_string(fpath)?;
    let matter = Matter::<TOML>::new();
    let md_doc = matter
        .parse::<FrontMatter>(&markdown_input)
        .with_context(|| format!("[POSTS] Failed to extract frontmatter from {:?}", fpath))?;

    // Convert frontmatter to metadata
    let note_md = match md_doc.data {
        Some(fm) => {
            let naive_date =
                NaiveDate::parse_from_str(&fm.date, "%Y-%m-%d").with_context(|| {
                    format!(
                        "[POSTS] Failed to parse date from str {} for file {:#?}",
                        &fm.date, &fpath
                    )
                })?;

            NoteMetadata {
                title: fm.title,
                date: naive_date,
            }
        }
        None => bail!(
            "[POSTS] Failed to extract frontmatter from file {:#?}",
            &fpath
        ),
    };

    // Create output file
    let parser = Parser::new_ext(&md_doc.content, Options::all());
    let mut note_content: String = generate_header(note_md);
    html::write_html_fmt(&mut note_content, parser)?;

    utils::write_html(&note_content, &out_path)?;
    Ok(note_content)
}

fn extract_stem_from_fpath(fpath: &PathBuf) -> Result<&str> {
    fpath.file_stem().and_then(|s| s.to_str()).ok_or(anyhow!(
        "[POSTS] Failed to extract fname from path: {:#?}",
        fpath
    ))
}

pub fn generate_out_path_vec(post_fpaths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let out_dir = Path::new(OUT_POSTS_DIR);

    post_fpaths
        .iter()
        .map(|fpath| {
            let stem = match extract_stem_from_fpath(fpath) {
                Ok(s) => s,
                Err(e) => bail!("Failed to extract stem from {:#?}: {e}", fpath),
            };

            let mut out_fpath = out_dir.join(stem);
            out_fpath.set_extension("html");
            Ok(out_fpath)
        })
        .collect()
}

pub fn generate_html_for_all_posts(post_fpaths: &Vec<PathBuf>) -> Result<()> {
    let post_out_fpaths_it = generate_out_path_vec(post_fpaths)?;

    for (fpath, out_fpath) in zip(post_fpaths, post_out_fpaths_it) {
        if !fpath.is_file() {
            continue;
        }

        let note_content = match generate_html_str(fpath, &out_fpath) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to convert {:#?} to HTML str {e}", &fpath);
                continue;
            }
        };

        utils::write_html(&note_content, &out_fpath.as_path())?;
    }

    Ok(())
}

fn generate_header(note_md: NoteMetadata) -> String {
    html!(
        (utils::page_header(&note_md.title))
        h1 {(note_md.title)}
        span {(note_md.date)}
    )
    .into_string()
}

pub fn create_index_html_str(pp: &PostsPage, post_fpaths: &Vec<PathBuf>) -> Result<String> {
    // TODO: Assign CSS classes!

    let post_out_paths = generate_out_path_vec(post_fpaths)?;

    let markup = html! {
        (utils::page_header(&pp.page_title))
        h1 {(pp.title)}
        p {(pp.desc)}

        @for (_fpath, _out_path) in zip(post_fpaths, post_out_paths) {
            // TODO: create post sections; need metadata
        }

    };

    Ok(markup.into_string())
}
