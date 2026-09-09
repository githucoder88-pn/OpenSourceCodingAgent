use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub provider: String,
    pub name: String,
    pub context_window: u32,
    pub max_output: Option<u32>,
    pub capabilities: ModelCapabilities,
    pub pricing: Option<ModelPricing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilities {
    pub text_generation: bool,
    pub streaming: bool,
    pub tool_calling: bool,
    pub vision: bool,
    pub structured_output: bool,
    pub reasoning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    pub input_per_mtok: Option<f64>,
    pub output_per_mtok: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderHealth {
    Available,
    Degraded,
    RateLimited,
    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderStatus {
    pub provider: String,
    pub health: ProviderHealth,
    pub latency_ms: Option<u64>,
    pub error_rate: f32,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingStrategy {
    Manual,
    Priority,
    LowestCost,
    LowestLatency,
    HighestQuality,
    Adaptive,
    LocalFirst,
    CloudFirst,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRequest {
    pub task_complexity: u8,
    pub required_capabilities: ModelCapabilities,
    pub context_size: u32,
    pub latency_requirement: Option<u64>,
    pub budget: Option<f64>,
    pub preferred_provider: Option<String>,
    pub strategy: RoutingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {
    pub provider: String,
    pub model: String,
    pub fallback_chain: Vec<(String, String)>,
    pub reasoning: String,
    pub estimated_cost: Option<f64>,
}
