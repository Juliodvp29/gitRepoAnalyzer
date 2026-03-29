use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use crate::models::{AnalyzeRequest, AnalyzeResponse};
use crate::services::{ai::AiService, git::GitService, analyzer::AnalyzerService};

pub struct AppState {
    pub ai_service: AiService,
}

#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
}

fn bad_request(msg: &str) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ApiError { error: msg.to_string() }),
    )
}

#[allow(dead_code)]
fn internal_error(msg: &str) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError { error: msg.to_string() }),
    )
}

pub async fn analyze_repo(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, (StatusCode, Json<ApiError>)> {

    let git = GitService::new();
    let analyzer = AnalyzerService::new();

    git.validate_url(&payload.repo_url)
        .map_err(|e| bad_request(&e))?;

    let repo_path = git.clone_repo(&payload.repo_url)
        .map_err(|e| bad_request(&e))?;

    let last_commit_days = git.last_commit_days(&repo_path);

    let mut result = analyzer.analyze(&repo_path, &payload.repo_url, last_commit_days);

    let readme = analyzer.read_readme(&repo_path);
    let context = analyzer.build_ai_context(&result, readme);

    let summary = state.ai_service.summarize(&context)
        .await
        .unwrap_or_else(|e| {
            tracing::warn!("Gemini falló: {}", e);
            "No se pudo generar resumen automático.".to_string()
        });

    result.summary = summary;

    git.cleanup(&repo_path);

    Ok(Json(result))
}