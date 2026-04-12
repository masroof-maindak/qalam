use anyhow::Result;

pub mod projects;
pub mod posts;

fn main() -> Result<()> {
    // TODO: check for index.toml

    // Projects
    let proj_info_file = projects::parse_projs_index_file()?;
    let projs_page_html = projects::generate_projs_page_html(&proj_info_file)?;
    projects::write_projs_html_page(projs_page_html)?;

    // Posts
    let post_fpaths = posts::get_files_from_posts_dir()?;
    posts::convert_all_posts_to_html(post_fpaths)?;

    Ok(())
}
