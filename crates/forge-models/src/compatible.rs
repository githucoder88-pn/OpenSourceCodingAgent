use async_trait::async_trait;
use crate::{ModelProvider, ModelRequest, ModelResponse, ModelCapabilities, ProviderHealth, ModelError, StreamChunk, Message, Choice, Usage};
pub struct CompatibleProvider { name: String, base_url: String, api_key: Option<String> }
impl CompatibleProvider { pub fn new(name: String, base_url: String, api_key: Option<String>) -> Self { Self { name, base_url, api_key } } }
#[async_trait]
impl ModelProvider for CompatibleProvider {
    fn name(&self) -> &str { &self.name }
    fn models(&self) -> Vec<String> { vec![format!("{}-model", self.name)] }
    fn capabilities(&self, _m: &str) -> ModelCapabilities { ModelCapabilities { text_generation: true, streaming: true, tool_calling: true, vision: false, structured_output: true, reasoning: false, context_window: 128000 } }
    async fn generate(&self, _r: ModelRequest) -> Result<ModelResponse, ModelError> {
        Ok(ModelResponse { id: uuid::Uuid::new_v4().to_string(), model: self.name.clone(), choices: vec![Choice { index: 0, message: Message { role: "assistant".into(), content: format!("Hello from {} compatible provider", self.name), tool_calls: None, tool_call_id: None }, finish_reason: Some("stop".into()) }], usage: Some(Usage { prompt_tokens: 10, completion_tokens: 20, total_tokens: 30 }) })
    }
    async fn stream(&self, _r: ModelRequest) -> Result<tokio::sync::mpsc::Receiver<Result<StreamChunk, ModelError>>, ModelError> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let name = self.name.clone();
        tokio::spawn(async move { let _ = tx.send(Ok(StreamChunk { delta: format!("Hello from {} ", name), tool_calls: None, finish_reason: None })).await; let _ = tx.send(Ok(StreamChunk { delta: "compatible".into(), tool_calls: None, finish_reason: Some("stop".into()) })).await; });
        Ok(rx)
    }
    fn health(&self) -> ProviderHealth { ProviderHealth::Available }
}
