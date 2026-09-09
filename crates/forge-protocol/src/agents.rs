use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::{AgentId, SessionId, TeamId, TaskId, AgentState, AgentRole, PermissionLevel, AgentMetrics};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: AgentId,
    pub session_id: SessionId,
    pub name: String,
    pub role: AgentRole,
    pub state: AgentState,
    pub progress: f32,
    pub current_task: Option<TaskId>,
    pub team_id: Option<TeamId>,
    pub parent_id: Option<AgentId>,
    pub children: Vec<AgentId>,
    pub model: String,
    pub permissions: PermissionLevel,
    pub workspace: String,
    pub metrics: AgentMetrics,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub current_action: Option<String>,
    pub files_changed: Vec<String>,
    pub tools_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    pub role: AgentRole,
    pub model: Option<String>,
    pub team_id: Option<TeamId>,
    pub parent_id: Option<AgentId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMessage {
    pub id: uuid::Uuid,
    pub from: AgentId,
    pub to: Option<AgentId>,
    pub team_id: Option<TeamId>,
    pub task_id: Option<TaskId>,
    pub message_type: MessageType,
    pub subject: String,
    pub body: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Request,
    Response,
    Question,
    Answer,
    Status,
    Handoff,
    Warning,
    Blocked,
    Approval,
    Broadcast,
}
