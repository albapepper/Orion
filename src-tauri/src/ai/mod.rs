pub mod cloud;
pub mod local;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String, // "system" | "user" | "assistant"
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiResponse {
    pub content: String,
    pub thinking: Option<String>, // chain-of-thought if enabled
}

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn complete(
        &self,
        system: &str,         // screen context injected here
        messages: &[Message], // conversation history
        thinking: bool,       // enable reasoning mode
    ) -> anyhow::Result<AiResponse>;
}

// Factory: build the right provider from config
pub fn build_provider(config: &crate::config::AiConfig) -> Arc<dyn AiProvider> {
    match config.provider.as_str() {
        "cloud" => Arc::new(cloud::CloudProvider::new(config)),
        _ => Arc::new(local::LocalProvider::new(config)),
    }
}
