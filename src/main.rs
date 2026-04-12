use anyhow::Result;

pub mod home;
pub mod posts;
pub mod projects;
pub mod utils;

use utils::{TomlFileType, parse_toml_file, write_html};

fn main() -> Result<()> {
    // TODO: create build/ dir

    // Homepage
    let home_cfg_file = parse_toml_file(TomlFileType::Home, home::IN_HOME_CFG_PATH)?;
    let home_page_html = home::create_html_str(&home_cfg_file.into_home()?);
    write_html(home_page_html, home::OUT_HOME_CFG_PATH)?;

    // Projects
    let proj_cfg_file = parse_toml_file(TomlFileType::Proj, projects::IN_PROJS_CFG_PATH)?;
    let projs_page_html = projects::create_html_str(&proj_cfg_file.into_proj()?);
    write_html(projs_page_html, projects::OUT_PROJ_PATH)?;

    // Posts
    let post_fpaths = posts::get_files_from_posts_dir()?;
    posts::convert_all_posts_to_html(post_fpaths)?;

    // Posts - Index
    let posts_cfg_file = parse_toml_file(TomlFileType::Posts, posts::IN_POSTS_CFG_PATH)?;
    let posts_page_html = posts::create_html_str(&posts_cfg_file.into_post()?);
    write_html(posts_page_html, posts::OUT_POSTS_PATH)?;

    Ok(())
}
