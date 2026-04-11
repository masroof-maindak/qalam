use serde::{Serialize, Deserialize};

const OUTPUT_PROJECTS_FPATH: &str = "build/projects.html";
const INPUT_PROJECTS_FILE: &str = "build/projects.html";

#[derive(Serialize, Deserialize, Debug)]
struct Project {
	title: String,
    bio: String,
	url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectPage {
    title: String,
    bio: String,
	projs: Vec<Project>,
}

// TODO: output projects.html from aforementioned

pub fn parse_projs_index_file() -> std::io::Result<ProjectPage> {
	todo!()
}

pub fn generate_projs_page_html(_p: &ProjectPage) -> std::io::Result<()> {
	todo!()
}