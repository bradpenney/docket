use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub database_path: PathBuf,
    pub server_port: u16,
}

impl Config {
    /// Load configuration from environment and defaults
    pub fn load() -> Result<Self> {
        let database_path = Self::get_database_path()?;
        let server_port = std::env::var("DOCKET_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        Ok(Self {
            database_path,
            server_port,
        })
    }

    /// Get the database file path, creating parent directories if needed
    fn get_database_path() -> Result<PathBuf> {
        // Check for environment override first
        if let Ok(path) = std::env::var("DOCKET_DB_PATH") {
            return Ok(PathBuf::from(path));
        }

        // Use XDG config directory
        let proj_dirs = ProjectDirs::from("com", "docket", "docket")
            .context("Failed to determine project directories")?;

        let config_dir = proj_dirs.config_dir();
        std::fs::create_dir_all(config_dir)
            .context("Failed to create config directory")?;

        Ok(config_dir.join("docket.db"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_load() {
        let config = Config::load().expect("Failed to load config");
        assert!(config.database_path.to_string_lossy().contains("docket.db"));
        assert!(config.server_port > 0);
    }
}
