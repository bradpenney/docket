# Docket - Development Guidelines

## Design Philosophy

**Docket is meant to be minimal and unopinionated.**

### Core Principles

1. **Minimal by Default** - The application should do one thing well: manage todos organized by project. No bloat, no unnecessary features.

2. **All Metadata is Optional** - If a user wants to add metadata (due dates, priorities, tags, etc.), they can. But:
   - The program must work perfectly without any metadata
   - The UI must look clean and complete without metadata fields
   - Empty/null metadata should never appear as "None" or placeholder text
   - Features should gracefully degrade when metadata is absent

3. **Unopinionated** - Don't force workflows on users:
   - No mandatory fields beyond the bare minimum (project name, todo description)
   - No enforced hierarchies or categorization schemes
   - Let users organize their own way

4. **Progressive Disclosure** - Advanced features should be discoverable but not in-your-face:
   - Basic usage should be obvious
   - Power features available for those who want them
   - Don't clutter the UI with rarely-used options

## Implementation Guidelines

### Adding New Features

When adding optional metadata or features:

```rust
// GOOD: Optional field with sensible default behavior
pub struct Todo {
    pub description: String,
    pub due_date: Option<DateTime<Utc>>,  // Optional - UI works fine without it
}

// BAD: Required field that adds friction
pub struct Todo {
    pub description: String,
    pub priority: Priority,  // Forces user to choose even when they don't care
}
```

### UI Guidelines

- Don't show empty metadata fields (no "Due: None" or "Priority: -")
- Completed items should be visually distinct but not distracting
- Keep the default view clean - filters/options can reveal more
- Keyboard-first, but mouse should work too

### Database Guidelines

- New columns for optional features should have `DEFAULT NULL` or sensible defaults
- Schema changes must not break existing databases
- Migrations should be idempotent where possible
