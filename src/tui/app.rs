use anyhow::Result;
use crate::core::{models::{ProjectWithStats, Todo}, service::DocketService};

/// Application view state
#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    ProjectList,
    TodoList(i64), // project_id
    ArchivedProjects,
    Help,
}

/// Input mode
#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Command,
    AddProject,
    AddTodo,
}

/// TUI Application state
pub struct App {
    pub service: DocketService,
    pub view_mode: ViewMode,
    pub input_mode: InputMode,
    pub projects: Vec<ProjectWithStats>,
    pub todos: Vec<Todo>,
    pub selected_index: usize,
    pub input_buffer: String,
    pub status_message: Option<String>,
    pub show_completed: bool,
    pub should_quit: bool,
}

impl App {
    /// Create a new App instance
    pub fn new(service: DocketService) -> Self {
        Self {
            service,
            view_mode: ViewMode::ProjectList,
            input_mode: InputMode::Normal,
            projects: Vec::new(),
            todos: Vec::new(),
            selected_index: 0,
            input_buffer: String::new(),
            status_message: None,
            show_completed: true,
            should_quit: false,
        }
    }

    /// Initialize app - load projects
    pub async fn init(&mut self) -> Result<()> {
        self.load_projects().await?;
        Ok(())
    }

    /// Load projects from database
    pub async fn load_projects(&mut self) -> Result<()> {
        self.projects = match self.view_mode {
            ViewMode::ArchivedProjects => self.service.list_all_projects().await?,
            _ => self.service.list_active_projects().await?,
        };
        // Reset selection if out of bounds
        if self.selected_index >= self.projects.len() && !self.projects.is_empty() {
            self.selected_index = self.projects.len() - 1;
        }
        Ok(())
    }

    /// Load todos for the current project
    pub async fn load_todos(&mut self, project_id: i64) -> Result<()> {
        self.todos = if self.show_completed {
            self.service.list_all_todos(project_id).await?
        } else {
            self.service.list_active_todos(project_id).await?
        };
        // Reset selection if out of bounds
        if self.selected_index >= self.todos.len() && !self.todos.is_empty() {
            self.selected_index = self.todos.len() - 1;
        }
        Ok(())
    }

    /// Navigate to previous item
    pub fn previous_item(&mut self) {
        let len = match &self.view_mode {
            ViewMode::ProjectList | ViewMode::ArchivedProjects => self.projects.len(),
            ViewMode::TodoList(_) => self.todos.len(),
            ViewMode::Help => 0,
        };

        if len > 0 && self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Navigate to next item
    pub fn next_item(&mut self) {
        let len = match &self.view_mode {
            ViewMode::ProjectList | ViewMode::ArchivedProjects => self.projects.len(),
            ViewMode::TodoList(_) => self.todos.len(),
            ViewMode::Help => 0,
        };

        if len > 0 && self.selected_index < len - 1 {
            self.selected_index += 1;
        }
    }

    /// Enter selected project (drill down to todos)
    pub async fn enter_project(&mut self) -> Result<()> {
        if let Some(project) = self.projects.get(self.selected_index) {
            let project_id = project.project.id;
            self.view_mode = ViewMode::TodoList(project_id);
            self.selected_index = 0;
            self.load_todos(project_id).await?;
        }
        Ok(())
    }

    /// Go back to project list
    pub async fn back_to_projects(&mut self) -> Result<()> {
        self.view_mode = ViewMode::ProjectList;
        self.selected_index = 0;
        self.load_projects().await?;
        Ok(())
    }

    /// Toggle between active and archived projects
    pub async fn toggle_archived(&mut self) -> Result<()> {
        self.view_mode = match &self.view_mode {
            ViewMode::ProjectList => ViewMode::ArchivedProjects,
            ViewMode::ArchivedProjects => ViewMode::ProjectList,
            _ => return Ok(()),
        };
        self.selected_index = 0;
        self.load_projects().await?;
        Ok(())
    }

    /// Show help view
    pub fn show_help(&mut self) {
        self.view_mode = ViewMode::Help;
    }

    /// Toggle completed todos visibility
    pub async fn toggle_completed(&mut self) -> Result<()> {
        if let ViewMode::TodoList(project_id) = self.view_mode {
            self.show_completed = !self.show_completed;
            self.selected_index = 0;
            self.load_todos(project_id).await?;
        }
        Ok(())
    }

    /// Set status message
    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    /// Clear status message
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    /// Start add project mode
    pub fn start_add_project(&mut self) {
        self.input_mode = InputMode::AddProject;
        self.input_buffer.clear();
    }

    /// Start add todo mode
    pub fn start_add_todo(&mut self) {
        if matches!(self.view_mode, ViewMode::TodoList(_)) {
            self.input_mode = InputMode::AddTodo;
            self.input_buffer.clear();
        }
    }

    /// Start command mode
    pub fn start_command_mode(&mut self) {
        self.input_mode = InputMode::Command;
        self.input_buffer.clear();
    }

    /// Cancel input mode
    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
    }
}
