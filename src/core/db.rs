use anyhow::{Context, Result};
use chrono::Utc;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::Row;
use std::path::Path;
use std::str::FromStr;

use super::models::{Project, ProjectWithStats, Todo};

/// Database connection pool wrapper
#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Initialize database connection and run migrations
    pub async fn new(database_path: &Path) -> Result<Self> {
        // Create connection options
        let options = SqliteConnectOptions::from_str(
            &format!("sqlite://{}", database_path.display())
        )?
        .create_if_missing(true);

        // Create connection pool
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .context("Failed to connect to database")?;

        // Run schema initialization (idempotent - uses CREATE TABLE IF NOT EXISTS)
        sqlx::query(include_str!("../../migrations/001_init.sql"))
            .execute(&pool)
            .await
            .context("Failed to initialize database schema")?;

        Ok(Self { pool })
    }

    // ===== Project Operations =====

    /// Create a new project
    pub async fn create_project(&self, name: &str, description: Option<&str>) -> Result<Project> {
        let result = sqlx::query(
            "INSERT INTO projects (name, description) VALUES (?, ?) RETURNING id, name, description, created_at, archived_at"
        )
        .bind(name)
        .bind(description)
        .fetch_one(&self.pool)
        .await
        .context("Failed to create project")?;

        Ok(Project {
            id: result.get("id"),
            name: result.get("name"),
            description: result.get("description"),
            created_at: result.get("created_at"),
            archived_at: result.get("archived_at"),
        })
    }

    /// List all projects with statistics
    pub async fn list_projects(&self, include_archived: bool) -> Result<Vec<ProjectWithStats>> {
        let query = if include_archived {
            r#"
            SELECT
                p.id,
                p.name,
                p.description,
                p.created_at,
                p.archived_at,
                COUNT(t.id) as total_todos,
                COUNT(CASE WHEN t.completed_at IS NOT NULL THEN 1 END) as completed_todos
            FROM projects p
            LEFT JOIN todos t ON p.id = t.project_id
            GROUP BY p.id
            ORDER BY p.created_at DESC
            "#
        } else {
            r#"
            SELECT
                p.id,
                p.name,
                p.description,
                p.created_at,
                p.archived_at,
                COUNT(t.id) as total_todos,
                COUNT(CASE WHEN t.completed_at IS NOT NULL THEN 1 END) as completed_todos
            FROM projects p
            LEFT JOIN todos t ON p.id = t.project_id
            WHERE p.archived_at IS NULL
            GROUP BY p.id
            ORDER BY p.created_at DESC
            "#
        };

        let rows = sqlx::query(query)
            .fetch_all(&self.pool)
            .await
            .context("Failed to list projects")?;

        let projects = rows
            .iter()
            .map(|row| {
                Ok(ProjectWithStats {
                    project: Project {
                        id: row.get("id"),
                        name: row.get("name"),
                        description: row.get("description"),
                        created_at: row.get("created_at"),
                        archived_at: row.get("archived_at"),
                    },
                    total_todos: row.get("total_todos"),
                    completed_todos: row.get("completed_todos"),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(projects)
    }

    /// Get a project by ID
    pub async fn get_project(&self, id: i64) -> Result<Project> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .context("Failed to get project")
    }

    /// Archive a project
    pub async fn archive_project(&self, id: i64) -> Result<()> {
        sqlx::query("UPDATE projects SET archived_at = ? WHERE id = ?")
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to archive project")?;
        Ok(())
    }

    /// Unarchive a project
    pub async fn unarchive_project(&self, id: i64) -> Result<()> {
        sqlx::query("UPDATE projects SET archived_at = NULL WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to unarchive project")?;
        Ok(())
    }

    /// Delete a project
    pub async fn delete_project(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to delete project")?;
        Ok(())
    }

    /// Update a project's description
    pub async fn update_project_description(&self, id: i64, description: Option<&str>) -> Result<()> {
        sqlx::query("UPDATE projects SET description = ? WHERE id = ?")
            .bind(description)
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to update project description")?;
        Ok(())
    }

    /// Update a project's name
    pub async fn update_project_name(&self, id: i64, name: &str) -> Result<()> {
        sqlx::query("UPDATE projects SET name = ? WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to update project name")?;
        Ok(())
    }

    // ===== Todo Operations =====

    /// Get a todo by ID
    pub async fn get_todo(&self, id: i64) -> Result<Todo> {
        sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .context("Failed to get todo")
    }

    /// Create a new todo
    pub async fn create_todo(&self, project_id: i64, description: &str) -> Result<Todo> {
        // Get the max position for this project's active todos
        let max_position: i64 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(position), 0) FROM todos WHERE project_id = ? AND completed_at IS NULL"
        )
        .bind(project_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to get max position")?;

        // New todo gets max_position + 1
        let new_position = max_position + 1;

        let result = sqlx::query(
            "INSERT INTO todos (project_id, description, position) VALUES (?, ?, ?) RETURNING id, project_id, description, details, created_at, completed_at, position"
        )
        .bind(project_id)
        .bind(description)
        .bind(new_position)
        .fetch_one(&self.pool)
        .await
        .context("Failed to create todo")?;

        Ok(Todo {
            id: result.get("id"),
            project_id: result.get("project_id"),
            description: result.get("description"),
            details: result.get("details"),
            created_at: result.get("created_at"),
            completed_at: result.get("completed_at"),
            position: result.get("position"),
        })
    }

    /// List todos for a project
    pub async fn list_todos(&self, project_id: i64, include_completed: bool) -> Result<Vec<Todo>> {
        let query = if include_completed {
            // Active todos first (ordered by position), then completed todos (ordered by completion date DESC)
            r#"SELECT * FROM todos
               WHERE project_id = ?
               ORDER BY
                 CASE WHEN completed_at IS NULL THEN 0 ELSE 1 END,
                 CASE WHEN completed_at IS NULL THEN position ELSE 0 END,
                 completed_at DESC"#
        } else {
            // Only active todos, ordered by position
            "SELECT * FROM todos WHERE project_id = ? AND completed_at IS NULL ORDER BY position ASC"
        };

        sqlx::query_as::<_, Todo>(query)
            .bind(project_id)
            .fetch_all(&self.pool)
            .await
            .context("Failed to list todos")
    }

    /// Complete a todo
    pub async fn complete_todo(&self, id: i64) -> Result<()> {
        // Set completed_at and reset position to 0 (completed todos don't need position)
        sqlx::query("UPDATE todos SET completed_at = ?, position = 0 WHERE id = ?")
            .bind(Utc::now())
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to complete todo")?;
        Ok(())
    }

    /// Uncomplete a todo
    pub async fn uncomplete_todo(&self, id: i64) -> Result<()> {
        // First, get the project_id for this todo
        let todo = self.get_todo(id).await?;

        // Get the max position for active todos in this project
        let max_position: i64 = sqlx::query_scalar(
            "SELECT COALESCE(MAX(position), 0) FROM todos WHERE project_id = ? AND completed_at IS NULL"
        )
        .bind(todo.project_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to get max position")?;

        // Assign new position at the end
        let new_position = max_position + 1;

        sqlx::query("UPDATE todos SET completed_at = NULL, position = ? WHERE id = ?")
            .bind(new_position)
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to uncomplete todo")?;
        Ok(())
    }

    /// Delete a todo
    pub async fn delete_todo(&self, id: i64) -> Result<()> {
        sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to delete todo")?;
        Ok(())
    }

    /// Update a todo's details
    pub async fn update_todo_details(&self, id: i64, details: Option<&str>) -> Result<()> {
        sqlx::query("UPDATE todos SET details = ? WHERE id = ?")
            .bind(details)
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to update todo details")?;
        Ok(())
    }

    /// Update a todo's description
    pub async fn update_todo(&self, id: i64, description: &str) -> Result<()> {
        sqlx::query("UPDATE todos SET description = ? WHERE id = ?")
            .bind(description)
            .bind(id)
            .execute(&self.pool)
            .await
            .context("Failed to update todo description")?;
        Ok(())
    }

    /// Reorder a todo by swapping positions with an adjacent todo
    /// direction: -1 for up (decrease position), +1 for down (increase position)
    pub async fn reorder_todo(&self, todo_id: i64, direction: i8) -> Result<()> {
        // Get the current todo
        let current_todo = self.get_todo(todo_id).await?;

        // Can only reorder active todos
        if current_todo.completed_at.is_some() {
            anyhow::bail!("Cannot reorder completed todos");
        }

        // Find the todo to swap with
        let swap_query = if direction < 0 {
            // Moving up: find the todo with the next lower position
            r#"SELECT id, position FROM todos
               WHERE project_id = ?
                 AND completed_at IS NULL
                 AND position < ?
               ORDER BY position DESC
               LIMIT 1"#
        } else {
            // Moving down: find the todo with the next higher position
            r#"SELECT id, position FROM todos
               WHERE project_id = ?
                 AND completed_at IS NULL
                 AND position > ?
               ORDER BY position ASC
               LIMIT 1"#
        };

        let swap_result = sqlx::query(swap_query)
            .bind(current_todo.project_id)
            .bind(current_todo.position)
            .fetch_optional(&self.pool)
            .await
            .context("Failed to find swap target")?;

        // If no todo found to swap with, we're at the boundary
        let Some(swap_row) = swap_result else {
            return Ok(()); // Silently ignore boundary cases
        };

        let swap_id: i64 = swap_row.get("id");
        let swap_position: i64 = swap_row.get("position");

        // Perform the swap in a transaction
        let mut tx = self.pool.begin().await?;

        // Temporarily set one to a negative value to avoid unique constraint issues
        sqlx::query("UPDATE todos SET position = -1 WHERE id = ?")
            .bind(current_todo.id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE todos SET position = ? WHERE id = ?")
            .bind(current_todo.position)
            .bind(swap_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query("UPDATE todos SET position = ? WHERE id = ?")
            .bind(swap_position)
            .bind(current_todo.id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
