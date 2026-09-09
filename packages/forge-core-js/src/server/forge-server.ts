import { EventBus } from "../core/events/event-bus.js";
import { AgentRuntime } from "../core/agent/agent-runtime.js";
import { SessionManager } from "../core/sessions/session-manager.js";
import { TaskGraphManager } from "../core/task_graph/task-manager.js";
import { TeamManager } from "../core/teams/team-manager.js";
import { MessageBus } from "../core/messaging/message-bus.js";
import { WorkspaceManager } from "../core/workspace/workspace-manager.js";
import { PermissionManager } from "../core/permissions/permission-manager.js";
import { CheckpointManager } from "../core/checkpoints/checkpoint-manager.js";
import { ToolRegistry } from "../core/tools/tool-registry.js";
import { ContextEngine } from "../core/context/context-engine.js";
import { MemoryStore } from "../core/memory/memory-store.js";
import { ModelRouter, DEFAULT_MODELS, ProviderStatus } from "../core/models/model-router.js";
import * as http from "http";
import * as fs from "fs";
import * as path from "path";
import { WebSocketServer, WebSocket } from "ws";

export class ForgeServer {
  public eventBus: EventBus;
  public agentRuntime: AgentRuntime;
  public sessionManager: SessionManager;
  public taskManager: TaskGraphManager;
  public teamManager: TeamManager;
  public messageBus: MessageBus;
  public workspaceManager: WorkspaceManager;
  public permissionManager: PermissionManager;
  public checkpointManager: CheckpointManager;
  public toolRegistry: ToolRegistry;
  public contextEngine: ContextEngine;
  public memoryStore: MemoryStore;
  public modelRouter: ModelRouter;
  public providerStatus: Map<string, ProviderStatus>;

  private httpServer?: http.Server;
  private wss?: WebSocketServer;
  private clients = new Set<WebSocket>();

  constructor() {
    this.eventBus = new EventBus();
    this.agentRuntime = new AgentRuntime(this.eventBus);
    this.sessionManager = new SessionManager(this.eventBus);
    this.taskManager = new TaskGraphManager(this.eventBus);
    this.teamManager = new TeamManager();
    this.messageBus = new MessageBus(this.eventBus);
    this.workspaceManager = new WorkspaceManager();
    this.permissionManager = new PermissionManager();
    this.checkpointManager = new CheckpointManager();
    this.toolRegistry = new ToolRegistry();
    this.contextEngine = new ContextEngine();
    this.memoryStore = new MemoryStore();
    this.modelRouter = new ModelRouter("adaptive", true);
    this.providerStatus = new Map([
      ["openai", { provider: "openai", health: "offline", error_rate: 0, last_check: new Date().toISOString() }],
      ["anthropic", { provider: "anthropic", health: "offline", error_rate: 0, last_check: new Date().toISOString() }],
      ["ollama", { provider: "ollama", health: "available", error_rate: 0, last_check: new Date().toISOString(), latency_ms: 50 }],
    ]);

    // Forward events to WS clients
    this.eventBus.subscribe((event) => {
      const msg = JSON.stringify(event);
      for (const client of this.clients) {
        if (client.readyState === WebSocket.OPEN) {
          client.send(msg);
        }
      }
    });
  }

  async start(port = 3000, host = "0.0.0.0"): Promise<void> {
    return new Promise((resolve) => {
      this.httpServer = http.createServer((req, res) => {
        this.handleRequest(req, res);
      });

      this.wss = new WebSocketServer({ server: this.httpServer, path: "/ws" });

      this.wss.on("connection", (ws) => {
        this.clients.add(ws);
        console.log("Client connected, total:", this.clients.size);

        // Send history
        const history = this.eventBus.getHistory(undefined, 50);
        for (const event of history) {
          ws.send(JSON.stringify(event));
        }

        ws.on("close", () => {
          this.clients.delete(ws);
        });

        ws.on("message", (data) => {
          try {
            const msg = JSON.parse(data.toString());
            this.handleWsMessage(ws, msg);
          } catch (e) {
            console.error("WS message error:", e);
          }
        });
      });

      this.httpServer.listen(port, host, () => {
        console.log(`Forge Core listening on http://${host}:${port}`);
        console.log(`WebSocket at ws://${host}:${port}/ws`);
        resolve();
      });
    });
  }

  private handleRequest(req: http.IncomingMessage, res: http.ServerResponse) {
    const url = new URL(req.url || "/", `http://${req.headers.host}`);
    
    // CORS
    res.setHeader("Access-Control-Allow-Origin", "*");
    res.setHeader("Access-Control-Allow-Methods", "GET, POST, OPTIONS");
    res.setHeader("Access-Control-Allow-Headers", "Content-Type");

    if (req.method === "OPTIONS") {
      res.writeHead(204);
      res.end();
      return;
    }

    if (url.pathname === "/" || url.pathname === "/api/health") {
      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ name: "Forge Core", version: "0.1.0", status: "running", timestamp: new Date().toISOString() }));
      return;
    }

    if (url.pathname === "/api/sessions" && req.method === "GET") {
      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(JSON.stringify(this.sessionManager.listSessions()));
      return;
    }

    if (url.pathname === "/api/sessions" && req.method === "POST") {
      let body = "";
      req.on("data", chunk => body += chunk);
      req.on("end", () => {
        try {
          const params = JSON.parse(body);
          const session = this.sessionManager.createSession(params.name || "Untitled", params.project_path || ".");
          this.workspaceManager.setMain(session.project_path);
          res.writeHead(201, { "Content-Type": "application/json" });
          res.end(JSON.stringify(session));
        } catch (e: any) {
          res.writeHead(400, { "Content-Type": "application/json" });
          res.end(JSON.stringify({ error: e.message }));
        }
      });
      return;
    }

    if (url.pathname.startsWith("/api/sessions/") && url.pathname.endsWith("/agents")) {
      const sessionId = url.pathname.split("/")[3];
      const agents = this.agentRuntime.listAgents(sessionId);
      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(JSON.stringify(agents));
      return;
    }

    if (url.pathname.startsWith("/api/sessions/") && url.pathname.endsWith("/tasks")) {
      const sessionId = url.pathname.split("/")[3];
      const tasks = this.taskManager.listTasks(sessionId);
      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(JSON.stringify(tasks));
      return;
    }

    if (url.pathname === "/api/tools") {
      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(JSON.stringify(this.toolRegistry.list()));
      return;
    }

    if (url.pathname === "/api/models") {
      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ models: DEFAULT_MODELS, providers: Array.from(this.providerStatus.values()) }));
      return;
    }

    if (url.pathname === "/api/rpc" && req.method === "POST") {
      let body = "";
      req.on("data", chunk => body += chunk);
      req.on("end", () => {
        this.handleRpc(body, res);
      });
      return;
    }

    // Serve static web app if exists
    const webDist = path.join(process.cwd(), "apps/forge-web/dist");
    if (fs.existsSync(webDist)) {
      let filePath = path.join(webDist, url.pathname === "/" ? "index.html" : url.pathname);
      if (!fs.existsSync(filePath) || fs.statSync(filePath).isDirectory()) {
        filePath = path.join(webDist, "index.html");
      }
      if (fs.existsSync(filePath)) {
        const ext = path.extname(filePath);
        const contentType: Record<string, string> = {
          ".html": "text/html",
          ".js": "application/javascript",
          ".css": "text/css",
          ".json": "application/json",
        };
        res.writeHead(200, { "Content-Type": contentType[ext] || "text/plain" });
        res.end(fs.readFileSync(filePath));
        return;
      }
    }

    res.writeHead(404, { "Content-Type": "application/json" });
    res.end(JSON.stringify({ error: "Not found", path: url.pathname }));
  }

  private handleRpc(body: string, res: http.ServerResponse) {
    try {
      const req = JSON.parse(body);
      let result: any = { error: `Unknown method: ${req.method}` };

      switch (req.method) {
        case "create_session": {
          const p = req.params;
          const session = this.sessionManager.createSession(p.name || "Untitled", p.project_path || ".");
          this.workspaceManager.setMain(session.project_path);
          result = { session_id: session.id, session };
          break;
        }
        case "list_sessions": {
          result = this.sessionManager.listSessions();
          break;
        }
        case "list_agents": {
          result = this.agentRuntime.listAgents(req.params.session_id);
          break;
        }
        case "create_agent": {
          const p = req.params;
          const session = this.sessionManager.getSession(p.session_id);
          if (!session) {
            result = { error: "Session not found" };
          } else {
            const agent = this.agentRuntime.createAgent({
              session_id: p.session_id,
              name: p.name,
              role: p.role,
              model: p.model,
              workspace: session.project_path,
              team_id: p.team_id,
            });
            this.sessionManager.addAgent(p.session_id, agent.id);
            result = agent;
          }
          break;
        }
        case "pause_agent": {
          const ok = this.agentRuntime.pauseAgent(req.params.agent_id);
          result = { success: ok };
          break;
        }
        case "resume_agent": {
          const ok = this.agentRuntime.resumeAgent(req.params.agent_id);
          result = { success: ok };
          break;
        }
        case "stop_agent": {
          const ok = this.agentRuntime.stopAgent(req.params.agent_id);
          result = { success: ok };
          break;
        }
        case "list_tasks": {
          result = this.taskManager.listTasks(req.params.session_id);
          break;
        }
        case "create_task": {
          const p = req.params;
          const task = this.taskManager.createTask({
            session_id: p.session_id,
            title: p.title,
            description: p.description,
            depends_on: p.depends_on,
            priority: p.priority,
            owner: p.owner,
            team_id: p.team_id,
          });
          this.sessionManager.addTask(p.session_id, task.id);
          result = task;
          break;
        }
        case "list_teams": {
          result = this.teamManager.listTeams(req.params.session_id);
          break;
        }
        case "create_team": {
          const p = req.params;
          const team = this.teamManager.createTeam({
            session_id: p.session_id,
            name: p.name,
            members: p.members,
          });
          result = team;
          break;
        }
        case "send_agent_message": {
          const p = req.params;
          const msg = this.messageBus.sendMessage({
            from: p.from,
            to: p.to,
            team_id: p.team_id,
            task_id: p.task_id,
            message_type: p.message_type || "request",
            subject: p.subject,
            body: p.body,
          });
          result = msg;
          break;
        }
        case "get_diff": {
          const p = req.params;
          const session = this.sessionManager.getSession(p.session_id);
          if (session) {
            const files = this.workspaceManager.listChangedFiles(session.project_path);
            result = { files, session_id: p.session_id };
          } else {
            result = { error: "Session not found" };
          }
          break;
        }
        case "execute_command": {
          const p = req.params;
          const tool = this.toolRegistry.get("shell");
          if (tool) {
            const session = this.sessionManager.getSession(p.session_id);
            const output = awaitWrapper(tool.execute({ command: p.command, cwd: p.cwd }, { agent_id: p.agent_id || "system", workspace_path: session?.project_path || ".", permissions: "full-workspace" }));
            result = output;
          }
          break;
        }
        case "get_context_stats": {
          const stats = this.contextEngine.getStats();
          result = stats;
          break;
        }
        case "create_checkpoint": {
          const p = req.params;
          const session = this.sessionManager.getSession(p.session_id);
          if (session) {
            const cp = this.checkpointManager.createCheckpoint(p.session_id, session.project_path, p.message);
            result = cp;
          }
          break;
        }
        case "list_checkpoints": {
          result = this.checkpointManager.listCheckpoints(req.params.session_id);
          break;
        }
        case "get_model_status": {
          result = { models: DEFAULT_MODELS, providers: Array.from(this.providerStatus.values()) };
          break;
        }
      }

      res.writeHead(200, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ jsonrpc: "2.0", id: req.id, result }));
    } catch (e: any) {
      res.writeHead(500, { "Content-Type": "application/json" });
      res.end(JSON.stringify({ jsonrpc: "2.0", id: null, error: { code: -32603, message: e.message } }));
    }
  }

  private handleWsMessage(ws: WebSocket, msg: any) {
    // Handle RPC over WS
    if (msg.method) {
      // Similar to HTTP RPC but send via WS
      const response = { jsonrpc: "2.0", id: msg.id, result: { ok: true, echo: msg } };
      ws.send(JSON.stringify(response));
    }
  }

  stop() {
    this.wss?.close();
    this.httpServer?.close();
  }
}

function awaitWrapper<T>(promise: Promise<T>): Promise<T> {
  return promise;
}
