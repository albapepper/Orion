// Any OpenAI-compatible cloud endpoint (OpenAI, Azure, Anthropic-compatible
// gateways, etc). Identical wire format to the local provider, plus auth.

use super::{AiProvider, AiResponse, Message};
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct CloudProvider {
    endpoint: String,
    model: String,
    api_key: String,
    client: reqwest::Client,
}

impl CloudProvider {
    pub fn new(config: &crate::config::AiConfig) -> Self {
        Self {
            endpoint: config.endpoint.clone(),
            model: config.model.clone(),
            api_key: config.api_key.clone(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AiProvider for CloudProvider {
    async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        _thinking: bool,
    ) -> anyhow::Result<AiResponse> {
        let mut all_messages = vec![json!({ "role": "system", "content": system })];
        for m in messages {
            all_messages.push(json!({ "role": m.role, "content": m.content }));
        }

        let body = json!({
            "model": self.model,
            "messages": all_messages,
            "stream": false,
        });

        let resp = self
            .client
            .post(format!("{}/chat/completions", self.endpoint))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;

        if let Some(err) = resp.get("error") {
            anyhow::bail!("AI endpoint error: {err}");
        }

        let content = resp["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(AiResponse {
            content,
            thinking: None,
        })
    }
}
