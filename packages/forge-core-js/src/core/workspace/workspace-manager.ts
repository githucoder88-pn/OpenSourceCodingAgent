import * as fs from "fs";
import * as path from "path";
import { execSync } from "child_process";

export type WorkspaceType = 
  | { type: "main" }
  | { type: "agent"; id: string }
  | { type: "team"; id: string }
  | { type: "temporary" };

export interface Workspace {
  id: string;
  path: string;
  workspace_type: WorkspaceType;
  created_at: string;
}

export class WorkspaceManager {
  private mainWorkspace?: Workspace;
  private agentWorkspaces = new Map<string, Workspace>();

  setMain(projectPath: string): Workspace {
    const ws: Workspace = {
      id: crypto.randomUUID(),
      path: path.resolve(projectPath),
      workspace_type: { type: "main" },
      created_at: new Date().toISOString(),
    };
    this.mainWorkspace = ws;
    return ws;
  }

  getMain(): Workspace | undefined {
    return this.mainWorkspace;
  }

  createAgentWorkspace(agentId: string, base?: string): Workspace {
    const basePath = base || this.mainWorkspace?.path || process.cwd();
    const agentPath = path.join(basePath, ".forge", "workspaces", agentId);
    fs.mkdirSync(agentPath, { recursive: true });
    const ws: Workspace = {
      id: crypto.randomUUID(),
      path: agentPath,
      workspace_type: { type: "agent", id: agentId },
      created_at: new Date().toISOString(),
    };
    this.agentWorkspaces.set(agentId, ws);
    return ws;
  }

  getAgentWorkspace(agentId: string): Workspace | undefined {
    return this.agentWorkspaces.get(agentId);
  }

  listChangedFiles(workspacePath: string): string[] {
    try {
      const output = execSync("git status --porcelain", { cwd: workspacePath, encoding: "utf-8" });
      return output.split("\n").filter(Boolean).map(line => line.slice(3).trim());
    } catch {
      return [];
    }
  }

  listFiles(dir: string, maxDepth = 3, currentDepth = 0): string[] {
    if (currentDepth > maxDepth) return [];
    try {
      const entries = fs.readdirSync(dir, { withFileTypes: true });
      const files: string[] = [];
      for (const entry of entries) {
        if (entry.name.startsWith(".") || ["node_modules", "target", "dist", "build"].includes(entry.name)) continue;
        const full = path.join(dir, entry.name);
        if (entry.isDirectory()) {
          files.push(...this.listFiles(full, maxDepth, currentDepth + 1));
        } else {
          files.push(full);
        }
      }
      return files;
    } catch {
      return [];
    }
  }
}
