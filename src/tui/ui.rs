use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
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
}

/// Render the header
fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let title = match &app.view_mode {
        ViewMode::ProjectList => "Docket - Projects",
        ViewMode::TodoList(_) => "Docket - Todos",
        ViewMode::ArchivedProjects => "Docket - Archived Projects",
        ViewMode::Help => "Docket - Help",
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
                        "j/k: navigate | Enter: open | a: add | d: delete | A: archive | v: toggle archived | ?: help | q: quit"
                    }
                    ViewMode::TodoList(_) => {
                        "j/k: navigate | Space: toggle | a: add | d: delete | c: toggle completed | Esc: back | q: quit"
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
