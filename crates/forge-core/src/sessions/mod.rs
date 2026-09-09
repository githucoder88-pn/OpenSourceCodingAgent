use forge_protocol::{SessionId, AgentId, TaskId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub name: String,
    pub project_path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub agents: Vec<AgentId>,
    pub tasks: Vec<TaskId>,
    pub current_branch: Option<String>,
    pub status: SessionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionStatus {
    Active,
    Paused,
    Completed,
    Archived,
}

impl Session {
    pub fn new(name: String, project_path: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            project_path,
            created_at: now,
            updated_at: now,
            agents: Vec::new(),
            tasks: Vec::new(),
            current_branch: None,
            status: SessionStatus::Active,
        }
    }
}

pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
    db_path: Option<String>,
}

impl SessionManager {
    pub fn new(db_path: Option<String>) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            db_path,
        }
    }

    pub async fn create_session(&self, name: String, project_path: String) -> Session {
        let session = Session::new(name, project_path);
        let id = session.id;
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(id, session.clone());
        }
        session
    }

    pub async fn get_session(&self, id: SessionId) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(&id).cloned()
    }

    pub async fn list_sessions(&self) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions.values().cloned().collect()
    }

    pub async fn resume_session(&self, id: SessionId) -> Option<Session> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&id) {
            session.updated_at = Utc::now();
            session.status = SessionStatus::Active;
            Some(session.clone())
        } else {
            None
        }
    }

    pub async fn add_agent(&self, session_id: SessionId, agent_id: AgentId) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.agents.push(agent_id);
            session.updated_at = Utc::now();
        }
    }

    pub async fn add_task(&self, session_id: SessionId, task_id: TaskId) {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(&session_id) {
            session.tasks.push(task_id);
            session.updated_at = Utc::now();
        }
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new(None)
    }
}
