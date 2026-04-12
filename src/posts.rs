use anyhow::{Context, Result, bail};
use pulldown_cmark::{Parser, html};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

const INPUT_POSTS_DIR: &str = "posts/";
const OUTPUT_POSTS_DIR: &str = "build/posts/";

pub fn get_files_from_posts_dir() -> Result<Vec<PathBuf>> {
    let mut post_fpaths: Vec<PathBuf> = vec![];
    let posts_dir = Path::new(INPUT_POSTS_DIR);

    if !posts_dir.is_dir() {
        bail!("{INPUT_POSTS_DIR} doesn't exist or isn't a directory.");
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
    let out_dir = Path::new(OUTPUT_POSTS_DIR);
    std::fs::create_dir_all(out_dir)
        .with_context(|| format!("Failed to create dir {OUTPUT_POSTS_DIR}"))?;

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
