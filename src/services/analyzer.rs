use std::path::PathBuf;
use crate::models::AnalyzeResponse;

pub struct AnalyzerService;

impl AnalyzerService {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, repo_path: &PathBuf, repo_url: &str, last_commit_days: Option<u64>) -> AnalyzeResponse {
        let has_readme = self.file_exists(repo_path, &["README.md", "readme.md", "README"]);
        let has_tests = self.has_tests(repo_path);
        let technologies = self.detect_technologies(repo_path);
        let dominant_language = self.detect_dominant_language(repo_path);
        let directory_tree = self.build_directory_tree(repo_path);
        let total_files = self.count_files(repo_path);
        let total_lines = self.count_lines(repo_path);
        let dependency_count = self.count_dependencies(repo_path);
        let (has_license, license_type) = self.detect_license(repo_path);
        let score = self.calculate_score(has_readme, has_tests, &technologies, last_commit_days);

        AnalyzeResponse {
            repo_url: repo_url.to_string(),
            technologies,
            dominant_language,
            directory_tree,
            total_files,
            total_lines,
            dependency_count,
            has_readme,
            has_tests,
            has_license,
            license_type,
            last_commit_days,
            contributors: 0,
            score,
            ai: None,
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


    pub fn detect_technologies(&self, base: &PathBuf) -> Vec<String> {
        let mut techs = Vec::new();

        let indicators = [
            ("Cargo.toml",              "Rust"),
            ("package.json",            "Node.js"),
            ("requirements.txt",        "Python"),
            ("pyproject.toml",          "Python"),
            ("go.mod",                  "Go"),
            ("pom.xml",                 "Java"),
            ("build.gradle",            "Java/Kotlin"),
            ("*.cs",                    "C#"),
            ("*.cpp",                   "C++"),
            ("*.c",                     "C"),
            ("*.hs",                    "Haskell"),
            ("*.rb",                    "Ruby"),
            ("Gemfile",                 "Ruby"),
            ("*.php",                   "PHP"),
            ("composer.json",           "PHP"),
            ("*.swift",                 "Swift"),
            ("*.kt",                    "Kotlin"),
            ("*.ex",                    "Elixir"),
            ("mix.exs",                 "Elixir"),
            ("angular.json",            "Angular"),
            ("next.config.js",          "Next.js"),
            ("next.config.ts",          "Next.js"),
            ("nuxt.config.js",          "Nuxt.js"),
            ("nuxt.config.ts",          "Nuxt.js"),
            ("svelte.config.js",        "Svelte"),
            ("vite.config.js",          "Vite"),
            ("vite.config.ts",          "Vite"),
            ("Dockerfile",              "Docker"),
            ("docker-compose.yml",      "Docker Compose"),
            ("docker-compose.yaml",     "Docker Compose"),
            ("*.tf",                    "Terraform"),
            (".github/workflows",       "GitHub Actions"),
            ("prisma",                  "Prisma ORM"),
            ("drizzle.config.ts",       "Drizzle ORM"),
            (".env.example",            "Env Config"),
        ];

        for (file, tech) in &indicators {
            if file.starts_with('.') || !file.contains('.') && !file.contains('*') {
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


    fn detect_dominant_language(&self, base: &PathBuf) -> Option<String> {
        let language_extensions = [
            ("rs",  "Rust"),
            ("js",  "JavaScript"),
            ("ts",  "TypeScript"),
            ("py",  "Python"),
            ("go",  "Go"),
            ("java","Java"),
            ("cs",  "C#"),
            ("cpp", "C++"),
            ("c",   "C"),
            ("rb",  "Ruby"),
            ("php", "PHP"),
            ("kt",  "Kotlin"),
            ("swift","Swift"),
            ("ex",  "Elixir"),
            ("hs",  "Haskell"),
        ];

        let mut counts: Vec<(String, usize)> = language_extensions
            .iter()
            .map(|(ext, lang)| {
                let count = self.count_files_with_extension(base, ext);
                (lang.to_string(), count)
            })
            .filter(|(_, count)| *count > 0)
            .collect();

        counts.sort_by(|a, b| b.1.cmp(&a.1));
        counts.into_iter().next().map(|(lang, _)| lang)
    }

    fn count_files_with_extension(&self, base: &PathBuf, ext: &str) -> usize {
        self.walk_files(base)
            .iter()
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some(ext))
            .count()
    }


    fn build_directory_tree(&self, base: &PathBuf) -> Vec<String> {
        let mut tree = Vec::new();
        self.walk_tree(base, base, 0, &mut tree);
        tree
    }

    fn walk_tree(&self, base: &PathBuf, current: &PathBuf, depth: usize, tree: &mut Vec<String>) {
        if depth > 2 { return; }

        let Ok(entries) = std::fs::read_dir(current) else { return };

        let mut entries: Vec<_> = entries.flatten().collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if name == ".git" || name == "node_modules" || name == "target" {
                continue;
            }

            let indent = "  ".repeat(depth);
            if path.is_dir() {
                tree.push(format!("{}{}/", indent, name));
                self.walk_tree(base, &path, depth + 1, tree);
            } else {
                tree.push(format!("{}{}", indent, name));
            }
        }
    }


    fn count_files(&self, base: &PathBuf) -> u32 {
        self.walk_files(base).len() as u32
    }

    fn count_lines(&self, base: &PathBuf) -> u32 {
        let code_extensions = ["rs", "js", "ts", "py", "go", "java", "cs", "cpp", "c", "rb", "php", "kt", "swift"];

        self.walk_files(base)
            .iter()
            .filter(|p| {
                p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| code_extensions.contains(&e))
                    .unwrap_or(false)
            })
            .filter_map(|p| std::fs::read_to_string(p).ok())
            .map(|content| content.lines().count() as u32)
            .sum()
    }

    fn walk_files(&self, base: &PathBuf) -> Vec<PathBuf> {
        let mut files = Vec::new();
        self.collect_files(base, &mut files);
        files
    }

    fn collect_files(&self, dir: &PathBuf, files: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };

        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if name == ".git" || name == "node_modules" || name == "target" {
                continue;
            }

            if path.is_dir() {
                self.collect_files(&path, files);
            } else {
                files.push(path);
            }
        }
    }


    fn count_dependencies(&self, base: &PathBuf) -> u32 {
        if let Ok(content) = std::fs::read_to_string(base.join("Cargo.toml")) {
            if let Some(start) = content.find("[dependencies]") {
                let section = &content[start..];
                let count = section
                    .lines()
                    .skip(1)
                    .take_while(|l| !l.starts_with('['))
                    .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
                    .count();
                return count as u32;
            }
        }

        if let Ok(content) = std::fs::read_to_string(base.join("package.json")) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                let deps = json["dependencies"].as_object().map(|o| o.len()).unwrap_or(0);
                let dev_deps = json["devDependencies"].as_object().map(|o| o.len()).unwrap_or(0);
                return (deps + dev_deps) as u32;
            }
        }

        if let Ok(content) = std::fs::read_to_string(base.join("requirements.txt")) {
            let count = content
                .lines()
                .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
                .count();
            return count as u32;
        }

        0
    }


    fn detect_license(&self, base: &PathBuf) -> (bool, Option<String>) {
        let license_files = ["LICENSE", "LICENSE.md", "LICENSE.txt", "LICENCE"];

        for name in &license_files {
            let path = base.join(name);
            if path.exists() {
                let license_type = std::fs::read_to_string(&path)
                    .ok()
                    .map(|content| self.classify_license(&content));
                return (true, license_type);
            }
        }

        (false, None)
    }

    fn classify_license(&self, content: &str) -> String {
        let content_lower = content.to_lowercase();

        if content_lower.contains("mit license") || content_lower.contains("permission is hereby granted") {
            "MIT".to_string()
        } else if content_lower.contains("apache license") {
            "Apache 2.0".to_string()
        } else if content_lower.contains("gnu general public") {
            "GPL".to_string()
        } else if content_lower.contains("bsd") {
            "BSD".to_string()
        } else if content_lower.contains("mozilla public license") {
            "MPL".to_string()
        } else {
            "Other".to_string()
        }
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

        score.clamp(0, 10) as u8
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

    pub fn build_ai_context(&self, response: &AnalyzeResponse, readme: Option<String>) -> String {
        let readme_section = match readme {
            Some(content) => {
                let trimmed = content.chars().take(1500).collect::<String>();
                format!("\n\nContenido del README:\n{}", trimmed)
            }
            None => "\n\nEste repositorio no tiene README.".to_string(),
        };

        format!(
            "Repositorio: {}\nTecnologías: {}\nLenguaje dominante: {}\nArchivos totales: {}\nLíneas de código: {}\nDependencias: {}\nTiene README: {}\nTiene tests: {}\nTiene licencia: {} ({})\nDías desde último commit: {}\nContribuidores: {}\nScore de calidad: {}/10{}",
            response.repo_url,
            response.technologies.join(", "),
            response.dominant_language.as_deref().unwrap_or("desconocido"),
            response.total_files,
            response.total_lines,
            response.dependency_count,
            response.has_readme,
            response.has_tests,
            response.has_license,
            response.license_type.as_deref().unwrap_or("N/A"),
            response.last_commit_days.map(|d| d.to_string()).unwrap_or("desconocido".to_string()),
            response.contributors,
            response.score,
            readme_section,
        )
    }
}