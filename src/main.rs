use pulldown_cmark::{Parser, html};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub mod projects;

const OUTPUT_POSTS_DIR: &str = "build/posts";
const INPUT_POSTS_DIR: &str = "posts/";

fn get_files_from_posts_dir() -> std::io::Result<Vec<PathBuf>> {
    let mut post_fpaths: Vec<PathBuf> = vec![];
    let posts_dir = Path::new(INPUT_POSTS_DIR);

    // TODO: print error if posts_dir doesn't exist

    for entry in fs::read_dir(posts_dir)? {
        let entry = entry?;
        let fpath = entry.path();
        if fpath.extension().and_then(|s| s.to_str()) == Some("md") {
            post_fpaths.push(fpath)
        }
    }

    Ok(post_fpaths)
}

fn main() -> std::io::Result<()> {
    // TODO: check if matching directory structure is found

    // TODO: create output folder structure

    // Projects
    let proj_info_file = projects::parse_projs_index_file()?;
    projects::generate_projs_page_html(&proj_info_file)?;

    // Posts
    let post_fpaths = get_files_from_posts_dir()?;
    for fpath in post_fpaths {
        if !fpath.is_file() {
            continue;
        }

        let stem = match fpath.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s,
            None => continue, // TODO: print warning/error
        };
        let mut out_fpath = Path::new(OUTPUT_POSTS_DIR).join(stem);
        out_fpath.set_extension("html");
        let out_file = File::create(out_fpath)?;

        let markdown_input = fs::read_to_string(fpath)?;
        let parser = Parser::new(markdown_input.as_str());
        html::write_html_io(out_file, parser)?;
    }

    Ok(())
}
