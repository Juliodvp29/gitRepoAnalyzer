use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub repo_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct AiAnalysis {
    pub summary: String,
    pub complexity: String,        
    pub category: String,         
    pub difficulty: String,        
    pub suggestions: Vec<String>,  
}

#[derive(Serialize)]
pub struct AnalyzeResponse {
    pub repo_url: String,
    pub technologies: Vec<String>,
    pub dominant_language: Option<String>,
    pub directory_tree: Vec<String>,
    pub total_files: u32,
    pub total_lines: u32,
    pub dependency_count: u32,
    pub has_readme: bool,
    pub has_tests: bool,
    pub has_license: bool,
    pub license_type: Option<String>,
    pub last_commit_days: Option<u64>,
    pub contributors: u32,
    pub score: u8,
    pub ai: Option<AiAnalysis>,
}