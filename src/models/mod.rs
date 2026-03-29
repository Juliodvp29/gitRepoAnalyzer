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

#[derive(Serialize, Clone)]
pub struct CodeSmell {
    pub kind: String,
    pub location: String,
    pub detail: String,
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
    pub code_smells: Vec<CodeSmell>,
    pub ai: Option<AiAnalysis>,
}

#[derive(Deserialize)]
pub struct CompareRequest {
    pub repo_a: String,
    pub repo_b: String,
}

#[derive(Serialize, Deserialize)]
pub struct AiComparison {
    pub verdict: String,
    pub reason: String,
    pub repo_a_strengths: Vec<String>,
    pub repo_b_strengths: Vec<String>,
    pub recommendation: String,
}

#[derive(Serialize)]
pub struct CompareResponse {
    pub repo_a: AnalyzeResponse,
    pub repo_b: AnalyzeResponse,
    pub comparison: Option<AiComparison>,
}