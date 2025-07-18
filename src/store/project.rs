use dioxus::prelude::*; // ✅ Fixes the Signal error
use regex::RegexBuilder;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

use crate::store::project_schema::{Project};

const PROJECT_FILE_PATH: &str = "assets/realistic_projects.json";
#[derive(Clone)]
pub struct ProjectState {
    pub project_id: Signal<String>,
    pub project_name: Signal<String>,
}

impl ProjectState {
    pub fn new() -> Self {
        Self {
            project_id: use_signal(|| "0".to_string()),
            project_name: use_signal(|| "null".to_string()),
        }
    }
}
/// Load all projects from the file
pub fn load_projects() -> Vec<Project> {
    match File::open(PROJECT_FILE_PATH) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("❌ JSON parse error: {}", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to open file: {}", e);
            Vec::new()
        }
    }
}

/// Save all projects to the file
pub fn save_projects(projects: &[Project]) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(PROJECT_FILE_PATH)?;
    serde_json::to_writer_pretty(BufWriter::new(file), projects)?;
    Ok(())
}

/// Get a project by its name
pub fn get_projects_by_name(query: &str) -> Vec<Project> {
    let pattern = query.replace(' ', ".*"); // make space act like wildcard
    let regex = RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()
        .unwrap_or_else(|_| RegexBuilder::new(".*").build().unwrap()); // fallback to match all

    load_projects()
        .into_iter()
        .filter(|p| regex.is_match(&p.name))
        .collect()
}


/// Get a project by id 
pub fn get_project_by_id(id: &str) -> Option<Project> {
    load_projects().into_iter().find(|p| p.id == id)    
}
/// Add a new project
pub fn add_project(project: Project) -> std::io::Result<()> {
    let mut projects = load_projects();

    if projects.iter().any(|p| p.name == project.name) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Project with the same name already exists",
        ));
    }

    // projects.push(project);
    projects.insert(0, project);
    save_projects(&projects)
}

/// Update an existing project by name
pub fn update_project(id: &str, updated_project: Project) -> std::io::Result<()> {
    println!("Updating project with id: {}", id);
    let mut projects = load_projects();
    if let Some(pos) = projects.iter().position(|p| p.id == id) {
        projects[pos] = updated_project;
        save_projects(&projects)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Project not found",
        ))
    }
}

/// Delete a project by name
pub fn delete_project(name: &str) -> std::io::Result<()> {
    let mut projects = load_projects();
    let original_len = projects.len();
    projects.retain(|p| p.name != name);
    if projects.len() != original_len {
        save_projects(&projects)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Project not found",
        ))
    }
}

// project_store.rs
pub mod project_store {
    pub fn add_project() {}
    pub fn delete_project() {}
    pub fn get_project_by_name() {}
    pub fn load_projects() {}
    pub fn update_project() {}
}
