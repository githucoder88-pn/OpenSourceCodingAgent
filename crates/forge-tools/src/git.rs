use async_trait::async_trait;
use serde_json::{Value, json};
use crate::{Tool, ToolContext, ToolError};
use forge_protocol::tools::ToolDefinition;
use forge_protocol::PermissionLevel;
use tokio::process::Command;

async fn run_git(args: &[&str], cwd: &str) -> Result<(String, String, bool, Option<i32>), ToolError> {
    let output = Command::new("git").args(args).current_dir(cwd).output().await.map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
    Ok((
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.success(),
        output.status.code(),
    ))
}

pub struct GitStatusTool;
#[async_trait]
impl Tool for GitStatusTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "git_status".into(),
            description: "Get git status".into(),
            parameters: json!({ "type": "object", "properties": {} }),
            permissions: vec!["read".into(), "git".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, _input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let (stdout, stderr, success, code) = run_git(&["status", "--porcelain", "-b"], &context.workspace_path).await?;
        Ok(json!({ "stdout": stdout, "stderr": stderr, "success": success, "code": code }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::ReadOnly }
}

pub struct GitDiffTool;
#[async_trait]
impl Tool for GitDiffTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "git_diff".into(),
            description: "Get git diff".into(),
            parameters: json!({ "type": "object", "properties": { "staged": { "type": "boolean" }, "path": { "type": "string" } } }),
            permissions: vec!["read".into(), "git".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let staged = input.get("staged").and_then(|v| v.as_bool()).unwrap_or(false);
        let path = input.get("path").and_then(|v| v.as_str());
        let mut args = vec!["diff"];
        if staged { args.push("--staged"); }
        if let Some(p) = path { args.push(p); }
        let (stdout, stderr, success, code) = run_git(&args, &context.workspace_path).await?;
        Ok(json!({ "diff": stdout, "stderr": stderr, "success": success, "code": code }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::ReadOnly }
}

pub struct GitLogTool;
#[async_trait]
impl Tool for GitLogTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "git_log".into(),
            description: "Get git log".into(),
            parameters: json!({ "type": "object", "properties": { "limit": { "type": "number" } } }),
            permissions: vec!["read".into(), "git".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let limit = input.get("limit").and_then(|v| v.as_u64()).unwrap_or(20);
        let (stdout, stderr, success, code) = run_git(&["log", "--oneline", &format!("-{}", limit)], &context.workspace_path).await?;
        Ok(json!({ "log": stdout, "stderr": stderr, "success": success, "code": code }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::ReadOnly }
}

pub struct GitAddTool;
#[async_trait]
impl Tool for GitAddTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "git_add".into(),
            description: "Git add files".into(),
            parameters: json!({ "type": "object", "properties": { "paths": { "type": "array", "items": { "type": "string" } } }, "required": ["paths"] }),
            permissions: vec!["write".into(), "git".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let paths = input.get("paths").and_then(|v| v.as_array()).ok_or_else(|| ToolError::ExecutionFailed("missing paths".into()))?;
        let mut args = vec!["add"];
        let path_strs: Vec<String> = paths.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect();
        let arg_refs: Vec<&str> = path_strs.iter().map(|s| s.as_str()).collect();
        args.extend(arg_refs);
        let (stdout, stderr, success, code) = run_git(&args, &context.workspace_path).await?;
        Ok(json!({ "stdout": stdout, "stderr": stderr, "success": success, "code": code }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::WorkspaceWrite }
}

pub struct GitCommitTool;
#[async_trait]
impl Tool for GitCommitTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "git_commit".into(),
            description: "Create git commit".into(),
            parameters: json!({ "type": "object", "properties": { "message": { "type": "string" } }, "required": ["message"] }),
            permissions: vec!["write".into(), "git".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let message = input.get("message").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing message".into()))?;
        let (stdout, stderr, success, code) = run_git(&["commit", "-m", message], &context.workspace_path).await?;
        Ok(json!({ "stdout": stdout, "stderr": stderr, "success": success, "code": code }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::WorkspaceWrite }
}

pub struct GitBranchTool;
#[async_trait]
impl Tool for GitBranchTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "git_branch".into(),
            description: "List or create git branches".into(),
            parameters: json!({ "type": "object", "properties": { "action": { "type": "string" }, "name": { "type": "string" } } }),
            permissions: vec!["read".into(), "git".into()],
            timeout_ms: Some(10000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let action = input.get("action").and_then(|v| v.as_str()).unwrap_or("list");
        match action {
            "list" => {
                let (stdout, stderr, success, code) = run_git(&["branch", "-a"], &context.workspace_path).await?;
                Ok(json!({ "branches": stdout, "stderr": stderr, "success": success, "code": code }))
            }
            "create" => {
                let name = input.get("name").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing branch name".into()))?;
                let (stdout, stderr, success, code) = run_git(&["checkout", "-b", name], &context.workspace_path).await?;
                Ok(json!({ "stdout": stdout, "stderr": stderr, "success": success, "code": code }))
            }
            _ => Err(ToolError::ExecutionFailed(format!("Unknown action: {}", action))),
        }
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::WorkspaceWrite }
}
