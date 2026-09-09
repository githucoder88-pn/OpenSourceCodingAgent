use forge_protocol::{PermissionLevel, AgentId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionPolicy {
    pub level: PermissionLevel,
    pub approval_policy: forge_protocol::ApprovalPolicy,
    pub allowed_tools: Option<Vec<String>>,
    pub denied_commands: Vec<String>,
}

impl Default for PermissionPolicy {
    fn default() -> Self {
        Self {
            level: PermissionLevel::WorkspaceWrite,
            approval_policy: forge_protocol::ApprovalPolicy::OnRiskyCommands,
            allowed_tools: None,
            denied_commands: vec!["rm -rf /".into(), "mkfs".into(), "dd if=".into()],
        }
    }
}

impl PermissionPolicy {
    pub fn can_execute(&self, tool: &str, command: Option<&str>) -> Result<(), String> {
        if let Some(allowed) = &self.allowed_tools {
            if !allowed.contains(&tool.to_string()) {
                return Err(format!("Tool {} not allowed by policy", tool));
            }
        }

        if let Some(cmd) = command {
            for denied in &self.denied_commands {
                if cmd.contains(denied) {
                    return Err(format!("Command contains denied pattern: {}", denied));
                }
            }

            // Check permission level
            match self.level {
                PermissionLevel::ReadOnly => {
                    if tool != "read_file" && tool != "list_directory" && tool != "search_files" && tool != "git_status" && tool != "git_diff" && tool != "git_log" {
                        return Err(format!("Tool {} requires write permission, current level is read-only", tool));
                    }
                }
                PermissionLevel::WorkspaceWrite => {
                    if tool == "shell" {
                        // Allow but will require approval for risky
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn requires_approval(&self, tool: &str, command: Option<&str>) -> bool {
        match self.approval_policy {
            forge_protocol::ApprovalPolicy::Always => true,
            forge_protocol::ApprovalPolicy::Never => false,
            forge_protocol::ApprovalPolicy::OnNewCommand => {
                // Simplified: always require for shell
                tool == "shell"
            }
            forge_protocol::ApprovalPolicy::OnRiskyCommands => {
                if let Some(cmd) = command {
                    let risky = ["rm -rf", "sudo", "chmod", "chown", "git push", "git reset --hard", "npm publish", "cargo publish"];
                    risky.iter().any(|r| cmd.contains(r))
                } else {
                    false
                }
            }
        }
    }
}

pub struct PermissionManager {
    policies: std::collections::HashMap<AgentId, PermissionPolicy>,
    default_policy: PermissionPolicy,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self { policies: std::collections::HashMap::new(), default_policy: PermissionPolicy::default() }
    }

    pub fn set_policy(&mut self, agent_id: AgentId, policy: PermissionPolicy) {
        self.policies.insert(agent_id, policy);
    }

    pub fn get_policy(&self, agent_id: AgentId) -> &PermissionPolicy {
        self.policies.get(&agent_id).unwrap_or(&self.default_policy)
    }

    pub fn set_default(&mut self, policy: PermissionPolicy) {
        self.default_policy = policy;
    }
}

impl Default for PermissionManager {
    fn default() -> Self { Self::new() }
}
