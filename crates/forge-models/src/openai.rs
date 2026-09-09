use async_trait::async_trait;
use crate::{ModelProvider, ModelRequest, ModelResponse, ModelCapabilities, ProviderHealth, ModelError, StreamChunk, Message, Choice, Usage};

pub struct OpenAIProvider {
    api_key: Option<String>,
    base_url: String,
}

impl OpenAIProvider {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key, base_url: "https://api.openai.com/v1".to_string() }
    }
    pub fn with_base_url(api_key: Option<String>, base_url: String) -> Self {
        Self { api_key, base_url }
    }
}

#[async_trait]
impl ModelProvider for OpenAIProvider {
    fn name(&self) -> &str { "openai" }
    fn models(&self) -> Vec<String> {
        vec!["gpt-4o".into(), "gpt-4o-mini".into(), "o1".into(), "o1-mini".into()]
    }
    fn capabilities(&self, model: &str) -> ModelCapabilities {
        ModelCapabilities {
            text_generation: true,
            streaming: true,
            tool_calling: !model.contains("o1"),
            vision: model.contains("4o"),
            structured_output: true,
            reasoning: model.contains("o1"),
            context_window: if model.contains("o1") { 200000 } else { 128000 },
        }
    }
    async fn generate(&self, _request: ModelRequest) -> Result<ModelResponse, ModelError> {
        if self.api_key.is_none() {
            return Err(ModelError::AuthFailed("OpenAI API key not configured".into()));
        }
        // Real implementation would call OpenAI API
        Ok(ModelResponse {
            id: uuid::Uuid::new_v4().to_string(),
            model: "gpt-4o".into(),
            choices: vec![Choice {
                index: 0,
                message: Message { role: "assistant".into(), content: "Hello from OpenAI provider stub".into(), tool_calls: None, tool_call_id: None },
                finish_reason: Some("stop".into()),
            }],
            usage: Some(Usage { prompt_tokens: 10, completion_tokens: 20, total_tokens: 30 }),
        })
    }
    async fn stream(&self, _request: ModelRequest) -> Result<tokio::sync::mpsc::Receiver<Result<StreamChunk, ModelError>>, ModelError> {
        if self.api_key.is_none() {
            return Err(ModelError::AuthFailed("OpenAI API key not configured".into()));
        }
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        tokio::spawn(async move {
            let _ = tx.send(Ok(StreamChunk { delta: "Hello ".into(), tool_calls: None, finish_reason: None })).await;
            let _ = tx.send(Ok(StreamChunk { delta: "world".into(), tool_calls: None, finish_reason: Some("stop".into()) })).await;
        });
        Ok(rx)
    }
    fn health(&self) -> ProviderHealth {
        if self.api_key.is_some() { ProviderHealth::Available } else { ProviderHealth::Offline }
    }
}
