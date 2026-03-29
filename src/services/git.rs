use std::path::PathBuf;
use std::process::Command;

pub struct GitService;

impl GitService {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_url(&self, url: &str) -> Result<(), String> {
        if url.is_empty() {
            return Err("La URL no puede estar vacía.".to_string());
        }

        if !url.starts_with("https://github.com/") {
            return Err("Solo se admiten URLs de GitHub (https://github.com/...).".to_string());
        }

        let parts: Vec<&str> = url
            .trim_end_matches('/')
            .split('/')
            .collect();

        if parts.len() < 5 {
            return Err("URL inválida. Formato esperado: https://github.com/usuario/repositorio".to_string());
        }

        Ok(())
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
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if stderr.contains("not found") || stderr.contains("does not exist") {
                return Err("Repositorio no encontrado. Verifica que la URL sea correcta y que el repo sea público.".to_string());
            }

            if stderr.contains("Authentication failed") {
                return Err("El repositorio es privado. Solo se pueden analizar repositorios públicos.".to_string());
            }

            return Err(format!("No se pudo clonar el repositorio: {}", stderr));
        }

        Ok(temp_dir)
    }

    pub fn cleanup(&self, path: &PathBuf) {
        if path.exists() {
            let _ = std::fs::remove_dir_all(path);
        }
    }

    pub fn last_commit_days(&self, repo_path: &PathBuf) -> Option<u64> {
        let output = Command::new("git")
            .args(["-C", repo_path.to_str().unwrap(), "log", "-1", "--format=%ct"])
            .output()
            .ok()?;

        let timestamp_str = String::from_utf8_lossy(&output.stdout)
            .trim()
            .to_string();

        let timestamp: u64 = timestamp_str.parse().ok()?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_secs();

        let days = (now - timestamp) / 86400;
        Some(days)
    }

    pub fn count_contributors(&self, repo_path: &PathBuf) -> u32 {
        let output = Command::new("git")
            .args(["-C", repo_path.to_str().unwrap(), "shortlog", "-sn", "--all"])
            .output();

        match output {
            Ok(out) => {
                String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .count() as u32
            }
            Err(_) => 0,
        }
    }
}