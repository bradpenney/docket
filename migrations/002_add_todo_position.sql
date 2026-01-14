-- Migration 002: Add position column for todo ordering
-- Note: This migration is idempotent and can be run multiple times safely

-- Initialize positions for existing active todos that don't have positions yet
-- (ordered by created_at DESC, newest first)
UPDATE todos
SET position = (
    SELECT COUNT(*) + 1
    FROM todos t2
    WHERE t2.project_id = todos.project_id
      AND t2.completed_at IS NULL
      AND t2.created_at > todos.created_at
)
WHERE completed_at IS NULL AND (position = 0 OR position IS NULL);

-- Ensure completed todos have position 0 (they will be sorted separately at the bottom)
UPDATE todos
SET position = 0
WHERE completed_at IS NOT NULL;

-- Create composite index for efficient position-based queries (idempotent)
CREATE INDEX IF NOT EXISTS idx_todos_position ON todos(project_id, completed_at, position);
