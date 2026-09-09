pub mod filesystem;
pub mod shell;
pub mod git;
pub mod search;
pub mod http;
pub mod package_manager;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use std::collections::HashMap;
use forge_protocol::tools::{ToolDefinition, ToolCall, ToolResult};
use forge_protocol::{ToolCallId, AgentId};

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("Tool not found: {0}")]
    NotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Timeout: {0}")]
    Timeout(String),
    #[error("Cancelled")]
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolContext {
    pub agent_id: AgentId,
    pub workspace_path: String,
    pub permissions: forge_protocol::PermissionLevel,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn definition(&self) -> ToolDefinition;
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError>;
    fn requires_permission(&self) -> forge_protocol::PermissionLevel;
}

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: HashMap::new() }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        let def = tool.definition();
        self.tools.insert(def.name.clone(), tool);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Tool> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    pub fn list(&self) -> Vec<ToolDefinition> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(filesystem::ReadFileTool));
        registry.register(Box::new(filesystem::WriteFileTool));
        registry.register(Box::new(filesystem::EditFileTool));
        registry.register(Box::new(filesystem::CreateFileTool));
        registry.register(Box::new(filesystem::DeleteFileTool));
        registry.register(Box::new(filesystem::ListDirectoryTool));
        registry.register(Box::new(search::SearchFilesTool));
        registry.register(Box::new(search::SearchSymbolsTool));
        registry.register(Box::new(shell::ShellTool));
        registry.register(Box::new(git::GitStatusTool));
        registry.register(Box::new(git::GitDiffTool));
        registry.register(Box::new(git::GitLogTool));
        registry.register(Box::new(git::GitAddTool));
        registry.register(Box::new(git::GitCommitTool));
        registry.register(Box::new(git::GitBranchTool));
        registry.register(Box::new(shell::RunTestsTool));
        registry.register(Box::new(shell::RunBuildTool));
        registry.register(Box::new(shell::RunLinterTool));
        registry.register(Box::new(http::HttpRequestTool));
        registry.register(Box::new(package_manager::PackageManagerTool));
        registry
    }
}

// Re-export for convenience
pub use filesystem::*;
pub use shell::*;
pub use git::*;
pub use search::*;
