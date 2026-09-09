use axum::{Router, routing::{get, post}, extract::{State, Json}, response::IntoResponse, http::StatusCode};
use std::sync::Arc;
use serde_json::{json, Value};
use crate::ForgeServer;
use forge_protocol::rpc::*;

pub fn create_router(server: &ForgeServer) -> Router {
    let state = Arc::new(ServerState {
        session_manager: server.session_manager.clone(),
        agent_runtime: server.agent_runtime.clone(),
        task_manager: server.task_manager.clone(),
        team_manager: server.team_manager.clone(),
        message_bus: server.message_bus.clone(),
        event_bus: server.event_bus.clone(),
        checkpoint_manager: server.checkpoint_manager.clone(),
    });

    Router::new()
        .route("/", get(root))
        .route("/api/health", get(health))
        .route("/api/rpc", post(rpc_handler))
        .route("/api/sessions", get(list_sessions).post(create_session))
        .route("/api/sessions/:id/agents", get(list_agents))
        .route("/api/sessions/:id/tasks", get(list_tasks))
        .route("/api/sessions/:id/teams", get(list_teams))
        .route("/api/sessions/:id/checkpoints", get(list_checkpoints))
        .route("/ws", get(crate::websocket::ws_handler))
        .with_state(state)
}

#[derive(Clone)]
pub struct ServerState {
    pub session_manager: Arc<forge_core::sessions::SessionManager>,
    pub agent_runtime: Arc<forge_core::agent::AgentRuntime>,
    pub task_manager: Arc<forge_core::task_graph::TaskGraphManager>,
    pub team_manager: Arc<forge_core::teams::TeamManager>,
    pub message_bus: Arc<forge_core::messaging::MessageBus>,
    pub event_bus: Arc<forge_core::events::EventBus>,
    pub checkpoint_manager: Arc<forge_core::checkpoints::CheckpointManager>,
}

async fn root() -> impl IntoResponse {
    Json(json!({ "name": "Forge Core", "version": "0.1.0", "status": "running" }))
}

async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok", "timestamp": chrono::Utc::now().to_rfc3339() }))
}

async fn rpc_handler(State(state): State<Arc<ServerState>>, Json(req): Json<JsonRpcRequest>) -> impl IntoResponse {
    let result = match req.method.as_str() {
        "create_session" => {
            if let Some(params) = req.params {
                if let Ok(p) = serde_json::from_value::<CreateSessionParams>(params) {
                    let session = state.session_manager.create_session(p.name.unwrap_or_else(|| "Untitled".into()), p.project_path).await;
                    json!({ "session_id": session.id, "name": session.name })
                } else {
                    json!({ "error": "invalid params" })
                }
            } else {
                json!({ "error": "missing params" })
            }
        }
        "list_sessions" => {
            let sessions = state.session_manager.list_sessions().await;
            json!(sessions)
        }
        "list_agents" => {
            if let Some(params) = req.params {
                if let Ok(p) = serde_json::from_value::<ListAgentsParams>(params) {
                    let agents = state.agent_runtime.list_agents(p.session_id).await;
                    json!(agents.iter().map(|a| a.to_info()).collect::<Vec<_>>())
                } else { json!([]) }
            } else { json!([]) }
        }
        "list_tasks" => {
            if let Some(params) = req.params {
                if let Ok(p) = serde_json::from_value::<ListTasksParams>(params) {
                    let tasks = state.task_manager.list_tasks(p.session_id).await;
                    json!(tasks)
                } else { json!([]) }
            } else { json!([]) }
        }
        _ => json!({ "error": format!("Unknown method: {}", req.method) }),
    };

    Json(JsonRpcResponse {
        jsonrpc: "2.0".into(),
        id: req.id,
        result: Some(result),
        error: None,
    })
}

async fn create_session(State(state): State<Arc<ServerState>>, Json(params): Json<CreateSessionParams>) -> impl IntoResponse {
    let session = state.session_manager.create_session(params.name.unwrap_or_else(|| "Untitled".into()), params.project_path).await;
    (StatusCode::CREATED, Json(session))
}

async fn list_sessions(State(state): State<Arc<ServerState>>) -> impl IntoResponse {
    let sessions = state.session_manager.list_sessions().await;
    Json(sessions)
}

async fn list_agents(State(state): State<Arc<ServerState>>, axum::extract::Path(id): axum::extract::Path<String>) -> impl IntoResponse {
    if let Ok(session_id) = uuid::Uuid::parse_str(&id) {
        let agents = state.agent_runtime.list_agents(session_id).await;
        Json(agents.iter().map(|a| a.to_info()).collect::<Vec<_>>())
    } else {
        Json(vec![] as Vec<forge_protocol::agents::AgentInfo>)
    }
}

async fn list_tasks(State(state): State<Arc<ServerState>>, axum::extract::Path(id): axum::extract::Path<String>) -> impl IntoResponse {
    if let Ok(session_id) = uuid::Uuid::parse_str(&id) {
        let tasks = state.task_manager.list_tasks(session_id).await;
        Json(tasks)
    } else {
        Json(vec![] as Vec<forge_protocol::tasks::TaskInfo>)
    }
}

async fn list_teams(State(state): State<Arc<ServerState>>, axum::extract::Path(id): axum::extract::Path<String>) -> impl IntoResponse {
    if let Ok(session_id) = uuid::Uuid::parse_str(&id) {
        let teams = state.team_manager.list_teams(session_id).await;
        Json(teams.iter().map(|t| t.info.clone()).collect::<Vec<_>>())
    } else {
        Json(vec![] as Vec<forge_protocol::teams::TeamInfo>)
    }
}

async fn list_checkpoints(State(state): State<Arc<ServerState>>, axum::extract::Path(id): axum::extract::Path<String>) -> impl IntoResponse {
    if let Ok(session_id) = uuid::Uuid::parse_str(&id) {
        let cps = state.checkpoint_manager.list_checkpoints(session_id).await;
        Json(cps)
    } else {
        Json(vec![] as Vec<forge_core::checkpoints::Checkpoint>)
    }
}
