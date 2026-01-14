use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a project containing todos
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub archived_at: Option<DateTime<Utc>>,
}

impl Project {
    /// Check if the project is archived
    pub fn is_archived(&self) -> bool {
        self.archived_at.is_some()
    }
}

/// Represents a todo item within a project
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub project_id: i64,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub position: i64,
}

impl Todo {
    /// Check if the todo is completed
    pub fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    /// Get a formatted completion date or "Pending"
    pub fn completion_status(&self) -> String {
        match &self.completed_at {
            Some(date) => date.format("%Y-%m-%d %H:%M").to_string(),
            None => "Pending".to_string(),
        }
    }

    /// Check if this todo can be reordered (only active todos can be reordered)
    pub fn can_reorder(&self) -> bool {
        self.completed_at.is_none()
    }
}

/// Project with todo statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectWithStats {
    #[serde(flatten)]
    pub project: Project,
    pub total_todos: i64,
    pub completed_todos: i64,
}

impl ProjectWithStats {
    /// Get the number of active (incomplete) todos
    pub fn active_todos(&self) -> i64 {
        self.total_todos - self.completed_todos
    }
}
