use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiResponse {
    pub content: String,
    pub thinking: Option<String>,
}

pub struct Provider {
    endpoint: String,
    model: String,
    api_key: Option<String>,
    client: reqwest::Client,
    thinking: bool,
}

impl Provider {
    pub fn new(config: &crate::config::AiConfig) -> Self {
        let cloud = config.provider == "cloud";
        Self {
            endpoint: config.endpoint.clone(),
            model: config.model.clone(),
            api_key: if cloud { Some(config.api_key.clone()) } else { None },
            client: reqwest::Client::new(),
            thinking: !cloud,
        }
    }

    pub async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        thinking: bool,
    ) -> anyhow::Result<AiResponse> {
        let mut all_messages = vec![serde_json::json!({ "role": "system", "content": system })];
        for m in messages {
            all_messages.push(serde_json::json!({ "role": m.role, "content": m.content }));
        }

        let mut body = serde_json::json!({
            "model": self.model,
            "messages": all_messages,
            "stream": false,
        });

        if self.thinking && thinking {
            body["thinking"] = serde_json::json!(true);
        }

        let mut req = self
            .client
            .post(format!("{}/chat/completions", self.endpoint))
            .json(&body);
        if let Some(key) = &self.api_key {
            req = req.header("Authorization", format!("Bearer {key}"));
        }

        let resp = req.send().await?.json::<serde_json::Value>().await?;

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
            thinking: if self.thinking { thinking_out } else { None },
        })
    }
}

pub fn build_provider(config: &crate::config::AiConfig) -> Arc<Provider> {
    Arc::new(Provider::new(config))
}
