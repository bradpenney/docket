# Docket

A project-based todo manager with both TUI (Terminal User Interface) and web interfaces. Built with Rust for performance and reliability.

> **For developers:** See [DEVELOPMENT.md](DEVELOPMENT.md) for development notes, current status, and next steps.

## Features

- **Dual Interface**: Use either the terminal (TUI) or web browser
- **Project Organization**: Group todos by projects
- **Completion Tracking**: Automatically timestamps when todos are completed
- **Archive Support**: Archive completed projects without deleting them
- **Local-First**: SQLite database stored in your config directory
- **K9s-Inspired TUI**: Keyboard-driven navigation with familiar keybindings
- **Cloud-Ready**: Deploy to any cloud platform (Fly.io, Railway, etc.)

## Installation

### From Source

```bash
git clone <repository-url>
cd docket
cargo build --release
```

The binary will be in `target/release/docket`.

## Usage

### TUI Mode (Default)

Simply run the command to launch the terminal interface:

```bash
docket
```

#### TUI Keybindings

**Navigation:**
- `j` / `↓` - Move down
- `k` / `↑` - Move up
- `Enter` - Open selected project
- `Esc` - Go back to project list

**Actions:**
- `a` - Add new project/todo
- `d` - Delete selected item
- `Space` - Toggle todo completion (in todo view)
- `A` - Archive/unarchive project
- `v` - Toggle between active and archived projects
- `c` - Toggle show/hide completed todos
- `?` - Show help
- `q` - Quit

### Web Mode

Start the web server:

```bash
docket server
# or with custom port
docket server --port 8080
# or
docket --port 8080
```

Then open your browser to `http://localhost:3000` (or your custom port).

## Configuration

### Database Location

By default, docket stores its database in:
- Linux: `~/.config/docket/docket.db`
- macOS: `~/Library/Application Support/docket/docket.db`
- Windows: `%APPDATA%\docket\docket.db`

You can override this with the `DOCKET_DB_PATH` environment variable:

```bash
export DOCKET_DB_PATH=/path/to/custom/docket.db
docket
```

### Server Port

The web server defaults to port 3000. You can change this via:
- Command-line flag: `--port 8080`
- Environment variable: `DOCKET_PORT=8080`

## Deployment

### Docker

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/docket /usr/local/bin/docket
EXPOSE 3000
ENV DOCKET_DB_PATH=/data/docket.db
VOLUME /data
CMD ["docket", "server"]
```

### Fly.io

```bash
fly launch
fly deploy
```

### Railway

Connect your repository and Railway will automatically detect and deploy.

## Development

### Prerequisites

- Rust 1.75 or later
- SQLite 3

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Locally

```bash
# TUI mode
cargo run

# Web server mode
cargo run -- server
```

## Architecture

- **Core**: Business logic and data models (shared by TUI and web)
- **Database**: SQLite with sqlx for async operations
- **TUI**: Built with Ratatui (crossterm backend)
- **Web**: Axum web server + Dioxus frontend
- **Single Binary**: One executable for both modes

## Roadmap

- [ ] Multi-user authentication (Clerk integration)
- [ ] Todo priorities and tags
- [ ] Due dates and reminders
- [ ] Export to various formats (JSON, CSV, Markdown)
- [ ] Mobile-responsive web UI improvements
- [ ] Project templates
- [ ] Search and filtering

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
