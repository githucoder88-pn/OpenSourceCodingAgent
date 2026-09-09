import { Session, SessionId, AgentId, TaskId } from "../types.js";
import { EventBus, createEvent } from "../events/event-bus.js";

export class SessionManager {
  private sessions = new Map<SessionId, Session>();

  constructor(private eventBus: EventBus) {}

  createSession(name: string, projectPath: string): Session {
    const now = new Date().toISOString();
    const session: Session = {
      id: crypto.randomUUID(),
      name,
      project_path: projectPath,
      created_at: now,
      updated_at: now,
      agents: [],
      tasks: [],
      status: "Active",
    };
    this.sessions.set(session.id, session);
    this.eventBus.publish(createEvent(session.id, {
      type: "session.created",
      data: { session_id: session.id, project_path: projectPath }
    }));
    return session;
  }

  getSession(id: SessionId): Session | undefined {
    return this.sessions.get(id);
  }

  listSessions(): Session[] {
    return Array.from(this.sessions.values()).sort((a, b) => 
      new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime()
    );
  }

  resumeSession(id: SessionId): Session | undefined {
    const session = this.sessions.get(id);
    if (session) {
      session.status = "Active";
      session.updated_at = new Date().toISOString();
      this.eventBus.publish(createEvent(session.id, {
        type: "session.resumed",
        data: { session_id: id }
      }));
    }
    return session;
  }

  addAgent(sessionId: SessionId, agentId: AgentId) {
    const session = this.sessions.get(sessionId);
    if (session) {
      if (!session.agents.includes(agentId)) {
        session.agents.push(agentId);
        session.updated_at = new Date().toISOString();
      }
    }
  }

  addTask(sessionId: SessionId, taskId: TaskId) {
    const session = this.sessions.get(sessionId);
    if (session) {
      if (!session.tasks.includes(taskId)) {
        session.tasks.push(taskId);
        session.updated_at = new Date().toISOString();
      }
    }
  }
}
