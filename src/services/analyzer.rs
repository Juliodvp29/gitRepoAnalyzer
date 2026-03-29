use std::path::PathBuf;
use crate::models::{AnalyzeResponse};

pub struct AnalyzerService;

impl AnalyzerService {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, repo_path: &PathBuf, repo_url: &str, last_commit_days: Option<u64>) -> AnalyzeResponse {
        let has_readme = self.file_exists(repo_path, &["README.md", "readme.md", "README"]);
        let has_tests = self.has_tests(repo_path);
        let technologies = self.detect_technologies(repo_path);
        let score = self.calculate_score(has_readme, has_tests, &technologies, last_commit_days);

        AnalyzeResponse {
            repo_url: repo_url.to_string(),
            technologies,
            has_readme,
            has_tests,
            score,
            last_commit_days,
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
            ("Cargo.toml",            "Rust"),
            ("package.json",          "Node.js"),
            ("requirements.txt",      "Python"),
            ("pyproject.toml",        "Python"),
            ("go.mod",                "Go"),
            ("pom.xml",               "Java"),
            ("build.gradle",          "Java/Kotlin"),
            ("*.cs",                  "C#"),
            ("*.cpp",                 "C++"),
            ("*.c",                   "C"),
            ("*.hs",                  "Haskell"),
            ("*.rb",                  "Ruby"),
            ("Gemfile",               "Ruby"),
            ("*.php",                 "PHP"),
            ("composer.json",         "PHP"),
            ("*.swift",               "Swift"),
            ("*.kt",                  "Kotlin"),
            ("*.ex",                  "Elixir"),
            ("mix.exs",               "Elixir"),
            ("*.rs",                  "Rust"),

            ("angular.json",          "Angular"),
            ("next.config.js",        "Next.js"),
            ("next.config.ts",        "Next.js"),
            ("nuxt.config.js",        "Nuxt.js"),
            ("nuxt.config.ts",        "Nuxt.js"),
            ("svelte.config.js",      "Svelte"),
            ("vite.config.js",        "Vite"),
            ("vite.config.ts",        "Vite"),

            ("Dockerfile",            "Docker"),
            ("docker-compose.yml",    "Docker Compose"),
            ("docker-compose.yaml",   "Docker Compose"),
            ("*.tf",                  "Terraform"),
            ("*.yml",                 "YAML/CI"),
            (".github/workflows",     "GitHub Actions"),
            ("k8s",                   "Kubernetes"),
            ("helm",                  "Helm"),

            (".env.example",          "Env Config"),
            ("prisma",                "Prisma ORM"),
            ("drizzle.config.ts",     "Drizzle ORM"),
        ];

        for (file, tech) in &indicators {
            if file.starts_with('.') {
                if base.join(file).exists() {
                    techs.push(tech.to_string());
                }
            } else if file.contains('*') {
                let ext = file.replace("*.", "");
                if self.has_extension(base, &ext) {
                    techs.push(tech.to_string());
                }
            } else if base.join(file).exists() {
                techs.push(tech.to_string());
            }
        }

        techs.dedup();
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

    fn calculate_score(&self, has_readme: bool, has_tests: bool, technologies: &[String], last_commit_days: Option<u64>) -> u8 {
        let mut score: i8 = 5;

        if has_readme { score += 2; }
        if has_tests  { score += 3; }
        if !technologies.is_empty() { score += 1; }

        if let Some(days) = last_commit_days {
            if days > 180 { score -= 2; }
            if days > 365 { score -= 1; } 
        }

        score = score.clamp(0, 10);
        score as u8
    }

    pub fn build_ai_context(&self, response: &AnalyzeResponse, readme: Option<String>) -> String {
        let readme_section = match readme {
            Some(content) => {
                let trimmed = content.chars().take(1500).collect::<String>();
                format!("\n\nContenido del README:\n{}", trimmed)
            }
            None => "\n\nEste repositorio no tiene README.".to_string(),
        };

        format!(
            "Repositorio: {}\nTecnologías: {}\nTiene README: {}\nTiene tests: {}\nScore de calidad: {}/10{}",
            response.repo_url,
            response.technologies.join(", "),
            response.has_readme,
            response.has_tests,
            response.score,
            readme_section,
        )
    }

    pub fn read_readme(&self, base: &PathBuf) -> Option<String> {
        let names = ["README.md", "readme.md", "README", "README.txt"];
        for name in &names {
            let path = base.join(name);
            if path.exists() {
                return std::fs::read_to_string(path).ok();
            }
        }
        None
    }
}