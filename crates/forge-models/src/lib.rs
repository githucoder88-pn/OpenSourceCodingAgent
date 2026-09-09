pub mod providers;
pub mod router;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Provider unavailable: {0}")]
    ProviderUnavailable(String),
    #[error("Model not found: {0}")]
    ModelNotFound(String),
    #[error("Rate limited: {0}")]
    RateLimited(String),
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Streaming failed: {0}")]
    StreamingFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub tools: Option<Vec<ToolDefinition>>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone)]
pub struct StreamChunk {
    pub delta: String,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub finish_reason: Option<String>,
}

#[async_trait]
pub trait ModelProvider: Send + Sync {
    fn name(&self) -> &str;
    fn models(&self) -> Vec<String>;
    fn capabilities(&self, model: &str) -> ModelCapabilities;

    async fn generate(&self, request: ModelRequest) -> Result<ModelResponse, ModelError>;
    async fn stream(&self, request: ModelRequest) -> Result<tokio::sync::mpsc::Receiver<Result<StreamChunk, ModelError>>, ModelError>;

    fn health(&self) -> ProviderHealth;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilities {
    pub text_generation: bool,
    pub streaming: bool,
    pub tool_calling: bool,
    pub vision: bool,
    pub structured_output: bool,
    pub reasoning: bool,
    pub context_window: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProviderHealth {
    Available,
    Degraded,
    RateLimited,
    Offline,
}

pub struct ModelRegistry {
    providers: std::collections::HashMap<String, Box<dyn ModelProvider>>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            providers: std::collections::HashMap::new(),
        }
    }

    pub fn register(&mut self, provider: Box<dyn ModelProvider>) {
        self.providers.insert(provider.name().to_string(), provider);
    }

    pub fn get(&self, name: &str) -> Option<&dyn ModelProvider> {
        self.providers.get(name).map(|p| p.as_ref())
    }

    pub fn list_providers(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    pub fn provider_health(&self) -> std::collections::HashMap<String, ProviderHealth> {
        self.providers.iter().map(|(k, v)| (k.clone(), v.health())).collect()
    }
}

impl Default for ModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Concrete providers are in submodules
pub mod openai;
pub mod anthropic;
pub mod google;
pub mod ollama;
pub mod openrouter;
pub mod azure;
pub mod lmstudio;
pub mod compatible;
