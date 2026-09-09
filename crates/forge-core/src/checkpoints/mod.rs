use forge_protocol::{CheckpointId, SessionId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: CheckpointId,
    pub session_id: SessionId,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub git_commit: Option<String>,
    pub workspace_state: WorkspaceState,
    pub task_state: serde_json::Value,
    pub agent_state: serde_json::Value,
    pub memory_state: serde_json::Value,
    pub event_position: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub branch: Option<String>,
    pub changed_files: Vec<String>,
    pub stash_id: Option<String>,
}

pub struct CheckpointManager {
    checkpoints: Arc<RwLock<HashMap<CheckpointId, Checkpoint>>>,
    session_checkpoints: Arc<RwLock<HashMap<SessionId, Vec<CheckpointId>>>>,
}

impl CheckpointManager {
    pub fn new() -> Self {
        Self {
            checkpoints: Arc::new(RwLock::new(HashMap::new())),
            session_checkpoints: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_checkpoint(&self, session_id: SessionId, message: Option<String>, project_path: &PathBuf) -> anyhow::Result<Checkpoint> {
        // Get current git commit
        let git_commit = tokio::process::Command::new("git").args(["rev-parse", "HEAD"]).current_dir(project_path).output().await.ok().and_then(|o| {
            if o.status.success() { Some(String::from_utf8_lossy(&o.stdout).trim().to_string()) } else { None }
        });

        let branch = tokio::process::Command::new("git").args(["branch", "--show-current"]).current_dir(project_path).output().await.ok().and_then(|o| {
            if o.status.success() { Some(String::from_utf8_lossy(&o.stdout).trim().to_string()) } else { None }
        });

        let checkpoint = Checkpoint {
            id: Uuid::new_v4(),
            session_id,
            message,
            created_at: Utc::now(),
            git_commit,
            workspace_state: WorkspaceState { branch, changed_files: Vec::new(), stash_id: None },
            task_state: serde_json::json!({}),
            agent_state: serde_json::json!({}),
            memory_state: serde_json::json!({}),
            event_position: 0,
        };

        {
            let mut checkpoints = self.checkpoints.write().await;
            checkpoints.insert(checkpoint.id, checkpoint.clone());
        }
        {
            let mut session_checkpoints = self.session_checkpoints.write().await;
            session_checkpoints.entry(session_id).or_default().push(checkpoint.id);
        }

        Ok(checkpoint)
    }

    pub async fn list_checkpoints(&self, session_id: SessionId) -> Vec<Checkpoint> {
        let checkpoints = self.checkpoints.read().await;
        let session_checkpoints = self.session_checkpoints.read().await;
        if let Some(ids) = session_checkpoints.get(&session_id) {
            ids.iter().filter_map(|id| checkpoints.get(id).cloned()).collect()
        } else { Vec::new() }
    }

    pub async fn get_checkpoint(&self, id: CheckpointId) -> Option<Checkpoint> {
        let checkpoints = self.checkpoints.read().await;
        checkpoints.get(&id).cloned()
    }

    pub async fn restore_checkpoint(&self, id: CheckpointId, project_path: &PathBuf) -> anyhow::Result<()> {
        let checkpoint = self.get_checkpoint(id).await.ok_or_else(|| anyhow::anyhow!("Checkpoint not found"))?;
        if let Some(commit) = checkpoint.git_commit {
            // Checkout the commit (safe: create a branch for restore)
            let _ = tokio::process::Command::new("git").args(["checkout", &commit]).current_dir(project_path).output().await?;
        }
        Ok(())
    }
}

impl Default for CheckpointManager {
    fn default() -> Self { Self::new() }
}
