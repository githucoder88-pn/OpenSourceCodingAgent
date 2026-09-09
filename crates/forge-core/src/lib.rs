pub mod agent;
pub mod orchestration;
pub mod scheduler;
pub mod task_graph;
pub mod teams;
pub mod messaging;
pub mod context;
pub mod memory;
pub mod workspace;
pub mod permissions;
pub mod checkpoints;
pub mod sessions;
pub mod events;

pub use agent::*;
pub use sessions::*;
pub use events::*;
pub use task_graph::*;
pub use teams::*;
pub use context::*;
pub use memory::*;
pub use permissions::*;
pub use checkpoints::*;
pub use workspace::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeConfig {
    pub project: ProjectConfig,
    pub orchestration: OrchestrationConfig,
    pub models: ModelsConfig,
    pub routing: RoutingConfig,
    pub autonomy: AutonomyConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationConfig {
    pub mode: String,
    pub max_agents: usize,
    pub max_parallel_tasks: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsConfig {
    pub primary: forge_protocol::ModelConfig,
    pub fast: Option<forge_protocol::ModelConfig>,
    pub reviewer: Option<forge_protocol::ModelConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    pub strategy: String,
    pub fallback: bool,
    pub cost_aware: bool,
    pub latency_aware: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomyConfig {
    pub default: String,
    pub approval_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub low_resource_mode: bool,
    pub max_parallel_agents: usize,
    pub max_context_cache_mb: usize,
    pub background_indexing: bool,
    pub local_embeddings: bool,
    pub verbose_events: bool,
}

impl Default for ForgeConfig {
    fn default() -> Self {
        Self {
            project: ProjectConfig { name: "my-project".into(), path: ".".into() },
            orchestration: OrchestrationConfig { mode: "supervisor".into(), max_agents: 6, max_parallel_tasks: 4 },
            models: ModelsConfig {
                primary: forge_protocol::ModelConfig { provider: "openai".into(), model: "gpt-4o".into(), api_key_env: Some("OPENAI_API_KEY".into()), base_url: None, context_window: Some(128000), max_tokens: None, temperature: Some(0.7) },
                fast: Some(forge_protocol::ModelConfig { provider: "ollama".into(), model: "llama3.1".into(), api_key_env: None, base_url: Some("http://localhost:11434".into()), context_window: Some(32768), max_tokens: None, temperature: Some(0.7) }),
                reviewer: Some(forge_protocol::ModelConfig { provider: "anthropic".into(), model: "claude-3-5-sonnet-20241022".into(), api_key_env: Some("ANTHROPIC_API_KEY".into()), base_url: None, context_window: Some(200000), max_tokens: None, temperature: Some(0.3) }),
            },
            routing: RoutingConfig { strategy: "adaptive".into(), fallback: true, cost_aware: true, latency_aware: true },
            autonomy: AutonomyConfig { default: "workspace-write".into(), approval_policy: "on-risky-commands".into() },
            performance: PerformanceConfig { low_resource_mode: false, max_parallel_agents: 6, max_context_cache_mb: 512, background_indexing: true, local_embeddings: false, verbose_events: true },
        }
    }
}
