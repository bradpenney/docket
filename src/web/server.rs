use anyhow::Result;
use axum::{
    Router,
    routing::{get, post, delete, patch},
    response::Html,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::core::service::DocketService;
use super::api;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub service: DocketService,
}

/// Serve the web UI
async fn serve_ui() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

/// Start the web server
pub async fn start_server(service: DocketService, port: u16) -> Result<()> {
    let state = Arc::new(AppState { service });

    let app = Router::new()
        // API routes
        .route("/api/projects", get(api::list_projects))
        .route("/api/projects", post(api::create_project))
        .route("/api/projects/:id", get(api::get_project))
        .route("/api/projects/:id", delete(api::delete_project))
        .route("/api/projects/:id", patch(api::update_project_name))
        .route("/api/projects/:id/archive", patch(api::archive_project))
        .route("/api/projects/:id/unarchive", patch(api::unarchive_project))
        .route("/api/projects/:id/description", patch(api::update_project_description))
        .route("/api/projects/:id/todos", get(api::list_todos))
        .route("/api/projects/:id/todos", post(api::create_todo))
        .route("/api/todos/:id", get(api::get_todo))
        .route("/api/todos/:id", delete(api::delete_todo))
        .route("/api/todos/:id", patch(api::update_todo))
        .route("/api/todos/:id/toggle", patch(api::toggle_todo))
        .route("/api/todos/:id/move", patch(api::move_todo))
        .route("/api/todos/:id/details", patch(api::update_todo_details))
        // Serve web UI
        .route("/", get(serve_ui))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    println!("🚀 Docket web server running on http://{}", addr);
    println!("   API: http://{}/ api/*", addr);
    println!("Press Ctrl+C to stop");

    axum::serve(listener, app).await?;

    Ok(())
}
