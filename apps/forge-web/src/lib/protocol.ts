export interface ForgeEvent {
  id: string;
  timestamp: string;
  session_id?: string;
  payload: {
    type: string;
    data: any;
  };
}

export class ForgeClient {
  private ws?: WebSocket;
  private eventHandlers = new Map<string, Set<(event: ForgeEvent) => void>>();
  private allHandlers = new Set<(event: ForgeEvent) => void>();

  constructor(private baseUrl: string = "http://localhost:3000") {}

  async fetchApi(path: string, options?: RequestInit) {
    const res = await fetch(`${this.baseUrl}${path}`, {
      ...options,
      headers: {
        "Content-Type": "application/json",
        ...options?.headers,
      },
    });
    if (!res.ok) throw new Error(`API error: ${res.status}`);
    return res.json();
  }

  async listSessions() {
    return this.fetchApi("/api/sessions");
  }

  async createSession(name: string, project_path: string) {
    return this.fetchApi("/api/sessions", {
      method: "POST",
      body: JSON.stringify({ name, project_path }),
    });
  }

  async listAgents(sessionId: string) {
    return this.fetchApi(`/api/sessions/${sessionId}/agents`);
  }

  async listTasks(sessionId: string) {
    return this.fetchApi(`/api/sessions/${sessionId}/tasks`);
  }

  async listTools() {
    return this.fetchApi("/api/tools");
  }

  async getModels() {
    return this.fetchApi("/api/models");
  }

  connectWebSocket(): WebSocket {
    const wsUrl = this.baseUrl.replace("http", "ws") + "/ws";
    this.ws = new WebSocket(wsUrl);

    this.ws.onmessage = (event) => {
      try {
        const forgeEvent: ForgeEvent = JSON.parse(event.data);
        // Call all handlers
        this.allHandlers.forEach(h => h(forgeEvent));
        // Call type-specific handlers
        const handlers = this.eventHandlers.get(forgeEvent.payload.type);
        if (handlers) {
          handlers.forEach(h => h(forgeEvent));
        }
      } catch (e) {
        console.error("Failed to parse WS event", e);
      }
    };

    this.ws.onopen = () => console.log("Forge WS connected");
    this.ws.onclose = () => console.log("Forge WS disconnected");
    this.ws.onerror = (e) => console.error("Forge WS error", e);

    return this.ws;
  }

  onEvent(type: string, handler: (event: ForgeEvent) => void) {
    if (!this.eventHandlers.has(type)) {
      this.eventHandlers.set(type, new Set());
    }
    this.eventHandlers.get(type)!.add(handler);
    return () => this.eventHandlers.get(type)?.delete(handler);
  }

  onAllEvents(handler: (event: ForgeEvent) => void) {
    this.allHandlers.add(handler);
    return () => this.allHandlers.delete(handler);
  }

  disconnect() {
    this.ws?.close();
  }
}

export const forgeClient = new ForgeClient();
