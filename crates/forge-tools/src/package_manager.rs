use async_trait::async_trait;
use serde_json::{Value, json};
use crate::{Tool, ToolContext, ToolError, shell::ShellTool};
use forge_protocol::tools::ToolDefinition;
use forge_protocol::PermissionLevel;

pub struct PackageManagerTool;

#[async_trait]
impl Tool for PackageManagerTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "package_manager".into(),
            description: "Run package manager commands (npm, cargo, pip, etc)".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "manager": { "type": "string", "enum": ["npm", "yarn", "pnpm", "cargo", "pip", "poetry"] },
                    "command": { "type": "string" },
                    "args": { "type": "array", "items": { "type": "string" } }
                },
                "required": ["manager", "command"]
            }),
            permissions: vec!["shell".into()],
            timeout_ms: Some(120000),
        }
    }

    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let manager = input.get("manager").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing manager".into()))?;
        let command = input.get("command").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing command".into()))?;
        let args = input.get("args").and_then(|v| v.as_array()).map(|arr| {
            arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<_>>().join(" ")
        }).unwrap_or_default();

        let full_command = format!("{} {} {}", manager, command, args);
        let shell = ShellTool;
        shell.execute(json!({ "command": full_command, "cwd": context.workspace_path }), context).await
    }

    fn requires_permission(&self) -> PermissionLevel {
        PermissionLevel::FullWorkspace
    }
}
