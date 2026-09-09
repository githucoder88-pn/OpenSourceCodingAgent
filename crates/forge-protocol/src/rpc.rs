use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{AgentId, SessionId, TaskId, TeamId, CheckpointId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Value,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcRequest {
    pub fn new(id: Value, method: impl Into<String>, params: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".into(),
            id,
            method: method.into(),
            params,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForgeMethod {
    #[serde(rename = "create_session")]
    CreateSession,
    #[serde(rename = "resume_session")]
    ResumeSession,
    #[serde(rename = "list_sessions")]
    ListSessions,
    #[serde(rename = "send_message")]
    SendMessage,
    #[serde(rename = "list_agents")]
    ListAgents,
    #[serde(rename = "create_agent")]
    CreateAgent,
    #[serde(rename = "pause_agent")]
    PauseAgent,
    #[serde(rename = "resume_agent")]
    ResumeAgent,
    #[serde(rename = "stop_agent")]
    StopAgent,
    #[serde(rename = "list_tasks")]
    ListTasks,
    #[serde(rename = "create_task")]
    CreateTask,
    #[serde(rename = "assign_task")]
    AssignTask,
    #[serde(rename = "list_teams")]
    ListTeams,
    #[serde(rename = "create_team")]
    CreateTeam,
    #[serde(rename = "send_agent_message")]
    SendAgentMessage,
    #[serde(rename = "stream_events")]
    StreamEvents,
    #[serde(rename = "execute_command")]
    ExecuteCommand,
    #[serde(rename = "inspect_workspace")]
    InspectWorkspace,
    #[serde(rename = "read_file")]
    ReadFile,
    #[serde(rename = "write_file")]
    WriteFile,
    #[serde(rename = "get_diff")]
    GetDiff,
    #[serde(rename = "get_model_status")]
    GetModelStatus,
    #[serde(rename = "configure_model")]
    ConfigureModel,
    #[serde(rename = "create_checkpoint")]
    CreateCheckpoint,
    #[serde(rename = "restore_checkpoint")]
    RestoreCheckpoint,
    #[serde(rename = "list_checkpoints")]
    ListCheckpoints,
    #[serde(rename = "get_context_stats")]
    GetContextStats,
}

// Request param types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionParams {
    pub project_path: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeSessionParams {
    pub session_id: SessionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageParams {
    pub session_id: SessionId,
    pub message: String,
    pub attachments: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAgentParams {
    pub session_id: SessionId,
    pub name: String,
    pub role: String,
    pub model: Option<String>,
    pub team_id: Option<TeamId>,
    pub parent_id: Option<AgentId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAgentsParams {
    pub session_id: SessionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentActionParams {
    pub agent_id: AgentId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskParams {
    pub session_id: SessionId,
    pub title: String,
    pub description: Option<String>,
    pub depends_on: Option<Vec<TaskId>>,
    pub priority: Option<u8>,
    pub owner: Option<AgentId>,
    pub team_id: Option<TeamId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignTaskParams {
    pub task_id: TaskId,
    pub agent_id: AgentId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTasksParams {
    pub session_id: SessionId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTeamParams {
    pub session_id: SessionId,
    pub name: String,
    pub members: Option<Vec<AgentId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendAgentMessageParams {
    pub from: AgentId,
    pub to: Option<AgentId>,
    pub team_id: Option<TeamId>,
    pub subject: String,
    pub body: String,
    pub task_id: Option<TaskId>,
    pub message_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteCommandParams {
    pub session_id: SessionId,
    pub command: String,
    pub cwd: Option<String>,
    pub agent_id: Option<AgentId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadFileParams {
    pub session_id: SessionId,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteFileParams {
    pub session_id: SessionId,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDiffParams {
    pub session_id: SessionId,
    pub agent_id: Option<AgentId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckpointParams {
    pub session_id: SessionId,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreCheckpointParams {
    pub session_id: SessionId,
    pub checkpoint_id: CheckpointId,
}
