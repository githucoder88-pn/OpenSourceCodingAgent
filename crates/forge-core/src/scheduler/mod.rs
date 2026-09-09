use forge_protocol::{TaskId, AgentId};
use std::collections::{HashMap, VecDeque};
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub task_id: TaskId,
    pub agent_id: Option<AgentId>,
    pub priority: u8,
    pub dependencies: Vec<TaskId>,
}

pub struct Scheduler {
    queue: Arc<RwLock<VecDeque<ScheduledTask>>>,
    running: Arc<RwLock<HashMap<TaskId, AgentId>>>,
    max_parallel: usize,
}

impl Scheduler {
    pub fn new(max_parallel: usize) -> Self {
        Self {
            queue: Arc::new(RwLock::new(VecDeque::new())),
            running: Arc::new(RwLock::new(HashMap::new())),
            max_parallel,
        }
    }

    pub async fn enqueue(&self, task: ScheduledTask) {
        let mut queue = self.queue.write().await;
        // Insert by priority (higher priority first)
        let pos = queue.iter().position(|t| t.priority < task.priority).unwrap_or(queue.len());
        queue.insert(pos, task);
    }

    pub async fn dequeue_ready(&self, completed_tasks: &[TaskId]) -> Option<ScheduledTask> {
        let mut queue = self.queue.write().await;
        let running = self.running.read().await;
        if running.len() >= self.max_parallel {
            return None;
        }

        // Find first task whose dependencies are all completed
        let idx = queue.iter().position(|t| {
            t.dependencies.iter().all(|dep| completed_tasks.contains(dep))
        });

        if let Some(i) = idx {
            queue.remove(i)
        } else {
            None
        }
    }

    pub async fn mark_running(&self, task_id: TaskId, agent_id: AgentId) {
        let mut running = self.running.write().await;
        running.insert(task_id, agent_id);
    }

    pub async fn mark_completed(&self, task_id: TaskId) {
        let mut running = self.running.write().await;
        running.remove(&task_id);
    }

    pub async fn running_count(&self) -> usize {
        self.running.read().await.len()
    }

    pub async fn queue_len(&self) -> usize {
        self.queue.read().await.len()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new(4)
    }
}
