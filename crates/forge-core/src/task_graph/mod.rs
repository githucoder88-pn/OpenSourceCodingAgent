use forge_protocol::{TaskId, SessionId, AgentId, TeamId};
use forge_protocol::tasks::{TaskInfo, TaskStatus, TaskGraph};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct TaskGraphManager {
    graphs: Arc<RwLock<HashMap<SessionId, TaskGraph>>>,
}

impl TaskGraphManager {
    pub fn new() -> Self {
        Self { graphs: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub async fn create_task(&self, session_id: SessionId, title: String, description: Option<String>, depends_on: Vec<TaskId>, priority: u8, owner: Option<AgentId>, team_id: Option<TeamId>) -> TaskInfo {
        let task = TaskInfo {
            id: Uuid::new_v4(),
            session_id,
            title,
            description,
            status: TaskStatus::Pending,
            priority,
            dependencies: depends_on,
            owner,
            team_id,
            progress: 0.0,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            artifacts: Vec::new(),
            errors: Vec::new(),
        };
        {
            let mut graphs = self.graphs.write().await;
            let graph = graphs.entry(session_id).or_insert_with(TaskGraph::new);
            graph.add_task(task.clone());
        }
        task
    }

    pub async fn get_graph(&self, session_id: SessionId) -> TaskGraph {
        let graphs = self.graphs.read().await;
        graphs.get(&session_id).cloned().unwrap_or_default()
    }

    pub async fn update_task_status(&self, session_id: SessionId, task_id: TaskId, status: TaskStatus, progress: Option<f32>) -> Option<TaskInfo> {
        let mut graphs = self.graphs.write().await;
        if let Some(graph) = graphs.get_mut(&session_id) {
            if let Some(task) = graph.tasks.iter_mut().find(|t| t.id == task_id) {
                task.status = status;
                if let Some(p) = progress { task.progress = p; }
                match task.status {
                    TaskStatus::Running => { if task.started_at.is_none() { task.started_at = Some(Utc::now()); } },
                    TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled => { task.completed_at = Some(Utc::now()); },
                    _ => {}
                }
                return Some(task.clone());
            }
        }
        None
    }

    pub async fn list_tasks(&self, session_id: SessionId) -> Vec<TaskInfo> {
        let graphs = self.graphs.read().await;
        graphs.get(&session_id).map(|g| g.tasks.clone()).unwrap_or_default()
    }

    pub async fn assign_task(&self, session_id: SessionId, task_id: TaskId, agent_id: AgentId) -> Option<TaskInfo> {
        let mut graphs = self.graphs.write().await;
        if let Some(graph) = graphs.get_mut(&session_id) {
            if let Some(task) = graph.tasks.iter_mut().find(|t| t.id == task_id) {
                task.owner = Some(agent_id);
                return Some(task.clone());
            }
        }
        None
    }
}

impl Default for TaskGraphManager {
    fn default() -> Self { Self::new() }
}
