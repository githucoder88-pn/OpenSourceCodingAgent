use async_trait::async_trait;
use serde_json::{Value, json};
use crate::{Tool, ToolContext, ToolError};
use forge_protocol::tools::ToolDefinition;
use forge_protocol::PermissionLevel;
use forge_search::Indexer;
use std::path::PathBuf;

pub struct SearchFilesTool;
#[async_trait]
impl Tool for SearchFilesTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "search_files".into(),
            description: "Search files for a query".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string" },
                    "limit": { "type": "number" }
                },
                "required": ["query"]
            }),
            permissions: vec!["read".into()],
            timeout_ms: Some(30000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let query = input.get("query").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing query".into()))?;
        let limit = input.get("limit").and_then(|v| v.as_u64()).unwrap_or(100) as usize;
        let indexer = Indexer::new(PathBuf::from(&context.workspace_path));
        let results = indexer.search(query).map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        let truncated: Vec<_> = results.into_iter().take(limit).collect();
        Ok(json!({
            "query": query,
            "count": truncated.len(),
            "results": truncated.iter().map(|r| json!({
                "path": r.path.to_string_lossy(),
                "line": r.line,
                "column": r.column,
                "content": r.content
            })).collect::<Vec<_>>()
        }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::ReadOnly }
}

pub struct SearchSymbolsTool;
#[async_trait]
impl Tool for SearchSymbolsTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "search_symbols".into(),
            description: "Search symbols (functions, classes, etc)".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string" }
                },
                "required": ["query"]
            }),
            permissions: vec!["read".into()],
            timeout_ms: Some(30000),
        }
    }
    async fn execute(&self, input: Value, context: ToolContext) -> Result<Value, ToolError> {
        let query = input.get("query").and_then(|v| v.as_str()).ok_or_else(|| ToolError::ExecutionFailed("missing query".into()))?;
        let indexer = Indexer::new(PathBuf::from(&context.workspace_path));
        let results = indexer.search_symbols(query).map_err(|e| ToolError::ExecutionFailed(e.to_string()))?;
        Ok(json!({
            "query": query,
            "count": results.len(),
            "results": results.iter().map(|s| json!({
                "name": s.name,
                "kind": format!("{:?}", s.kind),
                "path": s.path.to_string_lossy(),
                "line": s.line
            })).collect::<Vec<_>>()
        }))
    }
    fn requires_permission(&self) -> PermissionLevel { PermissionLevel::ReadOnly }
}
