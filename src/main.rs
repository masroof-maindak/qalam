use anyhow::{Context, Result};
use std::env;

pub mod home;
pub mod posts;
pub mod projects;
pub mod utils;

use utils::{
    TomlFileType, copy_images_to_build, generate_css_with_override, parse_toml_file, write_html,
};

pub const IMAGES_DIR: &str = "img/";
pub const OUT_IMAGES_DIR: &str = "build/img/";

const OUT_DIRS: [&str; 4] = [
    posts::OUT_POSTS_DIR,
    projects::OUT_PROJ_DIR,
    "build/themes/",
    OUT_IMAGES_DIR,
];

fn main() -> Result<()> {
    let start_path = env::current_dir()?;

    // Chdir if provided
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("Usage: {} <dir>", args[0]);
        std::process::exit(1);
    } else if args.len() == 2 {
        std::env::set_current_dir(&args[1])?;
    }

    // Ensure output dirs
    for d in OUT_DIRS {
        std::fs::create_dir_all(d).with_context(|| format!("Failed to create_dir {}", d))?;
    }

    copy_images_to_build(IMAGES_DIR, &OUT_IMAGES_DIR)?;

    generate_css_with_override(&start_path.join(utils::CSS_PATH))?;

    // Homepage
    let home_cfg_file = parse_toml_file(TomlFileType::Home, home::IN_HOME_CFG_PATH)?;
    let home_page_html = home::create_html_str(&home_cfg_file.into_home()?);
    write_html(&home_page_html, &home::OUT_HOME_CFG_PATH)?;

    // Projects
    let proj_cfg_file = parse_toml_file(TomlFileType::Proj, projects::IN_PROJS_CFG_PATH)?;
    let projs_page_html = projects::create_html_str(&proj_cfg_file.into_proj()?);
    write_html(&projs_page_html, &projects::OUT_PROJ_PATH)?;

    // Posts
    let post_fpaths = posts::get_files_from_posts_dir()?;
    posts::generate_html_files_all_posts(&post_fpaths)?;

    // Posts - Index
    let posts_cfg_file = parse_toml_file(TomlFileType::Posts, posts::IN_POSTS_CFG_PATH)?;
    let posts_page_html = posts::create_index_html_str(&posts_cfg_file.into_post()?, &post_fpaths)?;
    write_html(&posts_page_html, &posts::OUT_POSTS_PATH)?;

    Ok(())
}
