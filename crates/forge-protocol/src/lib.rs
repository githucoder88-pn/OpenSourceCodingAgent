pub mod agents;
pub mod events;
pub mod models;
pub mod rpc;
pub mod tasks;
pub mod teams;
pub mod tools;

pub use agents::*;
pub use events::*;
pub use models::*;
pub use rpc::*;
pub use tasks::*;
pub use teams::*;
pub use tools::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub type AgentId = Uuid;
pub type SessionId = Uuid;
pub type TaskId = Uuid;
pub type TeamId = Uuid;
pub type MessageId = Uuid;
pub type ToolCallId = Uuid;
pub type WorkspaceId = Uuid;
pub type ModelId = String;
pub type CheckpointId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentState {
    Created,
    Idle,
    Planning,
    Executing,
    WaitingForTool,
    WaitingForAgent,
    Blocked,
    Reviewing,
    Paused,
    Failed,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentIdentity {
    pub id: AgentId,
    pub name: String,
    pub role: AgentRole,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentRole {
    Architect,
    FrontendEngineer,
    BackendEngineer,
    DatabaseEngineer,
    DevOpsEngineer,
    SecurityEngineer,
    QAEngineer,
    Reviewer,
    Researcher,
    TechnicalWriter,
    ProductManager,
    EngineeringManager,
    Director,
    Custom(String),
}

impl std::fmt::Display for AgentRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Architect => write!(f, "Architect"),
            Self::FrontendEngineer => write!(f, "Frontend Engineer"),
            Self::BackendEngineer => write!(f, "Backend Engineer"),
            Self::DatabaseEngineer => write!(f, "Database Engineer"),
            Self::DevOpsEngineer => write!(f, "DevOps Engineer"),
            Self::SecurityEngineer => write!(f, "Security Engineer"),
            Self::QAEngineer => write!(f, "QA Engineer"),
            Self::Reviewer => write!(f, "Reviewer"),
            Self::Researcher => write!(f, "Researcher"),
            Self::TechnicalWriter => write!(f, "Technical Writer"),
            Self::ProductManager => write!(f, "Product Manager"),
            Self::EngineeringManager => write!(f, "Engineering Manager"),
            Self::Director => write!(f, "Director"),
            Self::Custom(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    pub tokens_used: u64,
    pub requests: u64,
    pub tool_calls: u64,
    pub files_changed: u64,
    pub latency_ms: u64,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Default for AgentMetrics {
    fn default() -> Self {
        Self {
            tokens_used: 0,
            requests: 0,
            tool_calls: 0,
            files_changed: 0,
            latency_ms: 0,
            started_at: None,
            completed_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionLevel {
    ReadOnly,
    WorkspaceWrite,
    FullWorkspace,
    Unrestricted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalPolicy {
    Always,
    OnRiskyCommands,
    OnNewCommand,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub provider: String,
    pub model: String,
    pub api_key_env: Option<String>,
    pub base_url: Option<String>,
    pub context_window: Option<u32>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}
