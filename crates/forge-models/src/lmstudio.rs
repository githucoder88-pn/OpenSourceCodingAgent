use async_trait::async_trait;
use crate::{ModelProvider, ModelRequest, ModelResponse, ModelCapabilities, ProviderHealth, ModelError, StreamChunk, Message, Choice, Usage};
pub struct LMStudioProvider { base_url: String }
impl LMStudioProvider { pub fn new(base_url: Option<String>) -> Self { Self { base_url: base_url.unwrap_or_else(|| "http://localhost:1234/v1".into()) } } }
#[async_trait]
impl ModelProvider for LMStudioProvider {
    fn name(&self) -> &str { "lmstudio" }
    fn models(&self) -> Vec<String> { vec!["local-model".into()] }
    fn capabilities(&self, _m: &str) -> ModelCapabilities { ModelCapabilities { text_generation: true, streaming: true, tool_calling: true, vision: false, structured_output: false, reasoning: false, context_window: 32768 } }
    async fn generate(&self, _r: ModelRequest) -> Result<ModelResponse, ModelError> {
        Ok(ModelResponse { id: uuid::Uuid::new_v4().to_string(), model: "local".into(), choices: vec![Choice { index: 0, message: Message { role: "assistant".into(), content: "Hello from LM Studio stub".into(), tool_calls: None, tool_call_id: None }, finish_reason: Some("stop".into()) }], usage: Some(Usage { prompt_tokens: 10, completion_tokens: 20, total_tokens: 30 }) })
    }
    async fn stream(&self, _r: ModelRequest) -> Result<tokio::sync::mpsc::Receiver<Result<StreamChunk, ModelError>>, ModelError> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        tokio::spawn(async move { let _ = tx.send(Ok(StreamChunk { delta: "Hello LM Studio".into(), tool_calls: None, finish_reason: Some("stop".into()) })).await; });
        Ok(rx)
    }
    fn health(&self) -> ProviderHealth { ProviderHealth::Available }
}
