import { EventBus, createEvent } from "../events/event-bus.js";

export interface AgentMessage {
  id: string;
  from: string;
  to?: string;
  team_id?: string;
  task_id?: string;
  message_type: "request" | "response" | "question" | "answer" | "status" | "handoff" | "warning" | "blocked" | "approval" | "broadcast";
  subject: string;
  body: string;
  timestamp: string;
}

export class MessageBus {
  private messages = new Map<string, AgentMessage>();

  constructor(private eventBus: EventBus) {}

  sendMessage(params: {
    from: string;
    to?: string;
    team_id?: string;
    task_id?: string;
    message_type: AgentMessage["message_type"];
    subject: string;
    body: string;
  }): AgentMessage {
    const msg: AgentMessage = {
      id: crypto.randomUUID(),
      from: params.from,
      to: params.to,
      team_id: params.team_id,
      task_id: params.task_id,
      message_type: params.message_type,
      subject: params.subject,
      body: params.body,
      timestamp: new Date().toISOString(),
    };

    this.messages.set(msg.id, msg);

    this.eventBus.publish(createEvent(undefined, {
      type: "agent.message.sent",
      data: { message_id: msg.id, from: msg.from as any, to: msg.to as any, team_id: msg.team_id as any, body: msg.body }
    }));

    if (msg.to) {
      this.eventBus.publish(createEvent(undefined, {
        type: "agent.message.received",
        data: { message_id: msg.id, to: msg.to as any, from: msg.from as any, body: msg.body }
      }));
    }

    return msg;
  }

  listMessages(filter?: { agent_id?: string; team_id?: string }): AgentMessage[] {
    let msgs = Array.from(this.messages.values());
    if (filter?.agent_id) {
      msgs = msgs.filter(m => m.from === filter.agent_id || m.to === filter.agent_id);
    }
    if (filter?.team_id) {
      msgs = msgs.filter(m => m.team_id === filter.team_id);
    }
    return msgs.sort((a, b) => new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime());
  }

  getMessage(id: string): AgentMessage | undefined {
    return this.messages.get(id);
  }
}
