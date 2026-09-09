use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::{TaskId, SessionId, AgentId, TeamId};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Ready,
    Running,
    Blocked,
    Reviewing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInfo {
    pub id: TaskId,
    pub session_id: SessionId,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: u8,
    pub dependencies: Vec<TaskId>,
    pub owner: Option<AgentId>,
    pub team_id: Option<TeamId>,
    pub progress: f32,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub artifacts: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskGraph {
    pub tasks: Vec<TaskInfo>,
    pub edges: Vec<(TaskId, TaskId)>, // dependency edges
}

impl TaskGraph {
    pub fn new() -> Self {
        Self { tasks: Vec::new(), edges: Vec::new() }
    }

    pub fn add_task(&mut self, task: TaskInfo) {
        for dep in &task.dependencies {
            self.edges.push((*dep, task.id));
        }
        self.tasks.push(task);
    }

    pub fn ready_tasks(&self) -> Vec<&TaskInfo> {
        self.tasks.iter().filter(|t| {
            t.status == TaskStatus::Pending && 
            t.dependencies.iter().all(|dep_id| {
                self.tasks.iter().find(|tt| tt.id == *dep_id).map(|tt| tt.status == TaskStatus::Completed).unwrap_or(false)
            })
        }).collect()
    }

    pub fn blocked_tasks(&self) -> Vec<&TaskInfo> {
        self.tasks.iter().filter(|t| t.status == TaskStatus::Blocked).collect()
    }
}

impl Default for TaskGraph {
    fn default() -> Self {
        Self::new()
    }
}
