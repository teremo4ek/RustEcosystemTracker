use serde::{Deserialize, Serialize};

use crate::config::LlmConfig;

// --- OpenAI-compatible request/response structs ---

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

/// Call the LLM API and return a short summary of the given text.
pub async fn summarize(config: &LlmConfig, text: &str) -> anyhow::Result<String> {
    let api_key = std::env::var(&config.api_key_env)
        .map_err(|_| anyhow::anyhow!("env var {} not set", config.api_key_env))?;

    let base_url = match config.provider.as_str() {
        "z-ai" => "https://api.z.ai/api/paas/v4",
        other => anyhow::bail!("unknown LLM provider: {}", other),
    };

    let url = format!("{}/chat/completions", base_url);

    let prompt = format!(
        "Summarize the following in 2-3 concise sentences. Focus on the key points:\n\n{}",
        text
    );

    let request = ChatRequest {
        model: config.model.clone(),
        messages: vec![ChatMessage {
            role: "user".into(),
            content: prompt,
        }],
        max_tokens: config.max_tokens,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        anyhow::bail!("LLM API error ({}): {}", status, body);
    }

    let response: ChatResponse = serde_json::from_str(&body)?;

    let content = response
        .choices
        .into_iter()
        .next()
        .map(|c| c.message.content)
        .unwrap_or_default();

    Ok(content.trim().to_string())
}
