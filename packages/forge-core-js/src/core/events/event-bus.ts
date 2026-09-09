import { ForgeEvent } from "../types.js";
import { EventEmitter } from "events";

export class EventBus extends EventEmitter {
  private history: ForgeEvent[] = [];
  private maxHistory = 10000;

  constructor() {
    super();
    this.setMaxListeners(100);
  }

  publish(event: ForgeEvent) {
    this.history.push(event);
    if (this.history.length > this.maxHistory) {
      this.history.shift();
    }
    this.emit("event", event);
    this.emit(event.payload.type, event);
  }

  subscribe(callback: (event: ForgeEvent) => void): () => void {
    this.on("event", callback);
    return () => this.off("event", callback);
  }

  subscribeTo(type: string, callback: (event: ForgeEvent) => void): () => void {
    this.on(type, callback);
    return () => this.off(type, callback);
  }

  getHistory(sessionId?: string, limit = 100): ForgeEvent[] {
    let filtered = this.history;
    if (sessionId) {
      filtered = filtered.filter(e => e.session_id === sessionId);
    }
    return filtered.slice(-limit);
  }

  clear() {
    this.history = [];
  }
}

export function createEvent(sessionId: string | undefined, payload: ForgeEvent["payload"]): ForgeEvent {
  return {
    id: crypto.randomUUID(),
    timestamp: new Date().toISOString(),
    session_id: sessionId,
    payload,
  };
}
