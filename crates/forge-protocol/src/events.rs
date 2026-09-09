use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::{AgentId, TaskId, TeamId, SessionId, ToolCallId, CheckpointId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub session_id: Option<SessionId>,
    pub payload: EventPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventPayload {
    SessionCreated { session_id: SessionId, project_path: String },
    SessionResumed { session_id: SessionId },

    AgentCreated { agent_id: AgentId, name: String, role: String },
    AgentStarted { agent_id: AgentId },
    AgentProgress { agent_id: AgentId, progress: f32, message: String },
    AgentWaiting { agent_id: AgentId, reason: String },
    AgentBlocked { agent_id: AgentId, reason: String },
    AgentPaused { agent_id: AgentId },
    AgentResumed { agent_id: AgentId },
    AgentCompleted { agent_id: AgentId },
    AgentFailed { agent_id: AgentId, error: String },

    AgentMessageSent { message_id: Uuid, from: AgentId, to: Option<AgentId>, team_id: Option<TeamId>, body: String },
    AgentMessageReceived { message_id: Uuid, to: AgentId, from: AgentId, body: String },

    TaskCreated { task_id: TaskId, title: String },
    TaskStarted { task_id: TaskId, agent_id: Option<AgentId> },
    TaskUpdated { task_id: TaskId, progress: f32, status: String },
    TaskCompleted { task_id: TaskId },
    TaskFailed { task_id: TaskId, error: String },

    ToolStarted { tool_call_id: ToolCallId, agent_id: AgentId, tool: String, input: serde_json::Value },
    ToolOutput { tool_call_id: ToolCallId, chunk: String, is_stderr: bool },
    ToolCompleted { tool_call_id: ToolCallId, result: serde_json::Value },
    ToolFailed { tool_call_id: ToolCallId, error: String },

    FileCreated { path: String, agent_id: Option<AgentId> },
    FileModified { path: String, agent_id: Option<AgentId>, diff: Option<String> },
    FileDeleted { path: String, agent_id: Option<AgentId> },

    ModelRequested { agent_id: AgentId, model: String, provider: String },
    ModelStarted { agent_id: AgentId, model: String },
    ModelStream { agent_id: AgentId, delta: String },
    ModelCompleted { agent_id: AgentId, tokens: u64 },
    ModelFailed { agent_id: AgentId, error: String },
    ModelFallback { agent_id: AgentId, from: String, to: String, reason: String },

    GitChanged { branch: Option<String>, status: String },

    TestStarted { suite: String },
    TestFailed { suite: String, error: String },
    TestPassed { suite: String },

    CheckpointCreated { checkpoint_id: CheckpointId, message: Option<String> },
    CheckpointRestored { checkpoint_id: CheckpointId },

    Log { level: String, message: String, agent_id: Option<AgentId> },
}

impl ForgeEvent {
    pub fn new(session_id: Option<SessionId>, payload: EventPayload) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            session_id,
            payload,
        }
    }
}
