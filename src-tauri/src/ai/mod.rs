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
        let tools = serde_json::json!([{
            "type": "function",
            "function": {
                "name": "get_handbook",
                "description": "Look up the product/screen handbook rules.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "product": { "type": "string" },
                        "screen": { "type": "string" }
                    },
                    "required": ["product", "screen"]
                }
            }
        }]);

        let mut msgs: Vec<serde_json::Value> = vec![serde_json::json!({ "role": "system", "content": system })];
        for m in messages {
            msgs.push(serde_json::json!({ "role": m.role, "content": m.content }));
        }

        for _ in 0..4 {
            let mut body = serde_json::json!({
                "model": self.model,
                "messages": msgs,
                "tools": tools,
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

            let message = resp["choices"][0]["message"].clone();
            let tool_calls = message.get("tool_calls").and_then(|v| v.as_array()).cloned();

            if let Some(calls) = tool_calls {
                msgs.push(message.clone());
                for call in calls {
                    let func = &call["function"];
                    let name = func["name"].as_str().unwrap_or("");
                    let args: serde_json::Value = func["arguments"]
                        .as_str()
                        .and_then(|s| serde_json::from_str(s).ok())
                        .unwrap_or_else(|| serde_json::json!({}));
                    let result = if name == "get_handbook" {
                        crate::handbooks::lookup(
                            args["product"].as_str().unwrap_or("30yr_fixed"),
                            args["screen"].as_str().unwrap_or(""),
                        )
                        .to_string()
                    } else {
                        "unknown tool".to_string()
                    };
                    msgs.push(serde_json::json!({
                        "role": "tool",
                        "tool_call_id": call["id"],
                        "content": result,
                    }));
                }
                continue;
            }

            let content = message["content"].as_str().unwrap_or("").to_string();
            let thinking_out = message["reasoning"]
                .as_str()
                .or_else(|| message["reasoning_content"].as_str())
                .map(|s| s.to_string());

            return Ok(AiResponse {
                content,
                thinking: if self.thinking { thinking_out } else { None },
            });
        }

        anyhow::bail!("tool call loop exceeded")
    }
}

pub fn build_provider(config: &crate::config::AiConfig) -> Arc<Provider> {
    Arc::new(Provider::new(config))
}