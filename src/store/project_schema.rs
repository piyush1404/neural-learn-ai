use serde::{Deserialize, Serialize};

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


#[derive(Serialize, Deserialize, Debug)]
pub struct FieldRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectFormData {
    pub name: String,
    pub platform: String,
    pub interface: String,
    pub project_type: String,
    pub description: String,
    pub categories: Vec<Category>,
    pub feature_extraction: FeatureExtraction,
}



impl ProjectFormData {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Project name cannot be empty".into());
        }
        if self.platform.trim().is_empty() {
            return Err("Platform must be selected".into());
        }
        if self.project_type.trim().is_empty() {
            return Err("Type must be selected".into());
        }
        if self.description.len() > 100 {
            return Err("Description must be under 100 characters".into());
        }
        if self.feature_extraction.if_field_range.min >= self.feature_extraction.if_field_range.max {
            return Err("Invalid influence field range (min should be < max)".into());
        }
        Ok(())
    }
}


