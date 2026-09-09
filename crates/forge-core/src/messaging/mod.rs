use forge_protocol::agents::{AgentMessage, MessageType};
use forge_protocol::{AgentId, TeamId, TaskId};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use tokio::sync::{RwLock, broadcast};
use std::sync::Arc;

pub struct MessageBus {
    messages: Arc<RwLock<HashMap<Uuid, AgentMessage>>>,
    sender: broadcast::Sender<AgentMessage>,
}

impl MessageBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self { messages: Arc::new(RwLock::new(HashMap::new())), sender }
    }

    pub async fn send_message(&self, from: AgentId, to: Option<AgentId>, team_id: Option<TeamId>, task_id: Option<TaskId>, message_type: MessageType, subject: String, body: String) -> AgentMessage {
        let msg = AgentMessage {
            id: Uuid::new_v4(),
            from,
            to,
            team_id,
            task_id,
            message_type,
            subject,
            body,
            timestamp: Utc::now(),
        };
        {
            let mut messages = self.messages.write().await;
            messages.insert(msg.id, msg.clone());
        }
        let _ = self.sender.send(msg.clone());
        msg
    }

    pub async fn list_messages(&self, agent_id: Option<AgentId>, team_id: Option<TeamId>) -> Vec<AgentMessage> {
        let messages = self.messages.read().await;
        messages.values().filter(|m| {
            if let Some(aid) = agent_id {
                m.from == aid || m.to == Some(aid)
            } else if let Some(tid) = team_id {
                m.team_id == Some(tid)
            } else { true }
        }).cloned().collect()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<AgentMessage> {
        self.sender.subscribe()
    }
}

impl Default for MessageBus {
    fn default() -> Self { Self::new() }
}
