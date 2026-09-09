import { AgentId } from "../types.js";

export enum PermissionLevel {
  ReadOnly = "read-only",
  WorkspaceWrite = "workspace-write",
  FullWorkspace = "full-workspace",
  Unrestricted = "unrestricted",
}

export enum ApprovalPolicy {
  Always = "always",
  OnRiskyCommands = "on-risky-commands",
  OnNewCommand = "on-new-command",
  Never = "never",
}

export interface PermissionPolicy {
  level: PermissionLevel;
  approval_policy: ApprovalPolicy;
  allowed_tools?: string[];
  denied_commands: string[];
}

export const DEFAULT_POLICY: PermissionPolicy = {
  level: PermissionLevel.WorkspaceWrite,
  approval_policy: ApprovalPolicy.OnRiskyCommands,
  denied_commands: ["rm -rf /", "mkfs", "dd if=", ":(){:|:&};:"],
};

export class PermissionManager {
  private policies = new Map<AgentId, PermissionPolicy>();
  private defaultPolicy: PermissionPolicy = DEFAULT_POLICY;

  setPolicy(agentId: AgentId, policy: PermissionPolicy) {
    this.policies.set(agentId, policy);
  }

  getPolicy(agentId: AgentId): PermissionPolicy {
    return this.policies.get(agentId) || this.defaultPolicy;
  }

  setDefault(policy: PermissionPolicy) {
    this.defaultPolicy = policy;
  }

  canExecute(agentId: AgentId, tool: string, command?: string): { allowed: boolean; reason?: string } {
    const policy = this.getPolicy(agentId);

    if (policy.allowed_tools && !policy.allowed_tools.includes(tool)) {
      return { allowed: false, reason: `Tool ${tool} not allowed` };
    }

    if (command) {
      for (const denied of policy.denied_commands) {
        if (command.includes(denied)) {
          return { allowed: false, reason: `Command contains denied pattern: ${denied}` };
        }
      }

      if (policy.level === PermissionLevel.ReadOnly) {
        const readOnlyTools = ["read_file", "list_directory", "search_files", "git_status", "git_diff", "git_log"];
        if (!readOnlyTools.includes(tool)) {
          return { allowed: false, reason: `Tool ${tool} requires write permission` };
        }
      }
    }

    return { allowed: true };
  }

  requiresApproval(agentId: AgentId, tool: string, command?: string): boolean {
    const policy = this.getPolicy(agentId);
    switch (policy.approval_policy) {
      case ApprovalPolicy.Always: return true;
      case ApprovalPolicy.Never: return false;
      case ApprovalPolicy.OnNewCommand: return tool === "shell";
      case ApprovalPolicy.OnRiskyCommands:
        if (!command) return false;
        const risky = ["rm -rf", "sudo", "chmod", "chown", "git push", "git reset --hard", "npm publish", "cargo publish"];
        return risky.some(r => command.includes(r));
      default: return false;
    }
  }
}
