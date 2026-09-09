export type AgentId = string;
export type SessionId = string;
export type TaskId = string;
export type TeamId = string;
export type MessageId = string;
export type ToolCallId = string;
export type WorkspaceId = string;
export type ModelId = string;
export type CheckpointId = string;

export enum AgentState {
  Created = "Created",
  Idle = "Idle",
  Planning = "Planning",
  Executing = "Executing",
  WaitingForTool = "WaitingForTool",
  WaitingForAgent = "WaitingForAgent",
  Blocked = "Blocked",
  Reviewing = "Reviewing",
  Paused = "Paused",
  Failed = "Failed",
  Completed = "Completed",
  Cancelled = "Cancelled",
}

export enum AgentRole {
  Architect = "Architect",
  FrontendEngineer = "Frontend Engineer",
  BackendEngineer = "Backend Engineer",
  DatabaseEngineer = "Database Engineer",
  DevOpsEngineer = "DevOps Engineer",
  SecurityEngineer = "Security Engineer",
  QAEngineer = "QA Engineer",
  Reviewer = "Reviewer",
  Researcher = "Researcher",
  TechnicalWriter = "Technical Writer",
  ProductManager = "Product Manager",
  EngineeringManager = "Engineering Manager",
  Director = "Director",
}

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

export interface AgentMetrics {
  tokens_used: number;
  requests: number;
  tool_calls: number;
  files_changed: number;
  latency_ms: number;
  started_at?: string;
  completed_at?: string;
}

export interface Agent {
  id: AgentId;
  session_id: SessionId;
  name: string;
  role: AgentRole;
  state: AgentState;
  progress: number;
  current_task?: TaskId;
  team_id?: TeamId;
  parent_id?: AgentId;
  children: AgentId[];
  model: string;
  permissions: PermissionLevel;
  workspace: string;
  metrics: AgentMetrics;
  created_at: string;
  updated_at: string;
  current_action?: string;
  files_changed: string[];
  tools_used: string[];
  is_paused: boolean;
}

export enum TaskStatus {
  Pending = "Pending",
  Ready = "Ready",
  Running = "Running",
  Blocked = "Blocked",
  Reviewing = "Reviewing",
  Completed = "Completed",
  Failed = "Failed",
  Cancelled = "Cancelled",
}

export interface Task {
  id: TaskId;
  session_id: SessionId;
  title: string;
  description?: string;
  status: TaskStatus;
  priority: number;
  dependencies: TaskId[];
  owner?: AgentId;
  team_id?: TeamId;
  progress: number;
  created_at: string;
  started_at?: string;
  completed_at?: string;
  artifacts: string[];
  errors: string[];
}

export interface Team {
  id: TeamId;
  session_id: SessionId;
  name: string;
  manager?: AgentId;
  members: AgentId[];
  created_at: string;
  description?: string;
  mode: string;
}

export interface Session {
  id: SessionId;
  name: string;
  project_path: string;
  created_at: string;
  updated_at: string;
  agents: AgentId[];
  tasks: TaskId[];
  current_branch?: string;
  status: "Active" | "Paused" | "Completed" | "Archived";
}

export interface ForgeEvent {
  id: string;
  timestamp: string;
  session_id?: SessionId;
  payload: EventPayload;
}

export type EventPayload =
  | { type: "session.created"; data: { session_id: SessionId; project_path: string } }
  | { type: "session.resumed"; data: { session_id: SessionId } }
  | { type: "agent.created"; data: { agent_id: AgentId; name: string; role: string } }
  | { type: "agent.started"; data: { agent_id: AgentId } }
  | { type: "agent.progress"; data: { agent_id: AgentId; progress: number; message: string } }
  | { type: "agent.waiting"; data: { agent_id: AgentId; reason: string } }
  | { type: "agent.blocked"; data: { agent_id: AgentId; reason: string } }
  | { type: "agent.paused"; data: { agent_id: AgentId } }
  | { type: "agent.resumed"; data: { agent_id: AgentId } }
  | { type: "agent.completed"; data: { agent_id: AgentId } }
  | { type: "agent.failed"; data: { agent_id: AgentId; error: string } }
  | { type: "agent.message.sent"; data: { message_id: string; from: AgentId; to?: AgentId; team_id?: TeamId; body: string } }
  | { type: "agent.message.received"; data: { message_id: string; to: AgentId; from: AgentId; body: string } }
  | { type: "task.created"; data: { task_id: TaskId; title: string } }
  | { type: "task.started"; data: { task_id: TaskId; agent_id?: AgentId } }
  | { type: "task.updated"; data: { task_id: TaskId; progress: number; status: string } }
  | { type: "task.completed"; data: { task_id: TaskId } }
  | { type: "task.failed"; data: { task_id: TaskId; error: string } }
  | { type: "tool.started"; data: { tool_call_id: ToolCallId; agent_id: AgentId; tool: string; input: any } }
  | { type: "tool.output"; data: { tool_call_id: ToolCallId; chunk: string; is_stderr: boolean } }
  | { type: "tool.completed"; data: { tool_call_id: ToolCallId; result: any } }
  | { type: "tool.failed"; data: { tool_call_id: ToolCallId; error: string } }
  | { type: "file.created"; data: { path: string; agent_id?: AgentId } }
  | { type: "file.modified"; data: { path: string; agent_id?: AgentId; diff?: string } }
  | { type: "file.deleted"; data: { path: string; agent_id?: AgentId } }
  | { type: "model.requested"; data: { agent_id: AgentId; model: string; provider: string } }
  | { type: "model.started"; data: { agent_id: AgentId; model: string } }
  | { type: "model.stream"; data: { agent_id: AgentId; delta: string } }
  | { type: "model.completed"; data: { agent_id: AgentId; tokens: number } }
  | { type: "model.failed"; data: { agent_id: AgentId; error: string } }
  | { type: "model.fallback"; data: { agent_id: AgentId; from: string; to: string; reason: string } }
  | { type: "git.changed"; data: { branch?: string; status: string } }
  | { type: "test.started"; data: { suite: string } }
  | { type: "test.failed"; data: { suite: string; error: string } }
  | { type: "test.passed"; data: { suite: string } }
  | { type: "checkpoint.created"; data: { checkpoint_id: CheckpointId; message?: string } }
  | { type: "checkpoint.restored"; data: { checkpoint_id: CheckpointId } }
  | { type: "log"; data: { level: string; message: string; agent_id?: AgentId } };

export interface ToolDefinition {
  name: string;
  description: string;
  parameters: any;
  permissions: string[];
  timeout_ms?: number;
}

export interface ModelProviderStatus {
  provider: string;
  health: "available" | "degraded" | "rate_limited" | "offline";
  latency_ms?: number;
  error_rate: number;
  last_check: string;
}

export interface RoutingDecision {
  provider: string;
  model: string;
  fallback_chain: [string, string][];
  reasoning: string;
}
