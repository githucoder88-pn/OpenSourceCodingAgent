use async_trait::async_trait;
use crate::{ModelProvider, ModelRequest, ModelResponse, ModelCapabilities, ProviderHealth, ModelError, StreamChunk, Message, Choice, Usage};

pub struct AnthropicProvider { api_key: Option<String> }
impl AnthropicProvider { pub fn new(api_key: Option<String>) -> Self { Self { api_key } } }

#[async_trait]
impl ModelProvider for AnthropicProvider {
    fn name(&self) -> &str { "anthropic" }
    fn models(&self) -> Vec<String> { vec!["claude-3-5-sonnet-20241022".into(), "claude-3-opus-20240229".into(), "claude-3-haiku-20240307".into()] }
    fn capabilities(&self, _m: &str) -> ModelCapabilities {
        ModelCapabilities { text_generation: true, streaming: true, tool_calling: true, vision: true, structured_output: true, reasoning: true, context_window: 200000 }
    }
    async fn generate(&self, _r: ModelRequest) -> Result<ModelResponse, ModelError> {
        if self.api_key.is_none() { return Err(ModelError::AuthFailed("Anthropic API key not configured".into())); }
        Ok(ModelResponse { id: uuid::Uuid::new_v4().to_string(), model: "claude-3-5-sonnet".into(), choices: vec![Choice { index: 0, message: Message { role: "assistant".into(), content: "Hello from Anthropic stub".into(), tool_calls: None, tool_call_id: None }, finish_reason: Some("stop".into()) }], usage: Some(Usage { prompt_tokens: 10, completion_tokens: 20, total_tokens: 30 }) })
    }
    async fn stream(&self, _r: ModelRequest) -> Result<tokio::sync::mpsc::Receiver<Result<StreamChunk, ModelError>>, ModelError> {
        if self.api_key.is_none() { return Err(ModelError::AuthFailed("Anthropic API key not configured".into())); }
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        tokio::spawn(async move { let _ = tx.send(Ok(StreamChunk { delta: "Hello ".into(), tool_calls: None, finish_reason: None })).await; let _ = tx.send(Ok(StreamChunk { delta: "Anthropic".into(), tool_calls: None, finish_reason: Some("stop".into()) })).await; });
        Ok(rx)
    }
    fn health(&self) -> ProviderHealth { if self.api_key.is_some() { ProviderHealth::Available } else { ProviderHealth::Offline } }
}
