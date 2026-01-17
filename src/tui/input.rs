use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

use super::app::{App, InputMode, ViewMode};

/// Handle keyboard input events
pub async fn handle_input(app: &mut App) -> Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => handle_normal_mode(app, key).await?,
                InputMode::AddProject => handle_add_project_mode(app, key).await?,
                InputMode::AddTodo => handle_add_todo_mode(app, key).await?,
                InputMode::EditDescription => handle_edit_description_mode(app, key).await?,
                InputMode::EditTodoDetails => handle_edit_todo_details_mode(app, key).await?,
                InputMode::EditTodo => handle_edit_todo_mode(app, key).await?,
                InputMode::EditProjectName => handle_edit_project_name_mode(app, key).await?,
                InputMode::Command => handle_command_mode(app, key).await?,
            }
        }
    }
    Ok(())
}

/// Handle keys in normal navigation mode
async fn handle_normal_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    // Clear any status message on keypress
    app.clear_status();

    match key.code {
        // Quit
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.should_quit = true
        }

        // Navigation
        KeyCode::Char('j') | KeyCode::Down => app.next_item(),
        KeyCode::Char('k') | KeyCode::Up => app.previous_item(),

        // Reordering (only in TodoList view for active todos)
        KeyCode::Char('J') if key.modifiers.contains(KeyModifiers::SHIFT) => {
            if let ViewMode::TodoList(project_id) = &app.view_mode {
                if let Some(todo) = app.todos.get(app.selected_index) {
                    // Only allow reordering active todos
                    if todo.can_reorder() {
                        let todo_id = todo.id;
                        if let Err(e) = app.service.move_todo_down(todo_id).await {
                            app.set_status(format!("Error moving todo: {}", e));
                        } else {
                            // Reload todos to reflect new order
                            app.load_todos(*project_id).await?;
                            // Move selection down to follow the moved todo
                            if app.selected_index < app.todos.len() - 1 {
                                app.selected_index += 1;
                            }
                        }
                    } else {
                        app.set_status("Cannot reorder completed todos");
                    }
                }
            }
        }

        KeyCode::Char('K') if key.modifiers.contains(KeyModifiers::SHIFT) => {
            if let ViewMode::TodoList(project_id) = &app.view_mode {
                if let Some(todo) = app.todos.get(app.selected_index) {
                    // Only allow reordering active todos
                    if todo.can_reorder() {
                        let todo_id = todo.id;
                        if let Err(e) = app.service.move_todo_up(todo_id).await {
                            app.set_status(format!("Error moving todo: {}", e));
                        } else {
                            // Reload todos to reflect new order
                            app.load_todos(*project_id).await?;
                            // Move selection up to follow the moved todo
                            if app.selected_index > 0 {
                                app.selected_index -= 1;
                            }
                        }
                    } else {
                        app.set_status("Cannot reorder completed todos");
                    }
                }
            }
        }

        // Actions based on view
        KeyCode::Enter => {
            match &app.view_mode {
                ViewMode::ProjectList | ViewMode::ArchivedProjects => {
                    app.enter_project().await?;
                }
                ViewMode::TodoList(_) => {
                    app.toggle_todo_expand();
                }
                ViewMode::Help => app.view_mode = ViewMode::ProjectList,
            }
        }

        KeyCode::Esc => {
            match &app.view_mode {
                ViewMode::TodoList(_) => {
                    // If a todo is expanded, collapse it first
                    if app.expanded_todo_id.is_some() {
                        app.expanded_todo_id = None;
                    } else {
                        app.back_to_projects().await?;
                    }
                }
                ViewMode::Help => app.view_mode = ViewMode::ProjectList,
                ViewMode::ArchivedProjects => {
                    app.view_mode = ViewMode::ProjectList;
                    app.load_projects().await?;
                }
                _ => {}
            }
        }

        // Add items
        KeyCode::Char('a') => {
            match &app.view_mode {
                ViewMode::ProjectList => app.start_add_project(),
                ViewMode::TodoList(_) => app.start_add_todo(),
                _ => {}
            }
        }

        // Delete
        KeyCode::Char('d') => {
            match app.view_mode.clone() {
                ViewMode::ProjectList | ViewMode::ArchivedProjects => {
                    if let Some(project) = app.projects.get(app.selected_index) {
                        let project_id = project.project.id;
                        if let Err(e) = app.service.delete_project(project_id).await {
                            app.set_status(format!("Error deleting project: {}", e));
                        } else {
                            app.set_status("Project deleted");
                            app.load_projects().await?;
                        }
                    }
                }
                ViewMode::TodoList(project_id) => {
                    if let Some(todo) = app.todos.get(app.selected_index) {
                        let todo_id = todo.id;
                        if let Err(e) = app.service.delete_todo(todo_id).await {
                            app.set_status(format!("Error deleting todo: {}", e));
                        } else {
                            app.set_status("Todo deleted");
                            app.load_todos(project_id).await?;
                        }
                    }
                }
                _ => {}
            }
        }

        // Toggle completion (todos only)
        KeyCode::Char(' ') => {
            if let ViewMode::TodoList(project_id) = &app.view_mode {
                if let Some(todo) = app.todos.get(app.selected_index) {
                    let todo_id = todo.id;
                    if let Err(e) = app.service.toggle_todo(todo_id).await {
                        app.set_status(format!("Error toggling todo: {}", e));
                    } else {
                        app.load_todos(*project_id).await?;
                    }
                }
            }
        }

        // Archive project
        KeyCode::Char('A') => {
            if matches!(app.view_mode, ViewMode::ProjectList) {
                if let Some(project) = app.projects.get(app.selected_index) {
                    let project_id = project.project.id;
                    if let Err(e) = app.service.archive_project(project_id).await {
                        app.set_status(format!("Error archiving project: {}", e));
                    } else {
                        app.set_status("Project archived");
                        app.load_projects().await?;
                    }
                }
            } else if matches!(app.view_mode, ViewMode::ArchivedProjects) {
                if let Some(project) = app.projects.get(app.selected_index) {
                    let project_id = project.project.id;
                    if let Err(e) = app.service.unarchive_project(project_id).await {
                        app.set_status(format!("Error unarchiving project: {}", e));
                    } else {
                        app.set_status("Project unarchived");
                        app.load_projects().await?;
                    }
                }
            }
        }

        // Toggle archived view
        KeyCode::Char('v') => {
            if matches!(app.view_mode, ViewMode::ProjectList | ViewMode::ArchivedProjects) {
                app.toggle_archived().await?;
            }
        }

        // Toggle completed todos
        KeyCode::Char('c') => {
            app.toggle_completed().await?;
        }

        // Edit: todo details (if expanded) or project description (otherwise)
        KeyCode::Char('e') => {
            if matches!(app.view_mode, ViewMode::TodoList(_)) {
                if app.expanded_todo_id.is_some() {
                    app.start_edit_todo_details();
                } else {
                    app.start_edit_description();
                }
            }
        }

        // Rename item (todo or project)
        KeyCode::Char('r') => {
             match app.view_mode {
                 ViewMode::TodoList(_) => app.start_edit_todo(),
                 ViewMode::ProjectList | ViewMode::ArchivedProjects => app.start_edit_project_name(),
                 _ => {}
             }
        }

        // Help
        KeyCode::Char('?') => app.show_help(),

        // Command mode
        KeyCode::Char(':') => app.start_command_mode(),

        _ => {}
    }
    Ok(())
}

/// Handle keys when adding a project
async fn handle_add_project_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            let name = app.input_buffer.trim().to_string();
            if !name.is_empty() {
                match app.service.create_project(&name).await {
                    Ok(_) => {
                        app.set_status(format!("Project '{}' created", name));
                        app.load_projects().await?;
                    }
                    Err(e) => {
                        app.set_status(format!("Error: {}", e));
                    }
                }
            }
            app.cancel_input();
        }
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        _ => {}
    }
    Ok(())
}

/// Handle keys when adding a todo
async fn handle_add_todo_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            if let ViewMode::TodoList(project_id) = app.view_mode {
                let description = app.input_buffer.trim().to_string();
                if !description.is_empty() {
                    match app.service.create_todo(project_id, &description).await {
                        Ok(_) => {
                            app.set_status("Todo created");
                            app.load_todos(project_id).await?;
                        }
                        Err(e) => {
                            app.set_status(format!("Error: {}", e));
                        }
                    }
                }
            }
            app.cancel_input();
        }
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        _ => {}
    }
    Ok(())
}

/// Handle keys when editing project description
async fn handle_edit_description_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            if let Err(e) = app.save_description().await {
                app.set_status(format!("Error: {}", e));
                app.cancel_input();
            }
        }
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        _ => {}
    }
    Ok(())
}

/// Handle keys when editing todo details
async fn handle_edit_todo_details_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            if let Err(e) = app.save_todo_details().await {
                app.set_status(format!("Error: {}", e));
                app.cancel_input();
            }
        }
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        _ => {}
    }
    Ok(())
}

/// Handle keys when editing todo description
async fn handle_edit_todo_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            if let Err(e) = app.save_todo().await {
                app.set_status(format!("Error: {}", e));
                app.cancel_input();
            }
        }
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        _ => {}
    }
    Ok(())
}

/// Handle keys when editing project name
async fn handle_edit_project_name_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            if let Err(e) = app.save_project_name().await {
                app.set_status(format!("Error: {}", e));
                app.cancel_input();
            }
        }
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        _ => {}
    }
    Ok(())
}

/// Handle keys in command mode
async fn handle_command_mode(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            let command = app.input_buffer.trim().to_lowercase();
            match command.as_str() {
                "q" | "quit" => app.should_quit = true,
                "help" => app.show_help(),
                _ => app.set_status(format!("Unknown command: {}", command)),
            }
            app.cancel_input();
        }
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Char(c) => app.input_buffer.push(c),
        KeyCode::Backspace => {
            app.input_buffer.pop();
        }
        _ => {}
    }
    Ok(())
}
