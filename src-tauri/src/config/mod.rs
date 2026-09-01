use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub app: AppConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DatabaseConfig {
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiConfig {
    pub provider: String, // "local" or "cloud"
    pub endpoint: String,
    pub model: String,
    pub api_key: String,
    pub thinking: bool,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: "local".into(),
            endpoint: "http://localhost:11434/v1".into(),
            model: "granite4.2:3b".into(),
            api_key: String::new(),
            thinking: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub company_id: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig::default(),
            ai: AiConfig::default(),
            app: AppConfig::default(),
        }
    }
}

impl Config {
    /// Load config from the platform app-data dir. A missing or unreadable
    /// file yields defaults so first-run can proceed to the setup wizard.
    pub fn load() -> Self {
        let path = config_path();
        match std::fs::read_to_string(&path) {
            Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = config_path();
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn is_configured(&self) -> bool {
        !self.database.url.is_empty() && !self.app.company_id.is_empty()
    }
}

pub fn config_path() -> PathBuf {
    dirs::data_dir()
        .expect("No data dir")
        .join("orion")
        .join("config.toml")
}
