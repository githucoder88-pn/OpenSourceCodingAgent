import { Agent, AgentState, AgentRole, PermissionLevel, SessionId, AgentId, TaskId, TeamId } from "../types.js";
import { EventBus, createEvent } from "../events/event-bus.js";

export class AgentRuntime {
  private agents = new Map<AgentId, Agent>();
  constructor(private eventBus: EventBus) {}

  createAgent(params: {
    session_id: SessionId;
    name: string;
    role: AgentRole;
    model?: string;
    workspace: string;
    team_id?: TeamId;
    parent_id?: AgentId;
    permissions?: PermissionLevel;
  }): Agent {
    const now = new Date().toISOString();
    const agent: Agent = {
      id: crypto.randomUUID(),
      session_id: params.session_id,
      name: params.name,
      role: params.role,
      state: AgentState.Created,
      progress: 0,
      team_id: params.team_id,
      parent_id: params.parent_id,
      children: [],
      model: params.model || "gpt-4o",
      permissions: params.permissions || PermissionLevel.WorkspaceWrite,
      workspace: params.workspace,
      metrics: {
        tokens_used: 0,
        requests: 0,
        tool_calls: 0,
        files_changed: 0,
        latency_ms: 0,
        started_at: now,
      },
      created_at: now,
      updated_at: now,
      files_changed: [],
      tools_used: [],
      is_paused: false,
    };

    this.agents.set(agent.id, agent);

    if (params.parent_id) {
      const parent = this.agents.get(params.parent_id);
      if (parent) {
        parent.children.push(agent.id);
      }
    }

    this.eventBus.publish(createEvent(agent.session_id, {
      type: "agent.created",
      data: { agent_id: agent.id, name: agent.name, role: agent.role }
    }));

    return agent;
  }

  getAgent(id: AgentId): Agent | undefined {
    return this.agents.get(id);
  }

  listAgents(sessionId: SessionId): Agent[] {
    return Array.from(this.agents.values()).filter(a => a.session_id === sessionId);
  }

  updateAgent(id: AgentId, updater: (agent: Agent) => void): Agent | undefined {
    const agent = this.agents.get(id);
    if (agent) {
      updater(agent);
      agent.updated_at = new Date().toISOString();
      return agent;
    }
    return undefined;
  }

  setState(id: AgentId, state: AgentState) {
    const agent = this.agents.get(id);
    if (agent) {
      agent.state = state;
      agent.updated_at = new Date().toISOString();
      const eventType = state === AgentState.Paused ? "agent.paused" :
                        state === AgentState.Completed ? "agent.completed" :
                        state === AgentState.Failed ? "agent.failed" :
                        state === AgentState.Executing ? "agent.started" : null;
      if (eventType) {
        if (eventType === "agent.failed") {
          this.eventBus.publish(createEvent(agent.session_id, { type: "agent.failed", data: { agent_id: id, error: "Failed" } } as any));
        } else {
          this.eventBus.publish(createEvent(agent.session_id, { type: eventType as any, data: { agent_id: id } } as any));
        }
      }
    }
  }

  setProgress(id: AgentId, progress: number, message?: string) {
    const agent = this.agents.get(id);
    if (agent) {
      agent.progress = Math.max(0, Math.min(100, progress));
      if (message) agent.current_action = message;
      agent.updated_at = new Date().toISOString();
      this.eventBus.publish(createEvent(agent.session_id, {
        type: "agent.progress",
        data: { agent_id: id, progress: agent.progress, message: message || agent.current_action || "" }
      }));
    }
  }

  pauseAgent(id: AgentId): boolean {
    const agent = this.agents.get(id);
    if (!agent) return false;
    agent.is_paused = true;
    this.setState(id, AgentState.Paused);
    return true;
  }

  resumeAgent(id: AgentId): boolean {
    const agent = this.agents.get(id);
    if (!agent) return false;
    agent.is_paused = false;
    this.setState(id, AgentState.Idle);
    this.eventBus.publish(createEvent(agent.session_id, { type: "agent.resumed", data: { agent_id: id } }));
    return true;
  }

  stopAgent(id: AgentId): boolean {
    const agent = this.agents.get(id);
    if (!agent) return false;
    this.setState(id, AgentState.Cancelled);
    return true;
  }

  spawnSubagent(parentId: AgentId, name: string, role: AgentRole): Agent | undefined {
    const parent = this.agents.get(parentId);
    if (!parent) return undefined;
    return this.createAgent({
      session_id: parent.session_id,
      name,
      role,
      model: parent.model,
      workspace: parent.workspace,
      team_id: parent.team_id,
      parent_id: parentId,
      permissions: parent.permissions,
    });
  }

  assignTask(agentId: AgentId, taskId: TaskId) {
    const agent = this.agents.get(agentId);
    if (agent) {
      agent.current_task = taskId;
      agent.state = AgentState.Executing;
      agent.updated_at = new Date().toISOString();
    }
  }

  recordFileChange(agentId: AgentId, filePath: string) {
    const agent = this.agents.get(agentId);
    if (agent) {
      if (!agent.files_changed.includes(filePath)) {
        agent.files_changed.push(filePath);
        agent.metrics.files_changed++;
      }
    }
  }

  recordToolUse(agentId: AgentId, toolName: string) {
    const agent = this.agents.get(agentId);
    if (agent) {
      if (!agent.tools_used.includes(toolName)) {
        agent.tools_used.push(toolName);
      }
      agent.metrics.tool_calls++;
    }
  }
}
