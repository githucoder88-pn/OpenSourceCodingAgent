export interface JsonRpcRequest {
  jsonrpc: string;
  id: any;
  method: string;
  params?: any;
}

export interface JsonRpcResponse {
  jsonrpc: string;
  id: any;
  result?: any;
  error?: { code: number; message: string; data?: any };
}

export class ForgeProtocolClient {
  private ws?: WebSocket;
  private pending = new Map<any, { resolve: (v: any) => void; reject: (e: any) => void }>();
  private nextId = 1;

  constructor(private url: string = "ws://localhost:3000/ws") {}

  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      this.ws = new WebSocket(this.url);
      this.ws.onopen = () => resolve();
      this.ws.onerror = (e) => reject(e);
      this.ws.onmessage = (event) => {
        try {
          const msg = JSON.parse(event.data);
          if (msg.id && this.pending.has(msg.id)) {
            const { resolve } = this.pending.get(msg.id)!;
            this.pending.delete(msg.id);
            resolve(msg);
          }
        } catch {}
      };
    });
  }

  async call(method: string, params?: any): Promise<any> {
    const id = this.nextId++;
    const req: JsonRpcRequest = { jsonrpc: "2.0", id, method, params };
    
    // Try HTTP fallback if WS not connected
    if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
      const res = await fetch(this.url.replace("ws://", "http://").replace("/ws", "/api/rpc"), {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(req),
      });
      const json = await res.json();
      return json.result;
    }

    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.ws!.send(JSON.stringify(req));
      setTimeout(() => {
        if (this.pending.has(id)) {
          this.pending.delete(id);
          reject(new Error("RPC timeout"));
        }
      }, 10000);
    });
  }

  // Convenience methods
  createSession(project_path: string, name?: string) {
    return this.call("create_session", { project_path, name });
  }

  listSessions() {
    return this.call("list_sessions");
  }

  listAgents(session_id: string) {
    return this.call("list_agents", { session_id });
  }

  createAgent(session_id: string, name: string, role: string, model?: string) {
    return this.call("create_agent", { session_id, name, role, model });
  }

  listTasks(session_id: string) {
    return this.call("list_tasks", { session_id });
  }

  createTask(session_id: string, title: string, description?: string) {
    return this.call("create_task", { session_id, title, description });
  }

  listTeams(session_id: string) {
    return this.call("list_teams", { session_id });
  }

  sendAgentMessage(from: string, to: string | undefined, subject: string, body: string) {
    return this.call("send_agent_message", { from, to, subject, body, message_type: "request" });
  }
}
