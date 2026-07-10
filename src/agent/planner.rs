//! LLM planning loop — proposes preprocessing improvements.

use crate::agent::Message;

/// Call OpenRouter with conversation history and return the assistant's reply.
pub async fn call_llm(api_key: &str, system: &str, messages: &[Message]) -> Result<String, String> {
    if api_key.is_empty() {
        return Err("API key not set. Configure it in Settings > LLM > API Key.".into());
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

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

    let status = resp.status();
    let data: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse response (status {}): {}", status, e))?;

    // Check for API-level errors
    if let Some(err) = data.get("error") {
        let msg = err["message"].as_str().unwrap_or("unknown error");
        let code = err["code"].as_str().unwrap_or("");
        return Err(format!("API error ({}): {}", code, msg));
    }

    // Extract content — can be null when model uses tool calls or refuses
    let content = data["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .unwrap_or_default();

    if content.is_empty() {
        // Check for refusal or finish reason
        let finish = data["choices"][0]["finish_reason"]
            .as_str()
            .unwrap_or("unknown");
        let refusal = data["choices"][0]["message"]["refusal"]
            .as_str()
            .unwrap_or("");
        if !refusal.is_empty() {
            return Err(format!("Model refused: {}", refusal));
        }
        // Content might be in a different format
        let full = serde_json::to_string_pretty(&data["choices"][0]["message"])
            .unwrap_or_else(|_| "could not serialize".into());
        return Err(format!(
            "Empty response (finish_reason: {}). Raw message: {}",
            finish, full
        ));
    }

    Ok(content)
}
