use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkspaceType {
    Main,
    Agent(Uuid),
    Team(Uuid),
    Temporary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub path: PathBuf,
    pub workspace_type: WorkspaceType,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Workspace {
    pub fn new(path: PathBuf, workspace_type: WorkspaceType) -> Self {
        Self { id: Uuid::new_v4(), path, workspace_type, created_at: chrono::Utc::now() }
    }
}

pub struct WorkspaceManager {
    main_workspace: Option<Workspace>,
    agent_workspaces: std::collections::HashMap<Uuid, Workspace>,
    temp_workspaces: Vec<Workspace>,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self { main_workspace: None, agent_workspaces: std::collections::HashMap::new(), temp_workspaces: Vec::new() }
    }

    pub fn set_main(&mut self, path: PathBuf) -> Workspace {
        let ws = Workspace::new(path, WorkspaceType::Main);
        self.main_workspace = Some(ws.clone());
        ws
    }

    pub fn create_agent_workspace(&mut self, agent_id: Uuid, base: &Path) -> anyhow::Result<Workspace> {
        let agent_path = base.join(".forge").join("workspaces").join(agent_id.to_string());
        std::fs::create_dir_all(&agent_path)?;
        let ws = Workspace::new(agent_path, WorkspaceType::Agent(agent_id));
        self.agent_workspaces.insert(agent_id, ws.clone());
        Ok(ws)
    }

    pub fn get_main(&self) -> Option<&Workspace> {
        self.main_workspace.as_ref()
    }

    pub fn get_agent_workspace(&self, agent_id: Uuid) -> Option<&Workspace> {
        self.agent_workspaces.get(&agent_id)
    }

    pub fn list_changed_files(&self, workspace_path: &Path) -> anyhow::Result<Vec<String>> {
        // Use git to list changed files
        let output = std::process::Command::new("git").args(["status", "--porcelain"]).current_dir(workspace_path).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let files = stdout.lines().filter_map(|line| {
            if line.len() > 3 {
                Some(line[3..].trim().to_string())
            } else { None }
        }).collect();
        Ok(files)
    }
}

impl Default for WorkspaceManager {
    fn default() -> Self { Self::new() }
}
