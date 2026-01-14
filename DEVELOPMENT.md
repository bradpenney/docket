# Docket Development Notes

## Session: 2026-01-12

### Project Status: ✅ MVP Complete

Docket is a fully functional project-based todo manager with TUI and web interfaces.

## What Was Accomplished

### Core Implementation
- ✅ SQLite database layer with migrations
- ✅ Project and Todo models with completion timestamps
- ✅ Service layer with business logic validation
- ✅ Configuration management (XDG-compliant paths)
- ✅ Database stored in `~/.config/docket/docket.db`

### TUI (Terminal User Interface)
- ✅ Built with Ratatui (K9s-inspired design)
- ✅ Keyboard-driven navigation (j/k, arrows, vim-style)
- ✅ Project list view with statistics
- ✅ Todo list view with completion toggles
- ✅ Help screen with keybindings
- ✅ Status bar with contextual hints
- ⚠️ **NOT YET TESTED** - needs interactive terminal testing

### Web Interface
- ✅ Axum REST API (fully tested and working)
- ✅ Simple HTML/JavaScript frontend
- ✅ All CRUD operations functional
- ✅ Todo toggle with timestamps verified
- ✅ Projects can be created, archived, deleted
- ✅ Clean, responsive design

### Deployment
- ✅ Dockerfile for containerization
- ✅ docker-compose.yml for easy deployment
- ✅ Health checks configured
- ✅ Volume persistence setup
- ✅ README with usage instructions

## Project Structure

```
source_code/docket/
├── src/
│   ├── main.rs              # CLI entry point (TUI or web server)
│   ├── config.rs            # XDG config paths, settings
│   ├── core/
│   │   ├── models.rs        # Project, Todo structs
│   │   ├── db.rs            # SQLite layer
│   │   └── service.rs       # Business logic
│   ├── tui/
│   │   ├── app.rs           # TUI state management
│   │   ├── ui.rs            # Rendering logic
│   │   ├── input.rs         # Keyboard handling
│   │   └── views.rs         # View components
│   └── web/
│       ├── server.rs        # Axum server setup
│       └── api.rs           # REST endpoints
├── static/
│   └── index.html           # Web frontend
├── migrations/
│   └── 001_init.sql         # Database schema
├── Dockerfile
├── docker-compose.yml
├── README.md
└── LICENSE (MIT)
```

## Current State

### Working ✅
- Compilation successful (some warnings about unused methods)
- Web server starts and serves API correctly
- All REST endpoints tested and working
- Database migrations with `IF NOT EXISTS` guards
- Todo completion timestamps work correctly
- HTML interface serves and loads

### Not Yet Tested ⚠️
- TUI functionality (needs interactive terminal session)
- TUI keyboard navigation and commands
- TUI rendering on different terminal sizes

### Known Issues
- Web UI uses vanilla HTML/JS instead of Dioxus (simplified for MVP)
- Some compiler warnings about unused helper methods (non-critical)
- `edition = "2024"` in Cargo.toml (works but could be 2021)

## Next Session - Start Here

### Option 1: Test the TUI 🎯 **RECOMMENDED FIRST**
```bash
cd /home/brad/notes/source_code/docket
cargo run
```

**Test checklist:**
- [ ] TUI starts without errors
- [ ] Can navigate with j/k keys
- [ ] Can add a project (press 'a')
- [ ] Can drill into project (press Enter)
- [ ] Can add todos
- [ ] Can toggle completion (Space)
- [ ] Can archive project (Shift+A)
- [ ] Can delete items (press 'd')
- [ ] Help screen works (press '?')
- [ ] Can quit cleanly (press 'q')

### Option 2: Enhance Web UI
Replace vanilla HTML/JS with a proper Rust frontend:
- Add Dioxus/Leptos for full Rust stack
- Or add React/Vue/Svelte for modern JS framework
- Current HTML is in `static/index.html`

### Option 3: Add Features
Potential enhancements:
- [ ] Clerk authentication for multi-user deployments
- [ ] Due dates for todos
- [ ] Priority levels (high, medium, low)
- [ ] Tags/labels for organization
- [ ] Search and filtering
- [ ] Export to JSON/CSV/Markdown
- [ ] Project templates
- [ ] Bulk operations

### Option 4: Deploy to Cloud
- [ ] Test Docker build
- [ ] Deploy to Fly.io
- [ ] Deploy to Railway
- [ ] Set up CI/CD with GitHub Actions

## How to Run

**TUI Mode:**
```bash
cargo run
# or
./target/debug/docket
```

**Web Server:**
```bash
cargo run -- server
# or with custom port
cargo run -- server --port 8080
```

**Docker:**
```bash
docker-compose up -d
```

## TUI Keybindings Reference

### Navigation
- `j` / `↓` - Move down
- `k` / `↑` - Move up
- `Enter` - Open selected project
- `Esc` - Back to project list

### Actions
- `a` - Add project/todo
- `d` - Delete item
- `Space` - Toggle todo completion
- `A` - Archive/unarchive project
- `v` - Toggle archived projects view
- `c` - Toggle show completed todos

### Other
- `?` - Show help
- `:` - Command mode
- `q` - Quit

## API Endpoints

All working and tested:

```bash
# Projects
GET    /api/projects
POST   /api/projects
DELETE /api/projects/:id
PATCH  /api/projects/:id/archive
PATCH  /api/projects/:id/unarchive

# Todos
GET    /api/projects/:id/todos
POST   /api/projects/:id/todos
PATCH  /api/todos/:id/toggle
DELETE /api/todos/:id
```

## Configuration

### Database Location
Default: `~/.config/docket/docket.db`

Override:
```bash
export DOCKET_DB_PATH=/custom/path/docket.db
```

### Server Port
Default: `3000`

Override:
```bash
export DOCKET_PORT=8080
# or
docket server --port 8080
```

## Build & Test Commands

```bash
# Build
cargo build

# Build release
cargo build --release

# Run TUI
cargo run

# Run web server
cargo run -- server

# Run tests (when added)
cargo test

# Fix warnings
cargo fix --bin "docket" -p docket

# Check code
cargo check
```

## Questions for Next Session

1. **TUI Testing** - Did the terminal interface work as expected?
2. **Web UI Enhancement** - Should we replace HTML with Dioxus/Leptos?
3. **Authentication** - Ready to add Clerk for multi-user?
4. **Deployment** - Want to deploy to Fly.io or Railway?
5. **Features** - Which features are most important?

## Git Status

**Note:** Project is not yet in git. To initialize:

```bash
cd /home/brad/notes/source_code/docket
git init
git add .
git commit -m "Initial commit: Docket MVP with TUI and web interface"
```

To make it public:
```bash
# Create repo on GitHub first, then:
git remote add origin <your-github-url>
git push -u origin main
```

---

**Last Updated:** 2026-01-12
**Status:** MVP Complete, TUI untested
**Next Step:** Test TUI functionality
