use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub repo_url: String,
}

#[derive(Serialize)]
pub struct AnalyzeResponse {
    pub repo_url: String,
    pub technologies: Vec<String>,
    pub has_readme: bool,
    pub has_tests: bool,
    pub score: u8,
    pub last_commit_days: Option<u64>,
    pub summary: String,
}