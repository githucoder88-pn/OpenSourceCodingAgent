use async_trait::async_trait;
use crate::{ModelProvider, ModelRequest, ModelResponse, ModelCapabilities, ProviderHealth, ModelError, StreamChunk, Message, Choice, Usage};
pub struct AzureProvider { api_key: Option<String>, endpoint: Option<String> }
impl AzureProvider { pub fn new(api_key: Option<String>, endpoint: Option<String>) -> Self { Self { api_key, endpoint } } }
#[async_trait]
impl ModelProvider for AzureProvider {
    fn name(&self) -> &str { "azure" }
    fn models(&self) -> Vec<String> { vec!["gpt-4o".into(), "gpt-4".into()] }
    fn capabilities(&self, _m: &str) -> ModelCapabilities { ModelCapabilities { text_generation: true, streaming: true, tool_calling: true, vision: true, structured_output: true, reasoning: false, context_window: 128000 } }
    async fn generate(&self, _r: ModelRequest) -> Result<ModelResponse, ModelError> {
        if self.api_key.is_none() { return Err(ModelError::AuthFailed("Azure API key not configured".into())); }
        Ok(ModelResponse { id: uuid::Uuid::new_v4().to_string(), model: "azure-gpt-4o".into(), choices: vec![Choice { index: 0, message: Message { role: "assistant".into(), content: "Hello from Azure stub".into(), tool_calls: None, tool_call_id: None }, finish_reason: Some("stop".into()) }], usage: Some(Usage { prompt_tokens: 10, completion_tokens: 20, total_tokens: 30 }) })
    }
    async fn stream(&self, _r: ModelRequest) -> Result<tokio::sync::mpsc::Receiver<Result<StreamChunk, ModelError>>, ModelError> {
        if self.api_key.is_none() { return Err(ModelError::AuthFailed("Azure API key not configured".into())); }
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        tokio::spawn(async move { let _ = tx.send(Ok(StreamChunk { delta: "Hello Azure".into(), tool_calls: None, finish_reason: Some("stop".into()) })).await; });
        Ok(rx)
    }
    fn health(&self) -> ProviderHealth { if self.api_key.is_some() { ProviderHealth::Available } else { ProviderHealth::Offline } }
}
