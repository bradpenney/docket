use anyhow::Result;
use crate::core::{models::{Project, ProjectWithStats, Todo}, service::DocketService};

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
    EditDescription,
    EditTodoDetails,
    EditTodo,
    EditProjectName,
}

/// TUI Application state
pub struct App {
    pub service: DocketService,
    pub view_mode: ViewMode,
    pub input_mode: InputMode,
    pub projects: Vec<ProjectWithStats>,
    pub todos: Vec<Todo>,
    pub current_project: Option<Project>,
    pub selected_index: usize,
    pub input_buffer: String,
    pub status_message: Option<String>,
    pub show_completed: bool,
    pub should_quit: bool,
    pub expanded_todo_id: Option<i64>,
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
            current_project: None,
            selected_index: 0,
            input_buffer: String::new(),
            status_message: None,
            show_completed: true,
            should_quit: false,
            expanded_todo_id: None,
        }
    }
    pub fn start_edit_todo(&mut self) {
        if matches!(self.view_mode, ViewMode::TodoList(_)) {
            if let Some(todo) = self.todos.get(self.selected_index) {
                self.input_mode = InputMode::EditTodo;
                self.input_buffer = todo.description.clone();
            }
        }
    }

    /// Start edit project name mode
    pub fn start_edit_project_name(&mut self) {
        let project = match &self.view_mode {
            ViewMode::ProjectList | ViewMode::ArchivedProjects => {
                self.projects.get(self.selected_index).map(|p| &p.project)
            }
            ViewMode::TodoList(_) => self.current_project.as_ref(),
            _ => None,
        };

        if let Some(project) = project {
            self.input_mode = InputMode::EditProjectName;
            self.input_buffer = project.name.clone();
        }
    }

    /// Save the edited project name
    pub async fn save_project_name(&mut self) -> Result<()> {
        let project_id = match &self.view_mode {
            ViewMode::ProjectList | ViewMode::ArchivedProjects => {
                self.projects.get(self.selected_index).map(|p| p.project.id)
            }
            ViewMode::TodoList(_) => self.current_project.as_ref().map(|p| p.id),
            _ => None,
        };

        if let Some(id) = project_id {
            let name = self.input_buffer.trim().to_string();
            if !name.is_empty() {
                match self.service.update_project_name(id, &name).await {
                    Ok(_) => {
                        self.set_status("Project name updated");
                        // Refresh data
                        if let ViewMode::TodoList(_) = self.view_mode {
                            self.current_project = Some(self.service.get_project(id).await?);
                        } else {
                            self.load_projects().await?;
                        }
                    }
                    Err(e) => self.set_status(format!("Error: {}", e)),
                }
            }
        }
        self.cancel_input();
        Ok(())
    }

    /// Save the edited todo description
    pub async fn save_todo(&mut self) -> Result<()> {
        if let ViewMode::TodoList(project_id) = self.view_mode {
            if let Some(todo) = self.todos.get(self.selected_index) {
                let description = self.input_buffer.trim().to_string();
                if !description.is_empty() {
                    match self.service.update_todo(todo.id, &description).await {
                        Ok(_) => {
                             self.load_todos(project_id).await?;
                             self.set_status("Todo updated");
                        }
                        Err(e) => self.set_status(format!("Error: {}", e)),
                    }
                }
            }
        }
        self.cancel_input();
        Ok(())
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
            self.current_project = Some(self.service.get_project(project_id).await?);
            self.view_mode = ViewMode::TodoList(project_id);
            self.selected_index = 0;
            self.load_todos(project_id).await?;
        }
        Ok(())
    }

    /// Go back to project list
    pub async fn back_to_projects(&mut self) -> Result<()> {
        self.view_mode = ViewMode::ProjectList;
        self.current_project = None;
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

    /// Start edit description mode
    pub fn start_edit_description(&mut self) {
        if matches!(self.view_mode, ViewMode::TodoList(_)) {
            self.input_mode = InputMode::EditDescription;
            // Pre-fill with existing description if any
            self.input_buffer = self.current_project
                .as_ref()
                .and_then(|p| p.description.clone())
                .unwrap_or_default();
        }
    }

    /// Save the edited description
    pub async fn save_description(&mut self) -> Result<()> {
        if let ViewMode::TodoList(project_id) = self.view_mode {
            let description = if self.input_buffer.trim().is_empty() {
                None
            } else {
                Some(self.input_buffer.as_str())
            };
            self.service.update_project_description(project_id, description).await?;
            // Reload project to get updated description
            self.current_project = Some(self.service.get_project(project_id).await?);
            self.set_status("Description updated");
        }
        self.cancel_input();
        Ok(())
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

    /// Toggle expansion of the selected todo
    pub fn toggle_todo_expand(&mut self) {
        if let ViewMode::TodoList(_) = self.view_mode {
            if let Some(todo) = self.todos.get(self.selected_index) {
                if self.expanded_todo_id == Some(todo.id) {
                    // Collapse if already expanded
                    self.expanded_todo_id = None;
                } else {
                    // Expand this todo
                    self.expanded_todo_id = Some(todo.id);
                }
            }
        }
    }

    /// Start edit todo details mode
    pub fn start_edit_todo_details(&mut self) {
        if let ViewMode::TodoList(_) = self.view_mode {
            if let Some(todo_id) = self.expanded_todo_id {
                // Find the todo and pre-fill with existing details
                if let Some(todo) = self.todos.iter().find(|t| t.id == todo_id) {
                    self.input_buffer = todo.details.clone().unwrap_or_default();
                    self.input_mode = InputMode::EditTodoDetails;
                }
            }
        }
    }

    /// Save the edited todo details
    pub async fn save_todo_details(&mut self) -> Result<()> {
        if let Some(todo_id) = self.expanded_todo_id {
            let details = if self.input_buffer.trim().is_empty() {
                None
            } else {
                Some(self.input_buffer.as_str())
            };
            self.service.update_todo_details(todo_id, details).await?;
            // Reload todos to get updated details
            if let ViewMode::TodoList(project_id) = self.view_mode {
                self.load_todos(project_id).await?;
            }
            self.set_status("Details updated");
        }
        self.cancel_input();
        Ok(())
    }

    /// Get the currently expanded todo, if any
    pub fn get_expanded_todo(&self) -> Option<&Todo> {
        self.expanded_todo_id
            .and_then(|id| self.todos.iter().find(|t| t.id == id))
    }
}
