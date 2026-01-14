use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

mod config;
mod core;
mod tui;
mod web;

use config::Config;
use core::{db::Database, service::DocketService};
use tui::{App, input, ui};

/// Docket - Project-based todo manager
#[derive(Parser)]
#[command(name = "docket")]
#[command(about = "Project-based todo manager with TUI and web interfaces", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Port for web server (default: 3000)
    #[arg(short, long)]
    port: Option<u16>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the web server
    Server {
        /// Port to bind to
        #[arg(short, long)]
        port: Option<u16>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Load configuration
    let config = Config::load()?;

    // Initialize database
    let db = Database::new(&config.database_path).await?;
    let service = DocketService::new(db);

    match cli.command {
        Some(Commands::Server { port }) => {
            // Run web server
            let port = port
                .or_else(|| std::env::var("DOCKET_PORT").ok().and_then(|p| p.parse().ok()))
                .unwrap_or(config.server_port);

            web::start_server(service, port).await?;
        }
        None if cli.port.is_some() => {
            // Port specified without subcommand, run web server
            let port = cli.port.unwrap();
            web::start_server(service, port).await?;
        }
        None => {
            // Run TUI
            run_tui(service).await?;
        }
    }

    Ok(())
}

/// Run the TUI application
async fn run_tui(service: DocketService) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(service);
    app.init().await?;

    // Main loop
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

/// Main TUI event loop
async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, app))?;

        input::handle_input(app).await?;

        if app.should_quit {
            break;
        }
    }

    Ok(())
}
