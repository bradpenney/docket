# Docket Quick Start

## 🚀 Getting Started in 30 Seconds

### First Time Setup
```bash
cd /home/brad/notes/source_code/docket
cargo build
```

### Try the TUI
```bash
cargo run
```

**Quick TUI Test:**
1. Press `a` to add a project
2. Type a name and press Enter
3. Press Enter to open the project
4. Press `a` to add a todo
5. Press Space to toggle completion
6. Press `?` for help
7. Press `q` to quit

### Try the Web Interface
```bash
cargo run -- server
```
Open: http://localhost:3000

### Run with Docker
```bash
docker-compose up -d
```

## 📁 Important Files

- `DEVELOPMENT.md` - Full development notes and status
- `TODO.md` - Task list for future work
- `README.md` - User-facing documentation
- `src/main.rs` - Entry point
- `static/index.html` - Web UI

## 🔑 Key Commands

**Build:**
```bash
cargo build          # Debug build
cargo build --release # Release build
```

**Run:**
```bash
cargo run            # TUI mode
cargo run -- server  # Web server
cargo run -- --help  # Show help
```

**Database Location:**
```bash
~/.config/docket/docket.db
```

**Clean Database (for testing):**
```bash
rm ~/.config/docket/docket.db
cargo run  # Will recreate
```

## 🎯 Next Session Start Here

1. Test TUI: `cargo run`
2. Check `DEVELOPMENT.md` for full status
3. See `TODO.md` for task list
4. Read this file's parent README.md for user docs

## 📝 Notes from Last Session

- ✅ MVP is complete and functional
- ✅ Web API tested and working
- ⚠️ TUI not yet tested interactively
- Web UI is basic but functional (can be enhanced)
- Ready for deployment

---
**Project Location:** `/home/brad/notes/source_code/docket/`
**Status:** Ready for TUI testing
**Last Updated:** 2026-01-12
