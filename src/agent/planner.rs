//! LLM planning loop — proposes preprocessing improvements.

use crate::agent::Message;

/// Call OpenRouter with conversation history and return the assistant's reply.
pub async fn call_llm(api_key: &str, system: &str, messages: &[Message]) -> Result<String, String> {
    if api_key.is_empty() {
        return Err("API key not set. Configure it in Settings > LLM > API Key.".into());
    }

    let client = reqwest::Client::new();

    let mut body_messages = Vec::new();
    body_messages.push(serde_json::json!({
        "role": "system",
        "content": system
    }));
    for msg in messages {
        body_messages.push(serde_json::json!({
            "role": if msg.role == "You" { "user" } else { "assistant" },
            "content": msg.content
        }));
    }

    let body = serde_json::json!({
        "model": "gpt-4o",
        "messages": body_messages,
        "temperature": 0.7,
        "max_tokens": 2048
    });

    let resp = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let data: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let content = data["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| "No content in response".to_string())?
        .to_string();

    Ok(content)
}
