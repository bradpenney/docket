use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use std::sync::Arc;

use super::server::AppState;
use crate::core::models::{ProjectWithStats, Todo};

// ===== Request/Response types =====

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub description: String,
}

#[derive(Deserialize)]
pub struct MoveTodoRequest {
    pub direction: String, // "up" or "down"
}

#[derive(Deserialize)]
pub struct ListProjectsQuery {
    #[serde(default)]
    pub include_archived: bool,
}

#[derive(Deserialize)]
pub struct ListTodosQuery {
    #[serde(default = "default_true")]
    pub include_completed: bool,
}

fn default_true() -> bool {
    true
}

// ===== Project handlers =====

/// List all projects
pub async fn list_projects(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListProjectsQuery>,
) -> Result<Json<Vec<ProjectWithStats>>, AppError> {
    let projects = if query.include_archived {
        state.service.list_all_projects().await?
    } else {
        state.service.list_active_projects().await?
    };
    Ok(Json(projects))
}

/// Create a new project
pub async fn create_project(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateProjectRequest>,
) -> Result<impl IntoResponse, AppError> {
    let project = state.service.create_project(&req.name).await?;
    Ok((StatusCode::CREATED, Json(project)))
}

/// Delete a project
pub async fn delete_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    state.service.delete_project(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Archive a project
pub async fn archive_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    state.service.archive_project(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Unarchive a project
pub async fn unarchive_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    state.service.unarchive_project(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ===== Todo handlers =====

/// List todos for a project
pub async fn list_todos(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<i64>,
    Query(query): Query<ListTodosQuery>,
) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = if query.include_completed {
        state.service.list_all_todos(project_id).await?
    } else {
        state.service.list_active_todos(project_id).await?
    };
    Ok(Json(todos))
}

/// Create a new todo
pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Path(project_id): Path<i64>,
    Json(req): Json<CreateTodoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let todo = state.service.create_todo(project_id, &req.description).await?;
    Ok((StatusCode::CREATED, Json(todo)))
}

/// Toggle todo completion
pub async fn toggle_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    state.service.toggle_todo(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Delete a todo
pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    state.service.delete_todo(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Move a todo up or down
pub async fn move_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(req): Json<MoveTodoRequest>,
) -> Result<StatusCode, AppError> {
    match req.direction.as_str() {
        "up" => state.service.move_todo_up(id).await?,
        "down" => state.service.move_todo_down(id).await?,
        _ => return Err(AppError(anyhow::anyhow!("Invalid direction: must be 'up' or 'down'"))),
    }
    Ok(StatusCode::NO_CONTENT)
}

// ===== Error handling =====

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
