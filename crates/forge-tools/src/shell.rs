use async_trait::async_trait;
use serde_json::{Value, json};
use crate::{Tool, ToolContext, ToolError};
use forge_protocol::tools::ToolDefinition;
use forge_protocol::PermissionLevel;
use tokio::process::Command;
use std::time::Instant;

pub struct ShellTool;

#[async_trait]
impl Tool for ShellTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "shell".into(),
            description: "Execute a shell command with streaming output".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "command": { "type": "string", "description": "Command to execute" },
                    "cwd": { "type": "string" },
                    "timeout_ms": { "type": "number" }
                },
                "required": ["command"]
            }),
            permissions: vec!["shell".into()],
            timeout_ms: Some(120000),
        }
    }

    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let command = input.get("command").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing command".into()))?;
        let cwd = input.get("cwd").and_then(|v| v.as_str()).unwrap_or(&context.workspace_path);
        let timeout_ms = input.get("timeout_ms").and_then(|v| v.as_u64()).unwrap_or(120000);

        // Dangerous command detection
        let dangerous_patterns = ["rm -rf /", "rm -rf ~", ":(){:|:&};:", "mkfs", "dd if=", "> /dev/sda", "format", "shutdown", "reboot"];
        for pattern in &dangerous_patterns {
            if command.contains(pattern) {
                return Err(ToolError::PermissionDenied(format!("Dangerous command detected: contains '{}'", pattern)));
            }
        }

        let start = Instant::now();
        let mut cmd = if cfg!(target_os = "windows") {
            let mut c = Command::new("cmd");
            c.arg("/C").arg(command);
            c
        } else {
            let mut c = Command::new("sh");
            c.arg("-c").arg(command);
            c
        };
        cmd.current_dir(cwd);

        let output = tokio::time::timeout(
            std::time::Duration::from_millis(timeout_ms),
            cmd.output()
        ).await.map_err(|_| ToolError::Timeout(format!("Command timed out after {}ms", timeout_ms)))?
        .map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;

        let duration = start.elapsed().as_millis() as u64;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok(json!({
            "command": command,
            "cwd": cwd,
            "exit_code": output.status.code(),
            "success": output.status.success(),
            "stdout": stdout,
            "stderr": stderr,
            "duration_ms": duration
        }))
    }

    fn requires_permission(&self) -> PermissionLevel {
        PermissionLevel::FullWorkspace
    }
}

pub struct RunTestsTool;
#[async_trait]
impl Tool for RunTestsTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "run_tests".into(),
            description: "Run tests in the workspace".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "test_command": { "type": "string" },
                    "cwd": { "type": "string" }
                },
                "required": []
            }),
            permissions: vec!["shell".into(), "test".into()],
            timeout_ms: Some(300000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let test_cmd = input.get("test_command").and_then(|v| v.as_str()).unwrap_or_else(|| {
            if std::path::Path::new(&format!("{}/Cargo.toml", context.workspace_path)).exists() {
                "cargo test"
            } else if std::path::Path::new(&format!("{}/package.json", context.workspace_path)).exists() {
                "npm test"
            } else {
                "pytest"
            }
        });
        let shell = ShellTool;
        shell.execute(json!({ "command": test_cmd, "cwd": context.workspace_path }), context).await
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::FullWorkspace }
}

pub struct RunBuildTool;
#[async_trait]
impl Tool for RunBuildTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "run_build".into(),
            description: "Run build in workspace".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "build_command": { "type": "string" }
                },
                "required": []
            }),
            permissions: vec!["shell".into()],
            timeout_ms: Some(300000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let build_cmd = input.get("build_command").and_then(|v| v.as_str()).unwrap_or_else(|| {
            if std::path::Path::new(&format!("{}/Cargo.toml", context.workspace_path)).exists() { "cargo build" }
            else if std::path::Path::new(&format!("{}/package.json", context.workspace_path)).exists() { "npm run build" }
            else { "make" }
        });
        let shell = ShellTool;
        shell.execute(json!({ "command": build_cmd, "cwd": context.workspace_path }), context).await
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::FullWorkspace }
}

pub struct RunLinterTool;
#[async_trait]
impl Tool for RunLinterTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "run_linter".into(),
            description: "Run linter".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "linter_command": { "type": "string" }
                },
                "required": []
            }),
            permissions: vec!["shell".into()],
            timeout_ms: Some(120000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let linter_cmd = input.get("linter_command").and_then(|v| v.as_str()).unwrap_or("cargo clippy || npm run lint || echo 'no linter configured'");
        let shell = ShellTool;
        shell.execute(json!({ "command": linter_cmd, "cwd": context.workspace_path }), context).await
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::FullWorkspace }
}
