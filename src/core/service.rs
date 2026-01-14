use anyhow::{bail, Result};

use super::db::Database;
use super::models::{Project, ProjectWithStats, Todo};

/// Business logic service layer
#[derive(Clone)]
pub struct DocketService {
    db: Database,
}

impl DocketService {
    /// Create a new service instance
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // ===== Project Operations =====

    /// Create a new project with validation
    pub async fn create_project(&self, name: &str) -> Result<Project> {
        let name = name.trim();
        if name.is_empty() {
            bail!("Project name cannot be empty");
        }
        if name.len() > 255 {
            bail!("Project name is too long (max 255 characters)");
        }
        self.db.create_project(name).await
    }

    /// List all active projects
    pub async fn list_active_projects(&self) -> Result<Vec<ProjectWithStats>> {
        self.db.list_projects(false).await
    }

    /// List all projects including archived
    pub async fn list_all_projects(&self) -> Result<Vec<ProjectWithStats>> {
        self.db.list_projects(true).await
    }

    /// Archive a project
    pub async fn archive_project(&self, id: i64) -> Result<()> {
        // Verify project exists
        self.db.get_project(id).await?;
        self.db.archive_project(id).await
    }

    /// Unarchive a project
    pub async fn unarchive_project(&self, id: i64) -> Result<()> {
        // Verify project exists
        self.db.get_project(id).await?;
        self.db.unarchive_project(id).await
    }

    /// Delete a project
    pub async fn delete_project(&self, id: i64) -> Result<()> {
        // Verify project exists
        self.db.get_project(id).await?;
        self.db.delete_project(id).await
    }

    // ===== Todo Operations =====

    /// Create a new todo with validation
    pub async fn create_todo(&self, project_id: i64, description: &str) -> Result<Todo> {
        let description = description.trim();
        if description.is_empty() {
            bail!("Todo description cannot be empty");
        }
        if description.len() > 500 {
            bail!("Todo description is too long (max 500 characters)");
        }

        // Verify project exists
        self.db.get_project(project_id).await?;

        self.db.create_todo(project_id, description).await
    }

    /// List all todos for a project (completed and active)
    pub async fn list_all_todos(&self, project_id: i64) -> Result<Vec<Todo>> {
        self.db.list_todos(project_id, true).await
    }

    /// List only active (incomplete) todos for a project
    pub async fn list_active_todos(&self, project_id: i64) -> Result<Vec<Todo>> {
        self.db.list_todos(project_id, false).await
    }

    /// Toggle todo completion status
    pub async fn toggle_todo(&self, id: i64) -> Result<()> {
        // Get the todo to check its completion status
        let todo = self.db.get_todo(id).await?;

        if todo.is_completed() {
            self.db.uncomplete_todo(id).await
        } else {
            self.db.complete_todo(id).await
        }
    }

    /// Delete a todo
    pub async fn delete_todo(&self, id: i64) -> Result<()> {
        self.db.delete_todo(id).await
    }

    /// Move a todo up in the list (decrease position number)
    pub async fn move_todo_up(&self, id: i64) -> Result<()> {
        self.db.reorder_todo(id, -1).await
    }

    /// Move a todo down in the list (increase position number)
    pub async fn move_todo_down(&self, id: i64) -> Result<()> {
        self.db.reorder_todo(id, 1).await
    }
}
