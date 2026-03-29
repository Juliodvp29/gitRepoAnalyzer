use axum::{extract::State, http::StatusCode, Json};
use crate::models::{AnalyzeRequest, AnalyzeResponse};
use crate::services::{ai::AiService, git::GitService, analyzer::AnalyzerService};

pub struct AppState {
    pub ai_service: AiService,
}

pub async fn analyze_repo(
    State(state): State<std::sync::Arc<AppState>>,
    Json(payload): Json<AnalyzeRequest>,
) -> Result<Json<AnalyzeResponse>, (StatusCode, String)> {

    let git = GitService::new();
    let analyzer = AnalyzerService::new();

    let repo_path = git.clone_repo(&payload.repo_url)
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    let mut result = analyzer.analyze(&repo_path, &payload.repo_url);

    let context = analyzer.build_ai_context(&result);
    let summary = state.ai_service.summarize(&context)
        .await
        .unwrap_or_else(|_| "No se pudo generar resumen.".to_string());

    result.summary = summary;

    git.cleanup(&repo_path);

    Ok(Json(result))
}