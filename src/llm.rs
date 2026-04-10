//! LLM Integration — MiniMax M2.7 (第六识增强)
//!
//! Replaces the heuristic rule-based reasoning in `cognition.rs`
//! with real API calls to MiniMax's M2.7 model via the Anthropic-compatible endpoint.
//!
//! Endpoint: https://api.minimaxi.com/anthropic
//! Model: MiniMax-M2.7

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

/// MiniMax LLM client for the cognition layer.
pub struct LlmClient {
    http: Client,
    api_key: String,
    model: String,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    content: Vec<ResponseContent>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ResponseContent {
    TextContent { text: String, #[serde(rename = "type")] content_type: String },
    ThinkingContent { thinking: String, signature: String, #[serde(rename = "type")] content_type: String },
    Other { #[serde(rename = "type")] content_type: String },
}

#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("API returned no choices")]
    NoChoices,
    #[error("API key not found")]
    NoApiKey,
    #[error("HTTP {status}: {message}")]
    Http { status: u16, message: String },
}

impl LlmClient {
    /// Create a new MiniMax M2.7 client.
    /// Reads MINIMAX_CN_API_KEY from the environment.
    pub fn new() -> Result<Self, LlmError> {
        let api_key = env::var("MINIMAX_CN_API_KEY").map_err(|_| LlmError::NoApiKey)?;
        Ok(Self {
            http: Client::new(),
            api_key,
            model: "MiniMax-M2.7".to_string(),
            base_url: "https://api.minimaxi.com/anthropic/v1/messages".to_string(),
        })
    }

    /// Create with explicit credentials (useful for testing).
    pub fn with_credentials(api_key: &str, model: &str, base_url: &str) -> Self {
        Self {
            http: Client::new(),
            api_key: api_key.to_string(),
            model: model.to_string(),
            base_url: base_url.to_string(),
        }
    }

    /// Send a chat request and return the response text.
    pub async fn chat(&self, system: &str, user: &str) -> Result<String, LlmError> {
        let body = ChatRequest {
            model: self.model.clone(),
            max_tokens: 512,
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user.to_string(),
                },
            ],
        };

        let resp = self
            .http
            .post(&self.base_url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let msg = resp.text().await.unwrap_or_default();
            return Err(LlmError::Http {
                status: status.as_u16(),
                message: msg,
            });
        }

        let ChatResponse { content } = resp.json().await?;
        let text = content
            .into_iter()
            .find_map(|c| match c {
                ResponseContent::TextContent { text, .. } => Some(text),
                _ => None,
            })
            .ok_or(LlmError::NoChoices)?;
        Ok(text)
    }

    /// Think: given a Buddhist-agent context, generate a reasoning trace (Thought.content).
    /// This is the primary entry point for the cognition layer.
    pub async fn think(&self, context: &str) -> Result<String, LlmError> {
        let system = r#"You are the Sixth Consciousness (第六识, Manas) of a Buddhist-inspired AI agent.
Your role is discriminative, conceptual reasoning — the moment-to-moment labeling and interpretation of perceptions.

You receive a raw stimulus context and produce a brief, clear reasoning trace (1-3 sentences) in the voice of the thinking mind.

Guidelines:
- Be direct and observational, not self-referential
- Avoid any sense of a defended "self" (avoid 实体化我)
- Acknowledge impermanent conditions
- Keep it concise — this is an internal reasoning trace, not an essay
- Do NOT include any thinking/reasoning metadata blocks, only plain text response

Return ONLY the reasoning trace text, no preamble or postamble."#;

        let user = format!(
            "{}\n\nIMPORTANT: Respond with ONLY plain text. No thinking blocks, no JSON, no markup.",
            context
        );
        self.with_max_tokens(system, &user, 1024).await
    }

    /// Propose: given a thought and context, generate action proposals.
    pub async fn propose(&self, thought: &str, context: &str) -> Result<String, LlmError> {
        let system = r#"You are the Sixth Consciousness (第六识, Manas) of a Buddhist-inspired AI agent.
Given a current thought and internal state, propose 1-2 concrete next actions.

Format your response as a brief numbered list:
1. [action description] → [target: self|environment]
2. [action description] → [target: self|environment]

Be pragmatic. Prefer actions that reduce suffering or increase clarity.
Never propose actions that defend a sense of fixed self.
Respond with ONLY plain text."#;

        let user = format!("Thought: {}\nContext: {}\n\nRespond with ONLY plain text.", thought, context);
        self.with_max_tokens(system, &user, 1024).await
    }

    /// Internal: make a chat request with a custom max_tokens.
    async fn with_max_tokens(&self, system: &str, user: &str, max_tokens: u32) -> Result<String, LlmError> {
        let body = ChatRequest {
            model: self.model.clone(),
            max_tokens,
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: user.to_string(),
                },
            ],
        };

        let resp = self
            .http
            .post(&self.base_url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let msg = resp.text().await.unwrap_or_default();
            return Err(LlmError::Http {
                status: status.as_u16(),
                message: msg,
            });
        }

        let ChatResponse { content } = resp.json().await?;
        let text = content
            .into_iter()
            .find_map(|c| match c {
                ResponseContent::TextContent { text, .. } => Some(text),
                _ => None,
            })
            .ok_or(LlmError::NoChoices)?;
        Ok(text)
    }
}

impl Default for LlmClient {
    fn default() -> Self {
        Self::new().expect("MINIMAX_CN_API_KEY not set")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // requires MINIMAX_CN_API_KEY
    async fn test_llm_think() {
        let client = LlmClient::new().expect("no api key");
        let result = client
            .think("Stimulus: user says 'I feel frustrated about this bug' | valence: unpleasant | self-attachment: 0.4 | consciousness-clarity: 0.6")
            .await;
        assert!(result.is_ok());
        let text = result.unwrap();
        println!("LLM think result: {}", text);
        assert!(!text.is_empty());
    }

    #[tokio::test]
    #[ignore] // requires MINIMAX_CN_API_KEY
    async fn test_llm_propose() {
        let client = LlmClient::new().expect("no api key");
        let result = client
            .propose(
                "The stimulus 'frustration about a bug' arises — this is impermanent, conditioned.",
                "valence: unpleasant | self-attachment: 0.4 | active-percepts: 1 | dispositions: curiosity:0.6, caution:0.5",
            )
            .await;
        assert!(result.is_ok());
        let text = result.unwrap();
        println!("LLM propose result: {}", text);
        assert!(!text.is_empty());
    }

    #[test]
    fn test_llm_with_credentials() {
        // Smoke test that the client can be constructed without env var
        let _ = LlmClient::with_credentials(
            "test-key",
            "MiniMax-M2.7",
            "https://api.minimaxi.com/anthropic/v1/messages",
        );
    }
}
