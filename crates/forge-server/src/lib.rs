pub mod api;
pub mod websocket;
pub mod runtime;

use std::sync::Arc;
use forge_core::{sessions::SessionManager, agent::AgentRuntime, task_graph::TaskGraphManager, teams::TeamManager, messaging::MessageBus, events::EventBus, checkpoints::CheckpointManager, permissions::PermissionManager};
use tokio::sync::mpsc;
use forge_protocol::events::ForgeEvent;

pub struct ForgeServer {
    pub session_manager: Arc<SessionManager>,
    pub agent_runtime: Arc<AgentRuntime>,
    pub task_manager: Arc<TaskGraphManager>,
    pub team_manager: Arc<TeamManager>,
    pub message_bus: Arc<MessageBus>,
    pub event_bus: Arc<EventBus>,
    pub checkpoint_manager: Arc<CheckpointManager>,
    pub permission_manager: Arc<PermissionManager>,
}

impl ForgeServer {
    pub fn new() -> Self {
        let event_bus = Arc::new(EventBus::new(10000));
        let (mpsc_tx, mpsc_rx) = mpsc::channel::<ForgeEvent>(1000);
        
        let event_bus_clone = event_bus.clone();
        tokio::spawn(async move {
            let bridge = forge_core::events::EventBridge::new(mpsc_rx, event_bus_clone);
            bridge.run().await;
        });

        Self {
            session_manager: Arc::new(SessionManager::new(None)),
            agent_runtime: Arc::new(AgentRuntime::new(mpsc_tx)),
            task_manager: Arc::new(TaskGraphManager::new()),
            team_manager: Arc::new(TeamManager::new()),
            message_bus: Arc::new(MessageBus::new()),
            event_bus,
            checkpoint_manager: Arc::new(CheckpointManager::new()),
            permission_manager: Arc::new(PermissionManager::new()),
        }
    }

    pub async fn start(&self, addr: &str) -> anyhow::Result<()> {
        let app = api::create_router(self);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        tracing::info!("Forge server listening on {}", addr);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

impl Default for ForgeServer {
    fn default() -> Self {
        Self::new()
    }
}
