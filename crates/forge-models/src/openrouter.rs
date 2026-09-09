use async_trait::async_trait;
use crate::{ModelProvider, ModelRequest, ModelResponse, ModelCapabilities, ProviderHealth, ModelError, StreamChunk, Message, Choice, Usage};
pub struct OpenRouterProvider { api_key: Option<String> }
impl OpenRouterProvider { pub fn new(api_key: Option<String>) -> Self { Self { api_key } } }
#[async_trait]
impl ModelProvider for OpenRouterProvider {
    fn name(&self) -> &str { "openrouter" }
    fn models(&self) -> Vec<String> { vec!["anthropic/claude-3.5-sonnet".into(), "openai/gpt-4o".into(), "google/gemini-pro".into()] }
    fn capabilities(&self, _m: &str) -> ModelCapabilities { ModelCapabilities { text_generation: true, streaming: true, tool_calling: true, vision: true, structured_output: true, reasoning: true, context_window: 128000 } }
    async fn generate(&self, _r: ModelRequest) -> Result<ModelResponse, ModelError> {
        if self.api_key.is_none() { return Err(ModelError::AuthFailed("OpenRouter API key not configured".into())); }
        Ok(ModelResponse { id: uuid::Uuid::new_v4().to_string(), model: "openrouter".into(), choices: vec![Choice { index: 0, message: Message { role: "assistant".into(), content: "Hello from OpenRouter stub".into(), tool_calls: None, tool_call_id: None }, finish_reason: Some("stop".into()) }], usage: Some(Usage { prompt_tokens: 10, completion_tokens: 20, total_tokens: 30 }) })
    }
    async fn stream(&self, _r: ModelRequest) -> Result<tokio::sync::mpsc::Receiver<Result<StreamChunk, ModelError>>, ModelError> {
        if self.api_key.is_none() { return Err(ModelError::AuthFailed("OpenRouter API key not configured".into())); }
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        tokio::spawn(async move { let _ = tx.send(Ok(StreamChunk { delta: "Hello OpenRouter".into(), tool_calls: None, finish_reason: Some("stop".into()) })).await; });
        Ok(rx)
    }
    fn health(&self) -> ProviderHealth { if self.api_key.is_some() { ProviderHealth::Available } else { ProviderHealth::Offline } }
}
