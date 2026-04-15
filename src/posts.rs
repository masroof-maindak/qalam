use anyhow::{Context, Result, anyhow, bail};
use camino::{Utf8Path, Utf8PathBuf};
use chrono::NaiveDate;
use gray_matter::{Matter, ParsedEntity, engine::TOML};
use itertools::Itertools;
use maud::html;
use pulldown_cmark::{Options, Parser, html};
use serde::Deserialize;
use std::fs::read_to_string;
use std::iter::zip;

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

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
struct NoteMetadata {
    // Generated from FrontMatter
    title: String,
    date: chrono::NaiveDate,
}

pub fn get_files_from_posts_dir() -> Result<Vec<Utf8PathBuf>> {
    let mut post_fpaths: Vec<Utf8PathBuf> = vec![];
    let posts_dir = Utf8Path::new(IN_POSTS_DIR);

    if !posts_dir.is_dir() {
        bail!("[POSTS] {IN_POSTS_DIR} doesn't exist or isn't a directory.");
    }

    for entry in posts_dir.read_dir_utf8()? {
        let entry = entry?;
        let fpath = entry.path();
        if fpath.extension() == Some("md") {
            post_fpaths.push(Utf8PathBuf::from(fpath))
        }
    }

    Ok(post_fpaths)
}

fn extract_frontmatter(fpath: &Utf8Path) -> Result<ParsedEntity<FrontMatter>> {
    let markdown_input = read_to_string(fpath)?;
    let matter = Matter::<TOML>::new();
    let md_doc = matter
        .parse::<FrontMatter>(&markdown_input)
        .with_context(|| format!("[POSTS] Failed to extract frontmatter from {:?}", fpath))?;
    Ok(md_doc)
}

fn convert_frontmatter_to_metadata(
    fpath: &Utf8Path,
    md_doc: &ParsedEntity<FrontMatter>,
) -> Result<NoteMetadata> {
    let note_md = match &md_doc.data {
        Some(fm) => {
            let naive_date =
                NaiveDate::parse_from_str(&fm.date, "%Y-%m-%d").with_context(|| {
                    format!(
                        "[POSTS] Failed to parse date from str {} for file {:#?}",
                        &fm.date, &fpath
                    )
                })?;

            NoteMetadata {
                title: fm.title.clone(),
                date: naive_date,
            }
        }
        None => bail!(
            "[POSTS] Failed to extract frontmatter from file {:#?}",
            &fpath
        ),
    };

    Ok(note_md)
}

fn extract_metadata_and_content(
    fpath: &Utf8Path,
) -> Result<(NoteMetadata, ParsedEntity<FrontMatter>)> {
    let fm = extract_frontmatter(fpath)?;
    let note_md = convert_frontmatter_to_metadata(fpath, &fm)?;
    Ok((note_md, fm))
}

pub fn generate_html_str(fpath: &Utf8Path) -> Result<String> {
    let (note_md, fm) = extract_metadata_and_content(fpath)?;

    let parser = Parser::new_ext(&fm.content, Options::all());
    let mut note_content: String = generate_header(note_md);
    html::write_html_fmt(&mut note_content, parser)?;

    Ok(note_content)
}

fn extract_stem_from_fpath(fpath: &Utf8PathBuf) -> Result<&str> {
    fpath.file_stem().ok_or(anyhow!(
        "[POSTS] Failed to extract fname from path: {:#?}",
        fpath
    ))
}

pub fn generate_out_path_vec(post_fpaths: &[Utf8PathBuf]) -> Result<Vec<Utf8PathBuf>> {
    let out_dir = Utf8Path::new(OUT_POSTS_DIR);

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

pub fn generate_html_files_all_posts(post_fpaths: &Vec<Utf8PathBuf>) -> Result<()> {
    let post_out_fpaths = generate_out_path_vec(post_fpaths)?;

    for (fpath, out_fpath) in zip(post_fpaths, post_out_fpaths) {
        if !fpath.is_file() {
            continue;
        }

        let note_content = match generate_html_str(fpath) {
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
        // TODO: add tags here, eventually
    )
    .into_string()
}

pub fn create_index_html_str(pp: &PostsPage, post_fpaths: &[Utf8PathBuf]) -> Result<String> {
    // TODO: Assign CSS classes!

    let post_out_paths = generate_out_path_vec(post_fpaths)?;
    let post_out_fnames = post_out_paths
        .iter()
        .filter_map(|fpath| match fpath.file_name() {
            Some(s) => Some(s),
            None => {
                eprintln!("[POSTS] failed to extract filename from path {fpath}");
                None
            }
        });

    let post_metadatas = post_fpaths
        .iter()
        .map(|f| extract_metadata_and_content(f))
        .collect::<Result<Vec<(NoteMetadata, ParsedEntity<FrontMatter>)>>>()?;

    let out_path_metadata_it =
        zip(post_out_fnames, post_metadatas).sorted_by_key(|(_, md)| md.0.date);

    let markup = html! {
        (utils::page_header(&pp.page_title))
        h1 {(pp.title)}
        p {(pp.desc)}

        div .post-list {
            @for (rel_url, md) in out_path_metadata_it {
                a .post-list-entry href={(rel_url)} {(md.0.title)}
                span {(md.0.date)}
                br;
            }
        }
    };

    Ok(markup.into_string())
}
