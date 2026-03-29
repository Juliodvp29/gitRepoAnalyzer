use std::path::PathBuf;
use crate::models::{AnalyzeResponse};

pub struct AnalyzerService;

impl AnalyzerService {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, repo_path: &PathBuf, repo_url: &str) -> AnalyzeResponse {
        let has_readme = self.file_exists(repo_path, &["README.md", "readme.md", "README"]);
        let has_tests = self.has_tests(repo_path);
        let technologies = self.detect_technologies(repo_path);
        let score = self.calculate_score(has_readme, has_tests, &technologies);

        AnalyzeResponse {
            repo_url: repo_url.to_string(),
            technologies,
            has_readme,
            has_tests,
            score,
            summary: String::new(), 
        }
    }

    fn file_exists(&self, base: &PathBuf, names: &[&str]) -> bool {
        names.iter().any(|name| base.join(name).exists())
    }

    fn has_tests(&self, base: &PathBuf) -> bool {
        let test_dirs = ["tests", "test", "__tests__", "spec"];
        if test_dirs.iter().any(|d| base.join(d).is_dir()) {
            return true;
        }

        let test_files = ["jest.config.js", "pytest.ini", "vitest.config.ts"];
        test_files.iter().any(|f| base.join(f).exists())
    }

    fn detect_technologies(&self, base: &PathBuf) -> Vec<String> {
        let mut techs = Vec::new();

        let indicators = [
            ("Cargo.toml",       "Rust"),
            ("package.json",     "Node.js"),
            ("angular.json",     "Angular"),
            ("requirements.txt", "Python"),
            ("go.mod",           "Go"),
            ("pom.xml",          "Java"),
            ("Dockerfile",       "Docker"),
            ("docker-compose.yml","Docker Compose"),
            ("*.tf",             "Terraform"),
        ];

        for (file, tech) in &indicators {
            if file.contains('*') {
                let ext = file.replace("*.", "");
                if self.has_extension(base, &ext) {
                    techs.push(tech.to_string());
                }
            } else if base.join(file).exists() {
                techs.push(tech.to_string());
            }
        }

        techs
    }

    fn has_extension(&self, base: &PathBuf, ext: &str) -> bool {
        if let Ok(entries) = std::fs::read_dir(base) {
            for entry in entries.flatten() {
                if let Some(e) = entry.path().extension() {
                    if e == ext {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn calculate_score(&self, has_readme: bool, has_tests: bool, technologies: &[String]) -> u8 {
        let mut score: i8 = 5; 

        if has_readme { score += 2; }
        if has_tests  { score += 3; }
        if !technologies.is_empty() { score += 1; }
        if score > 10 { score = 10; }
        if score < 0  { score = 0; }

        score as u8
    }

    pub fn build_ai_context(&self, response: &AnalyzeResponse) -> String {
        format!(
            "Repositorio: {}\nTecnologías: {}\nTiene README: {}\nTiene tests: {}\nScore de calidad: {}/10",
            response.repo_url,
            response.technologies.join(", "),
            response.has_readme,
            response.has_tests,
            response.score,
        )
    }
}