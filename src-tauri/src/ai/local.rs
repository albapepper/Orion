// Ollama via OpenAI-compatible endpoint

use super::{AiProvider, AiResponse, Message};
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct LocalProvider {
    endpoint: String,
    model: String,
    client: reqwest::Client,
}

impl LocalProvider {
    pub fn new(config: &crate::config::AiConfig) -> Self {
        Self {
            endpoint: config.endpoint.clone(),
            model: config.model.clone(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AiProvider for LocalProvider {
    async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        thinking: bool,
    ) -> anyhow::Result<AiResponse> {
        let mut all_messages = vec![json!({ "role": "system", "content": system })];
        for m in messages {
            all_messages.push(json!({ "role": m.role, "content": m.content }));
        }

        let mut body = json!({
            "model": self.model,
            "messages": all_messages,
            "stream": false,
        });

        // Granite 4.2 thinking mode
        if thinking {
            body["thinking"] = json!(true);
        }

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.endpoint))
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;

        if let Some(err) = resp.get("error") {
            anyhow::bail!("AI endpoint error: {err}");
        }

        let message = &resp["choices"][0]["message"];
        let content = message["content"].as_str().unwrap_or("").to_string();
        let thinking_out = message["reasoning"]
            .as_str()
            .or_else(|| message["reasoning_content"].as_str())
            .map(|s| s.to_string());

        Ok(AiResponse {
            content,
            thinking: thinking_out,
        })
    }
}
