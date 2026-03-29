use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;
use crate::models::{CompareRequest, CompareResponse};
use crate::routes::analyze::AppState;
use crate::services::{git::GitService, analyzer::AnalyzerService};

pub async fn compare_repos(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CompareRequest>,
) -> Result<Json<CompareResponse>, (StatusCode, Json<crate::routes::analyze::ApiError>)> {

    let git = GitService::new();
    let analyzer = AnalyzerService::new();

    git.validate_url(&payload.repo_a)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(crate::routes::analyze::ApiError { error: e })))?;
    git.validate_url(&payload.repo_b)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(crate::routes::analyze::ApiError { error: e })))?;

    let path_a = git.clone_repo(&payload.repo_a)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(crate::routes::analyze::ApiError { error: e })))?;

    let path_b = git.clone_repo(&payload.repo_b)
        .map_err(|e| {
            git.cleanup(&path_a);
            (StatusCode::BAD_REQUEST, Json(crate::routes::analyze::ApiError { error: e }))
        })?;

    let last_commit_a = git.last_commit_days(&path_a);
    let last_commit_b = git.last_commit_days(&path_b);
    let contributors_a = git.count_contributors(&path_a);
    let contributors_b = git.count_contributors(&path_b);

    let mut result_a = analyzer.analyze(&path_a, &payload.repo_a, last_commit_a);
    let mut result_b = analyzer.analyze(&path_b, &payload.repo_b, last_commit_b);

    result_a.contributors = contributors_a;
    result_b.contributors = contributors_b;

    let readme_a = analyzer.read_readme(&path_a);
    let readme_b = analyzer.read_readme(&path_b);
    let context_a = analyzer.build_ai_context(&result_a, readme_a);
    let context_b = analyzer.build_ai_context(&result_b, readme_b);

    git.cleanup(&path_a);
    git.cleanup(&path_b);

    let comparison = state.ai_service.compare(&context_a, &context_b)
        .await
        .map_err(|e| {
            tracing::warn!("Gemini compare falló: {}", e);
            None::<()>
        })
        .ok();

    Ok(Json(CompareResponse {
        repo_a: result_a,
        repo_b: result_b,
        comparison,
    }))
}