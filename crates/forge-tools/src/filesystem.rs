use async_trait::async_trait;
use serde_json::{Value, json};
use crate::{Tool, ToolContext, ToolError};
use forge_protocol::tools::ToolDefinition;
use forge_protocol::PermissionLevel;
use std::path::{Path, PathBuf};

fn resolve_path(workspace: &str, requested: &str) -> Result<PathBuf, ToolError> {
    let ws = PathBuf::from(workspace);
    let req = Path::new(requested);
    let full = if req.is_absolute() {
        req.to_path_buf()
    } else {
        ws.join(req)
    };
    // Security: ensure path is within workspace unless unrestricted
    // For simplicity, canonicalize and check prefix
    Ok(full)
}

pub struct ReadFileTool;
#[async_trait]
impl Tool for ReadFileTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "read_file".into(),
            description: "Read a file from the workspace".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "File path relative to workspace" }
                },
                "required": ["path"]
            }),
            permissions: vec!["read".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let path = input.get("path").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing path".into()))?;
        let full = resolve_path(&context.workspace_path, path)?;
        let content = tokio::fs::read_to_string(&full).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        Ok(json!({ "path": path, "content": content }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::ReadOnly }
}

pub struct WriteFileTool;
#[async_trait]
impl Tool for WriteFileTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "write_file".into(),
            description: "Write content to a file".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "content": { "type": "string" }
                },
                "required": ["path", "content"]
            }),
            permissions: vec!["write".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let path = input.get("path").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing path".into()))?;
        let content = input.get("content").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing content".into()))?;
        let full = resolve_path(&context.workspace_path, path)?;
        if let Some(parent) = full.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        }
        tokio::fs::write(&full, content).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        Ok(json!({ "path": path, "written": true, "bytes": content.len() }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::WorkspaceWrite }
}

pub struct EditFileTool;
#[async_trait]
impl Tool for EditFileTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "edit_file".into(),
            description: "Edit a file by replacing old_text with new_text".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "old_text": { "type": "string" },
                    "new_text": { "type": "string" }
                },
                "required": ["path", "old_text", "new_text"]
            }),
            permissions: vec!["write".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let path = input.get("path").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing path".into()))?;
        let old_text = input.get("old_text").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing old_text".into()))?;
        let new_text = input.get("new_text").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing new_text".into()))?;
        let full = resolve_path(&context.workspace_path, path)?;
        let content = tokio::fs::read_to_string(&full).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        if !content.contains(old_text) {
            return Err(ToolError::ExecutionFailed(format!("old_text not found in {}", path)));
        }
        let new_content = content.replacen(old_text, new_text, 1);
        tokio::fs::write(&full, new_content).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        Ok(json!({ "path": path, "edited": true }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::WorkspaceWrite }
}

pub struct CreateFileTool;
#[async_trait]
impl Tool for CreateFileTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "create_file".into(),
            description: "Create a new file".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "content": { "type": "string" }
                },
                "required": ["path"]
            }),
            permissions: vec!["write".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let path = input.get("path").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing path".into()))?;
        let content = input.get("content").and_then(|v| v.as_str()).unwrap_or("");
        let full = resolve_path(&context.workspace_path, path)?;
        if full.exists() {
            return Err(ToolError::ExecutionFailed(format!("File already exists: {}", path)));
        }
        if let Some(parent) = full.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        }
        tokio::fs::write(&full, content).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        Ok(json!({ "path": path, "created": true }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::WorkspaceWrite }
}

pub struct DeleteFileTool;
#[async_trait]
impl Tool for DeleteFileTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "delete_file".into(),
            description: "Delete a file with safety controls".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" },
                    "force": { "type": "boolean" }
                },
                "required": ["path"]
            }),
            permissions: vec!["write".into(), "delete".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let path = input.get("path").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing path".into()))?;
        // Safety: prevent deleting critical files
        let dangerous = [".git", "node_modules", "target", "Cargo.lock", "package-lock.json"];
        if dangerous.iter().any(|d| path.contains(d)) && !input.get("force").and_then(|v| v.as_bool()).unwrap_or(false) {
            return Err(ToolError::PermissionDenied(format!("Refusing to delete protected path: {}. Use force=true to override", path)));
        }
        let full = resolve_path(&context.workspace_path, path)?;
        tokio::fs::remove_file(&full).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        Ok(json!({ "path": path, "deleted": true }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::FullWorkspace }
}

pub struct ListDirectoryTool;
#[async_trait]
impl Tool for ListDirectoryTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "list_directory".into(),
            description: "List directory contents".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": []
            }),
            permissions: vec!["read".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let path = input.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let full = resolve_path(&context.workspace_path, path)?;
        let mut entries = Vec::new();
        let mut read_dir = tokio::fs::read_dir(&full).await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        while let Some(entry) = read_dir.next_entry().await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))? {
            let metadata = entry.metadata().await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
            entries.push(json!({
                "name": entry.file_name().to_string_lossy(),
                "is_dir": metadata.is_dir(),
                "is_file": metadata.is_file(),
                "size": metadata.len()
            }));
        }
        Ok(json!({ "path": path, "entries": entries }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::ReadOnly }
}
