use forge_protocol::{AgentId, SessionId, TaskId, TeamId, AgentState, AgentRole, PermissionLevel, AgentMetrics, AgentInfo};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use tokio::sync::{mpsc, RwLock};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
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
    pub is_paused: bool,
}

impl Agent {
    pub fn new(session_id: SessionId, name: String, role: AgentRole, model: String, workspace: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            session_id,
            name,
            role,
            state: AgentState::Created,
            progress: 0.0,
            current_task: None,
            team_id: None,
            parent_id: None,
            children: Vec::new(),
            model,
            permissions: PermissionLevel::WorkspaceWrite,
            workspace,
            metrics: AgentMetrics { started_at: Some(now), ..Default::default() },
            created_at: now,
            updated_at: now,
            current_action: None,
            files_changed: Vec::new(),
            tools_used: Vec::new(),
            is_paused: false,
        }
    }

    pub fn to_info(&self) -> AgentInfo {
        AgentInfo {
            id: self.id,
            session_id: self.session_id,
            name: self.name.clone(),
            role: self.role.clone(),
            state: self.state.clone(),
            progress: self.progress,
            current_task: self.current_task,
            team_id: self.team_id,
            parent_id: self.parent_id,
            children: self.children.clone(),
            model: self.model.clone(),
            permissions: self.permissions.clone(),
            workspace: self.workspace.clone(),
            metrics: self.metrics.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            current_action: self.current_action.clone(),
            files_changed: self.files_changed.clone(),
            tools_used: self.tools_used.clone(),
        }
    }

    pub fn set_state(&mut self, state: AgentState) {
        self.state = state;
        self.updated_at = Utc::now();
    }

    pub fn set_progress(&mut self, progress: f32, action: Option<String>) {
        self.progress = progress.clamp(0.0, 100.0);
        if let Some(a) = action {
            self.current_action = Some(a);
        }
        self.updated_at = Utc::now();
    }
}

pub struct AgentRuntime {
    agents: Arc<RwLock<HashMap<AgentId, Agent>>>,
    event_sender: mpsc::Sender<forge_protocol::events::ForgeEvent>,
}

impl AgentRuntime {
    pub fn new(event_sender: mpsc::Sender<forge_protocol::events::ForgeEvent>) -> Self {
        Self {
            agents: Arc::new(RwLock::new(HashMap::new())),
            event_sender,
        }
    }

    pub async fn create_agent(&self, agent: Agent) -> AgentId {
        let id = agent.id;
        let mut agents = self.agents.write().await;
        agents.insert(id, agent.clone());
        // Emit event
        let event = forge_protocol::events::ForgeEvent::new(
            Some(agent.session_id),
            forge_protocol::events::EventPayload::AgentCreated {
                agent_id: id,
                name: agent.name.clone(),
                role: agent.role.to_string(),
            },
        );
        let _ = self.event_sender.send(event).await;
        id
    }

    pub async fn get_agent(&self, id: AgentId) -> Option<Agent> {
        let agents = self.agents.read().await;
        agents.get(&id).cloned()
    }

    pub async fn list_agents(&self, session_id: SessionId) -> Vec<Agent> {
        let agents = self.agents.read().await;
        agents.values().filter(|a| a.session_id == session_id).cloned().collect()
    }

    pub async fn update_agent<F>(&self, id: AgentId, f: F) -> Option<Agent>
    where
        F: FnOnce(&mut Agent),
    {
        let mut agents = self.agents.write().await;
        if let Some(agent) = agents.get_mut(&id) {
            f(agent);
            Some(agent.clone())
        } else {
            None
        }
    }

    pub async fn pause_agent(&self, id: AgentId) -> Result<(), String> {
        let mut agents = self.agents.write().await;
        if let Some(agent) = agents.get_mut(&id) {
            agent.is_paused = true;
            agent.set_state(AgentState::Paused);
            let event = forge_protocol::events::ForgeEvent::new(
                Some(agent.session_id),
                forge_protocol::events::EventPayload::AgentPaused { agent_id: id },
            );
            let sender = self.event_sender.clone();
            tokio::spawn(async move { let _ = sender.send(event).await; });
            Ok(())
        } else {
            Err(format!("Agent not found: {}", id))
        }
    }

    pub async fn resume_agent(&self, id: AgentId) -> Result<(), String> {
        let mut agents = self.agents.write().await;
        if let Some(agent) = agents.get_mut(&id) {
            agent.is_paused = false;
            agent.set_state(AgentState::Idle);
            let event = forge_protocol::events::ForgeEvent::new(
                Some(agent.session_id),
                forge_protocol::events::EventPayload::AgentResumed { agent_id: id },
            );
            let sender = self.event_sender.clone();
            tokio::spawn(async move { let _ = sender.send(event).await; });
            Ok(())
        } else {
            Err(format!("Agent not found: {}", id))
        }
    }

    pub async fn stop_agent(&self, id: AgentId) -> Result<(), String> {
        let mut agents = self.agents.write().await;
        if let Some(agent) = agents.get_mut(&id) {
            agent.set_state(AgentState::Cancelled);
            let event = forge_protocol::events::ForgeEvent::new(
                Some(agent.session_id),
                forge_protocol::events::EventPayload::AgentFailed { agent_id: id, error: "Cancelled by user".into() },
            );
            let sender = self.event_sender.clone();
            tokio::spawn(async move { let _ = sender.send(event).await; });
            Ok(())
        } else {
            Err(format!("Agent not found: {}", id))
        }
    }

    pub async fn spawn_subagent(&self, parent_id: AgentId, name: String, role: AgentRole) -> Result<AgentId, String> {
        let parent = {
            let agents = self.agents.read().await;
            agents.get(&parent_id).cloned().ok_or_else(|| format!("Parent agent not found: {}", parent_id))?
        };
        let mut child = Agent::new(parent.session_id, name, role, parent.model.clone(), parent.workspace.clone());
        child.parent_id = Some(parent_id);
        child.team_id = parent.team_id;
        let child_id = child.id;

        {
            let mut agents = self.agents.write().await;
            agents.insert(child_id, child);
            if let Some(parent_agent) = agents.get_mut(&parent_id) {
                parent_agent.children.push(child_id);
            }
        }

        Ok(child_id)
    }
}
