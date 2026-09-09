import { execSync } from "child_process";

export interface Checkpoint {
  id: string;
  session_id: string;
  message?: string;
  created_at: string;
  git_commit?: string;
  workspace_state: {
    branch?: string;
    changed_files: string[];
  };
  task_state: any;
  agent_state: any;
  memory_state: any;
  event_position: number;
}

export class CheckpointManager {
  private checkpoints = new Map<string, Checkpoint>();
  private sessionCheckpoints = new Map<string, Set<string>>();

  createCheckpoint(sessionId: string, projectPath: string, message?: string): Checkpoint {
    let gitCommit: string | undefined;
    let branch: string | undefined;

    try {
      gitCommit = execSync("git rev-parse HEAD", { cwd: projectPath, encoding: "utf-8" }).trim();
    } catch {}
    try {
      branch = execSync("git branch --show-current", { cwd: projectPath, encoding: "utf-8" }).trim();
    } catch {}

    const cp: Checkpoint = {
      id: crypto.randomUUID(),
      session_id: sessionId,
      message,
      created_at: new Date().toISOString(),
      git_commit: gitCommit,
      workspace_state: { branch, changed_files: [] },
      task_state: {},
      agent_state: {},
      memory_state: {},
      event_position: 0,
    };

    this.checkpoints.set(cp.id, cp);
    if (!this.sessionCheckpoints.has(sessionId)) {
      this.sessionCheckpoints.set(sessionId, new Set());
    }
    this.sessionCheckpoints.get(sessionId)!.add(cp.id);

    return cp;
  }

  listCheckpoints(sessionId: string): Checkpoint[] {
    const ids = this.sessionCheckpoints.get(sessionId);
    if (!ids) return [];
    return Array.from(ids).map(id => this.checkpoints.get(id)!).filter(Boolean).sort((a, b) => 
      new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
    );
  }

  getCheckpoint(id: string): Checkpoint | undefined {
    return this.checkpoints.get(id);
  }

  restoreCheckpoint(id: string, projectPath: string): boolean {
    const cp = this.checkpoints.get(id);
    if (!cp || !cp.git_commit) return false;
    try {
      execSync(`git checkout ${cp.git_commit}`, { cwd: projectPath });
      return true;
    } catch {
      return false;
    }
  }
}
