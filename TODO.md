# Docket TODO

## Next Session - Priority Tasks

### 🎯 High Priority (Do First)
- [ ] **TEST TUI** - Run `cargo run` and verify all keyboard navigation works
- [ ] Fix compiler warnings (`cargo fix --bin "docket" -p docket`)
- [ ] Test TUI on different terminal sizes
- [ ] Verify database persistence between runs

### 🚀 Deployment & Polish
- [ ] Test Docker build locally
- [ ] Add fly.toml for Fly.io deployment
- [ ] Add Railway configuration
- [ ] Set up GitHub repository
- [ ] Add GitHub Actions CI/CD

### 🎨 UI/UX Enhancements
- [ ] Replace HTML/JS with Dioxus or Leptos frontend
- [ ] Add CSS framework (Tailwind or similar)
- [ ] Improve mobile responsiveness
- [ ] Add dark mode toggle

### ✨ Feature Additions
- [ ] Add Clerk authentication for multi-user
- [ ] Due dates for todos
- [ ] Priority levels (high/medium/low)
- [ ] Tags/labels
- [ ] Search functionality
- [ ] Filter todos by status/date
- [ ] Export to JSON/CSV/Markdown
- [ ] Bulk operations (complete all, delete all)
- [ ] Project templates

### 🧪 Testing & Quality
- [ ] Add unit tests
- [ ] Add integration tests
- [ ] Add API tests
- [ ] Test with large datasets (100+ projects)
- [ ] Performance profiling

### 📚 Documentation
- [ ] Add API documentation
- [ ] Create user guide
- [ ] Add screenshots to README
- [ ] Record demo GIF/video
- [ ] Add contributing guidelines

## Known Issues
- Some unused methods in codebase (non-critical)
- `edition = "2024"` could be "2021" for broader compatibility
- Web UI is basic HTML/JS (works but could be better)

## Ideas for Future
- Mobile app (using Tauri or similar)
- Desktop app (using Tauri)
- Sync between devices
- Recurring todos
- Subtasks/nested todos
- Comments on todos
- File attachments
- Team collaboration features
- Time tracking
- Calendar integration
- Notifications/reminders
