use regex::RegexBuilder;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Project {
    pub name: String,
    pub platform: String,
    pub interface: String,
    pub r#type: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub files: Option<Vec<FileInfo>>,
    pub neurons: Option<NeuronConfig>,
    pub categories: Option<Vec<Category>>,
    pub feature_extraction: Option<FeatureExtraction>,
    pub learn_properties: Option<LearnProperties>,
    pub recognize_properties: Option<RecognizeProperties>,
    pub annotations: Option<AnnotationData>,
    pub knowledge: Option<Knowledge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FileInfo {
    pub id: u32,
    pub path: String,
    pub r#type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NeuronConfig {
    pub min_if: u32,
    pub max_if: u32,
    pub search_area: Vec<u32>,
    pub total_neurons: u32,
    pub committed_neurons: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub color: String,
    pub context_id: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct FeatureExtraction {
    pub algorithm: String,
    pub normalized: bool,
    pub roi_width: u32,
    pub roi_height: u32,
    pub block_width: u32,
    pub block_height: u32,
    pub if_field_range: IfFieldRange,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct IfFieldRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LearnProperties {
    pub step_xy: StepXY,
    pub auto_learn: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RecognizeProperties {
    pub output_mode: String,
    pub step_xy: StepXY,
    pub skip_xy: SkipXY,
    pub auto_recognize: bool,
    pub full_image: bool,
    pub show_mode: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StepXY {
    pub x: u32,
    pub y: u32,
    pub metric: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SkipXY {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AnnotationData {
    pub options: AnnotationOptions,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AnnotationOptions {
    pub auto_surrounding_examples: String,
    pub positions: AnnotationPositions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AnnotationPositions {
    pub directions: Vec<String>,
    pub distance_to_center: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Annotation {
    pub id: u32,
    pub category_id: u32,
    pub file_id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Knowledge {
    pub categories: Vec<KnowledgeCategory>,
    pub neurons: Vec<KnowledgeNeuron>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KnowledgeCategory {
    pub id: u32,
    pub base64: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KnowledgeNeuron {
    pub neuron_id: u32,
    pub category: u32,
    pub ncr: u32,
    pub model: String,
    pub active_if: u32,
    pub min_if: u32,
    pub degenerate: bool,
}

const PROJECT_FILE_PATH: &str = "assets/realistic_projects.json";

/// Load all projects from the file
pub fn load_projects() -> Vec<Project> {
    File::open(PROJECT_FILE_PATH)
        .ok()
        .map(|f| serde_json::from_reader(BufReader::new(f)).unwrap_or_default())
        .unwrap_or_default()
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

/// Add a new project
pub fn add_project(project: Project) -> std::io::Result<()> {
    let mut projects = load_projects();

    if projects.iter().any(|p| p.name == project.name) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Project with the same name already exists",
        ));
    }

    projects.push(project);
    save_projects(&projects)
}

/// Update an existing project by name
pub fn update_project(name: &str, updated_project: Project) -> std::io::Result<()> {
    let mut projects = load_projects();
    if let Some(pos) = projects.iter().position(|p| p.name == name) {
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
    pub struct Project {}
}
