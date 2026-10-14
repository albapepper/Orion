pub mod ai_commands;
pub mod aus_commands;
pub mod condition_commands;
pub mod config_commands;
pub mod contacts_commands;
pub mod document_commands;
pub mod export_commands;
pub mod loan_commands;
pub mod pricing_commands;
pub mod task_commands;
pub mod user_commands;

use crate::config::Config;
use crate::harness::PluginRegistry;
use crate::plugins;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

/// Shared state across all Tauri commands.
pub struct AppState {
    pub config: RwLock<Config>,
    pub db: RwLock<Option<PgPool>>,
    pub ai: RwLock<Arc<crate::ai::Provider>>,
    pub current_user_id: Mutex<Option<Uuid>>,
    pub plugins: PluginRegistry,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let ai = crate::ai::build_provider(&config.ai);
        let mut plugins = PluginRegistry::new(
            std::sync::Arc::new(crate::harness::GenericPlugin),
        );
        plugins::register(&mut plugins);
        Self {
            config: RwLock::new(config),
            db: RwLock::new(None),
            ai: RwLock::new(ai),
            current_user_id: Mutex::new(None),
            plugins,
        }
    }

    /// Clone the pool out of the lock, or explain that setup is incomplete.
    pub async fn pool(&self) -> Result<PgPool, String> {
        self.db
            .read()
            .await
            .clone()
            .ok_or_else(|| "Database not configured — run the setup wizard".to_string())
    }

    pub async fn provider(&self) -> Arc<crate::ai::Provider> {
        self.ai.read().await.clone()
    }

    pub async fn company_id(&self) -> Result<Uuid, String> {
        let cfg = self.config.read().await;
        Uuid::parse_str(&cfg.app.company_id)
            .map_err(|_| "No company configured — run the setup wizard".to_string())
    }
}
