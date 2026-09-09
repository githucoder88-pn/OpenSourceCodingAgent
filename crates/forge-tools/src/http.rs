use async_trait::async_trait;
use serde_json::{Value, json};
use crate::{Tool, ToolContext, ToolError};
use forge_protocol::tools::ToolDefinition;
use forge_protocol::PermissionLevel;

pub struct HttpRequestTool;

#[async_trait]
impl Tool for HttpRequestTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "http_request".into(),
            description: "Make HTTP request".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string" },
                    "method": { "type": "string" },
                    "headers": { "type": "object" },
                    "body": { "type": "string" }
                },
                "required": ["url"]
            }),
            permissions: vec!["network".into()],
            timeout_ms: Some(30000),
        }
    }

    async fn execute(&self, input: Value, _context: ToolContext) -> Result<Value, ToolError> {
        let url = input.get("url").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing url".into()))?;
        // For security, only allow http/https and block private networks in real impl
        // Here we stub
        Ok(json!({
            "url": url,
            "status": 200,
            "body": format!("Stub response for {}", url),
            "note": "Real HTTP implementation would use reqwest"
        }))
    }

    fn requires_permission(&self) -> PermissionLevel {
        PermissionLevel::FullWorkspace
    }
}
