use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use super::app::{App, InputMode, ViewMode};
use super::views;

/// Render the TUI
pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Input/Status bar
        ])
        .split(f.area());

    render_header(f, chunks[0], app);
    render_content(f, chunks[1], app);
    render_footer(f, chunks[2], app);

    // Render modal overlay for editing
    match app.input_mode {
        InputMode::EditDescription => render_description_modal(f, app),
        InputMode::EditTodoDetails => render_todo_details_modal(f, app),
        InputMode::EditTodo => render_todo_modal(f, app),
        InputMode::EditProjectName => render_project_name_modal(f, app),
        _ => {}
    }
}
// ...
/// Render the project name edit modal
fn render_project_name_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 10, f.area());

    // Clear the area behind the modal
    f.render_widget(Clear, area);

    let content = format!(
        "{}\n\n[Enter] Save  [Esc] Cancel",
        if app.input_buffer.is_empty() {
            "(empty)"
        } else {
            &app.input_buffer
        }
    );

    let modal = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Edit Project Name")
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(modal, area);
}

/// Render the header
fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let title = match &app.view_mode {
        ViewMode::ProjectList => "Docket - Projects".to_string(),
        ViewMode::TodoList(_) => {
            if let Some(project) = &app.current_project {
                format!("Docket - {}", project.name)
            } else {
                "Docket - Todos".to_string()
            }
        }
        ViewMode::ArchivedProjects => "Docket - Archived Projects".to_string(),
        ViewMode::Help => "Docket - Help".to_string(),
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(header, area);
}

/// Render main content area
fn render_content(f: &mut Frame, area: Rect, app: &App) {
    match &app.view_mode {
        ViewMode::ProjectList | ViewMode::ArchivedProjects => {
            views::render_project_list(f, area, app)
        }
        ViewMode::TodoList(_) => views::render_todo_list(f, area, app),
        ViewMode::Help => views::render_help(f, area),
    }
}

/// Render the footer (status bar and input)
fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    let (content, style) = match &app.input_mode {
        InputMode::Normal => {
            if let Some(msg) = &app.status_message {
                (msg.clone(), Style::default().fg(Color::Yellow))
            } else {
                let hints = match &app.view_mode {
                    ViewMode::ProjectList => {
                        "j/k: navigate | Enter: open | a: add | d: delete | r: rename | A: archive | v: toggle archived | ?: help | q: quit"
                    }
                    ViewMode::TodoList(_) => {
                        if app.expanded_todo_id.is_some() {
                            "Enter/Esc: collapse | e: edit details | Space: toggle | d: delete"
                        } else {
                            "j/k: navigate | Enter: expand | Space: toggle | a: add | d: delete | r: rename | e: edit desc | Esc: back"
                        }
                    }
                    ViewMode::ArchivedProjects => {
                        "j/k: navigate | Enter: open | d: delete | A: unarchive | v: back to active | ?: help | q: quit"
                    }
                    ViewMode::Help => "Press Esc or Enter to close help",
                };
                (hints.to_string(), Style::default().fg(Color::DarkGray))
            }
        }
        InputMode::AddProject => (
            format!("Add Project: {}", app.input_buffer),
            Style::default().fg(Color::Green),
        ),
        InputMode::AddTodo => (
            format!("Add Todo: {}", app.input_buffer),
            Style::default().fg(Color::Green),
        ),
        InputMode::EditDescription => (
            "Editing description... (Enter to save, Esc to cancel)".to_string(),
            Style::default().fg(Color::Green),
        ),
        InputMode::EditTodoDetails => (
            "Editing details... (Enter to save, Esc to cancel)".to_string(),
            Style::default().fg(Color::Green),
        ),
        InputMode::EditTodo => (
            format!("Edit Todo: {}", app.input_buffer),
            Style::default().fg(Color::Green),
        ),
        InputMode::EditProjectName => (
            format!("Edit Project Name: {}", app.input_buffer),
            Style::default().fg(Color::Green),
        ),
        InputMode::Command => (
            format!(":{}", app.input_buffer),
            Style::default().fg(Color::Yellow),
        ),
    };

    let footer = Paragraph::new(content)
        .style(style)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}

/// Calculate a centered rectangle of given percentage dimensions
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Render the description edit modal
fn render_description_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 30, f.area());

    // Clear the area behind the modal
    f.render_widget(Clear, area);

    let content = format!(
        "{}\n\n[Enter] Save  [Esc] Cancel",
        if app.input_buffer.is_empty() {
            "(empty - press Enter to clear description)"
        } else {
            &app.input_buffer
        }
    );

    let modal = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Edit Project Description")
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(modal, area);
}

/// Render the todo details edit modal
fn render_todo_details_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 30, f.area());

    // Clear the area behind the modal
    f.render_widget(Clear, area);

    let content = format!(
        "{}\n\n[Enter] Save  [Esc] Cancel",
        if app.input_buffer.is_empty() {
            "(empty - press Enter to clear details)"
        } else {
            &app.input_buffer
        }
    );

    let modal = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Edit Todo Details")
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(modal, area);
}

/// Render the todo edit modal
fn render_todo_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 10, f.area()); // Smaller height for single line description

    // Clear the area behind the modal
    f.render_widget(Clear, area);

    let content = format!(
        "{}\n\n[Enter] Save  [Esc] Cancel",
        if app.input_buffer.is_empty() {
            "(empty)"
        } else {
            &app.input_buffer
        }
    );

    let modal = Paragraph::new(content)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Edit Todo Description")
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(modal, area);
}
