use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

use super::app::App;

/// Render the project list table
pub fn render_project_list(f: &mut Frame, area: Rect, app: &App) {
    let header_cells = ["Name", "Active", "Completed", "Total"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells)
        .style(Style::default())
        .height(1)
        .bottom_margin(1);

    let rows = app.projects.iter().enumerate().map(|(i, project)| {
        let style = if i == app.selected_index {
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let name = if project.project.is_archived() {
            format!("{} [ARCHIVED]", project.project.name)
        } else {
            project.project.name.clone()
        };

        let cells = vec![
            Cell::from(name),
            Cell::from(project.active_todos().to_string()),
            Cell::from(project.completed_todos.to_string()),
            Cell::from(project.total_todos.to_string()),
        ];
        Row::new(cells).style(style).height(1)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(50),
            Constraint::Percentage(15),
            Constraint::Percentage(20),
            Constraint::Percentage(15),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Projects"))
    .row_highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(table, area);
}

/// Render the todo list table
pub fn render_todo_list(f: &mut Frame, area: Rect, app: &App) {
    // Check if we have a description to display
    let has_description = app.current_project
        .as_ref()
        .and_then(|p| p.description.as_ref())
        .map(|d| !d.is_empty())
        .unwrap_or(false);

    // Check if we have an expanded todo with details
    let expanded_todo = app.get_expanded_todo();
    let has_expanded_details = expanded_todo.is_some();

    // Build layout constraints
    let mut constraints = Vec::new();
    if has_description {
        constraints.push(Constraint::Length(3));  // Description area
    }
    constraints.push(Constraint::Min(0));  // Table area
    if has_expanded_details {
        constraints.push(Constraint::Length(5));  // Details area
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    let mut chunk_idx = 0;

    // Render project description if present
    if has_description {
        if let Some(description) = app.current_project.as_ref().and_then(|p| p.description.as_ref()) {
            let desc_widget = Paragraph::new(description.as_str())
                .style(Style::default().fg(Color::DarkGray))
                .wrap(Wrap { trim: true });
            f.render_widget(desc_widget, chunks[chunk_idx]);
        }
        chunk_idx += 1;
    }

    let table_area = chunks[chunk_idx];
    chunk_idx += 1;

    // Render todo table
    let header_cells = ["Status", "Description", "Completed"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells)
        .style(Style::default())
        .height(1)
        .bottom_margin(1);

    let rows = app.todos.iter().enumerate().map(|(i, todo)| {
        let is_expanded = app.expanded_todo_id == Some(todo.id);
        let style = if i == app.selected_index {
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        } else if todo.is_completed() {
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::CROSSED_OUT)
        } else {
            Style::default()
        };

        let status = if todo.is_completed() {
            "✓"
        } else if is_expanded {
            "▼"
        } else {
            " "
        };

        let cells = vec![
            Cell::from(status),
            Cell::from(todo.description.clone()),
            Cell::from(todo.completion_status()),
        ];
        Row::new(cells).style(style).height(1)
    });

    let table = Table::new(
        rows,
        [
            Constraint::Length(8),
            Constraint::Percentage(60),
            Constraint::Percentage(30),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Todos"))
    .row_highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(table, table_area);

    // Render expanded todo details if present
    if has_expanded_details {
        let details_area = chunks[chunk_idx];
        if let Some(todo) = expanded_todo {
            let details_text = todo.details.as_deref().unwrap_or("[no details - press 'e' to add]");
            let details_lines = vec![
                Line::from(Span::styled(
                    format!("Details for: {}", todo.description),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(details_text),
            ];
            let details_widget = Paragraph::new(details_lines)
                .block(Block::default().borders(Borders::ALL))
                .wrap(Wrap { trim: true });
            f.render_widget(details_widget, details_area);
        }
    }
}

/// Render the help screen
pub fn render_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Docket", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" - Project Todo Manager"),
        ]),
        Line::from(""),
        Line::from(Span::styled("Navigation:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from("  j / ↓        Move down"),
        Line::from("  k / ↑        Move up"),
        Line::from("  Enter        Open project / Expand todo / Close help"),
        Line::from("  Esc          Collapse todo / Back to project list"),
        Line::from(""),
        Line::from(Span::styled("Actions:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from("  a            Add new project/todo"),
        Line::from("  d            Delete selected item"),
        Line::from("  Space        Toggle todo completion (todo view only)"),
        Line::from("  r            Rename selected item (project/todo)"),
        Line::from("  e            Edit details (expanded) / Edit project desc"),
        Line::from("  A            Archive/Unarchive project"),
        Line::from("  v            Toggle between active and archived projects"),
        Line::from("  c            Toggle show/hide completed todos"),
        Line::from(""),
        Line::from(Span::styled("Other:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from("  :            Command mode"),
        Line::from("  ?            Show this help"),
        Line::from("  q            Quit"),
        Line::from("  Ctrl+C       Quit"),
        Line::from(""),
        Line::from(Span::styled("Press Esc or Enter to close", Style::default().fg(Color::Green))),
    ];

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .style(Style::default());

    f.render_widget(help, area);
}
