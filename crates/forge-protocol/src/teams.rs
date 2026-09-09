use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::{TeamId, SessionId, AgentId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamInfo {
    pub id: TeamId,
    pub session_id: SessionId,
    pub name: String,
    pub manager: Option<AgentId>,
    pub members: Vec<AgentId>,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrchestrationMode {
    Solo,
    Supervisor,
    Parallel,
    Pipeline,
    Debate,
    ReviewLoop,
    Swarm,
    Company,
}
