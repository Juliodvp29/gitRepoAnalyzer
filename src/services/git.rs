use std::path::PathBuf;
use std::process::Command;

pub struct GitService;

impl GitService {
    pub fn new() -> Self {
        Self
    }

    pub fn clone_repo(&self, repo_url: &str) -> Result<PathBuf, String> {
        let repo_name = repo_url
            .split('/')
            .last()
            .unwrap_or("repo")
            .replace(".git", "");

        let temp_dir = std::env::temp_dir().join(format!("ghan_{}", repo_name));

        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir)
                .map_err(|e| format!("Error al limpiar directorio temporal: {}", e))?;
        }

        let output = Command::new("git")
            .args(["clone", "--depth=1", repo_url, temp_dir.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Error al ejecutar git: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Git clone falló: {}", stderr));
        }

        Ok(temp_dir)
    }

    pub fn cleanup(&self, path: &PathBuf) {
        if path.exists() {
            let _ = std::fs::remove_dir_all(path);
        }
    }
}