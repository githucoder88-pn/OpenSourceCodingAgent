use forge_protocol::events::{ForgeEvent, EventPayload};
use tokio::sync::{broadcast, mpsc};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct EventBus {
    sender: broadcast::Sender<ForgeEvent>,
    history: Arc<RwLock<Vec<ForgeEvent>>>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self {
            sender,
            history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ForgeEvent> {
        self.sender.subscribe()
    }

    pub async fn publish(&self, event: ForgeEvent) {
        {
            let mut history = self.history.write().await;
            history.push(event.clone());
            if history.len() > 10000 {
                history.remove(0);
            }
        }
        let _ = self.sender.send(event);
    }

    pub async fn history(&self, session_id: Option<forge_protocol::SessionId>, limit: usize) -> Vec<ForgeEvent> {
        let history = self.history.read().await;
        let mut filtered: Vec<_> = history.iter()
            .filter(|e| session_id.map(|sid| e.session_id == Some(sid)).unwrap_or(true))
            .cloned()
            .collect();
        filtered.truncate(limit);
        filtered
    }

    pub fn sender(&self) -> broadcast::Sender<ForgeEvent> {
        self.sender.clone()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(1000)
    }
}

// Adapter for mpsc to broadcast
pub struct EventBridge {
    mpsc_receiver: mpsc::Receiver<ForgeEvent>,
    event_bus: Arc<EventBus>,
}

impl EventBridge {
    pub fn new(receiver: mpsc::Receiver<ForgeEvent>, event_bus: Arc<EventBus>) -> Self {
        Self { mpsc_receiver: receiver, event_bus }
    }

    pub async fn run(mut self) {
        while let Some(event) = self.mpsc_receiver.recv().await {
            self.event_bus.publish(event).await;
        }
    }
}
