use serde::{Deserialize, Serialize};

const ANTHROPIC_BASE_URL: &str = "https://api.minimaxi.com/anthropic";

#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<AnthropicMessage>,
}

#[derive(Debug, Serialize)]
struct AnthropicMessage {
    role: String,
    content: Vec<AnthropicContent>,
}

#[derive(Debug, Serialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicResponseBlock>,
}

#[derive(Debug, Deserialize)]
struct AnthropicResponseBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: Option<String>,
    thinking: Option<String>,
}

pub struct AnthropicClient {
    api_key: String,
    client: reqwest::Client,
}

impl AnthropicClient {
    pub fn new(api_key: &str) -> Result<Self, String> {
        Ok(Self {
            api_key: api_key.to_string(),
            client: reqwest::Client::new(),
        })
    }

    pub async fn chat(&self, system_prompt: &str, user_message: &str) -> Result<String, String> {
        let req = AnthropicRequest {
            model: "MiniMax-M2.7".to_string(),
            max_tokens: 4000,
            system: system_prompt.to_string(),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: vec![AnthropicContent {
                    content_type: "text".to_string(),
                    text: user_message.to_string(),
                }],
            }],
        };

        let response = self.client
            .post(format!("{}/v1/messages", ANTHROPIC_BASE_URL))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .body(serde_json::to_string(&req).map_err(|e| e.to_string())?)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("API error {}: {}", status, body));
        }

        let resp: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))?;

        // Extract text content (skip thinking blocks)
        for block in &resp.content {
            if block.block_type == "text" {
                if let Some(ref text) = block.text {
                    return Ok(text.clone());
                }
            }
        }

        Err("No text content in response".to_string())
    }
}
