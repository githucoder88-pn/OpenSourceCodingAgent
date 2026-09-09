# Forge Architecture

## Overview

Forge is an open-source AI engineering platform for autonomous coding agents and collaborative agent teams.

**Core Philosophy:** The Core is the product. Every interface is a client of the Core.

```
                         ┌───────────────────────┐
                         │        FORGE CORE     │
                         │                       │
                         │ Agent Runtime         │
                         │ Orchestrator          │
                         │ Scheduler             │
                         │ Task Graph            │
                         │ Team Runtime           │
                         │ Model Router           │
                         │ Tool Runtime           │
                         │ Context Engine         │
                         │ Memory                 │
                         │ Permissions            │
                         │ Workspace              │
                         │ Git                    │
                         │ Event Bus              │
                         │ Plugin Runtime         │
                         └───────────┬───────────┘
                                     │
                              IPC / RPC / WebSocket
                                     │
                 ┌───────────────────┼───────────────────┐
                 │                   │                   │
                 ▼                   ▼                   ▼
             Tauri App          Electron App        CLI / TUI
                 │                   │                   │
                 └───────────────────┼───────────────────┘
                                     │
                                     ▼
                                  Web App
```

## Core Runtime

The Core is a long-running local daemon. Clients connect via:

- **IPC** for Tauri/Electron (Unix sockets / named pipes)
- **WebSocket** for real-time events
- **HTTP + JSON-RPC** for request/response
- **STDIO** for CLI

Responsibilities:

- session management
- agent lifecycle (Created → Idle → Planning → Executing → Completed/Failed)
- task scheduling with DAG dependencies
- model calls with routing and failover
- tool execution with permission checks
- event streaming (structured, timestamped, ordered)
- workspace management (main, agent, team, temp)
- Git abstraction (status, diff, commit, branch, worktrees)
- checkpoints (Git + workspace + task + memory state)
- orchestration (solo, supervisor, parallel, pipeline, debate, review-loop, swarm, company)
- teams and messaging
- memory (Global, Project, Session, Team, Agent, Task)

## Agent Engine

```rust
AgentId, SessionId, TaskId, TeamId, MessageId, ToolCallId, WorkspaceId, ModelId, CheckpointId

enum AgentState {
  Created, Idle, Planning, Executing,
  WaitingForTool, WaitingForAgent, Blocked,
  Reviewing, Paused, Failed, Completed, Cancelled
}

struct Agent {
  id: AgentId,
  identity: AgentIdentity,
  role: AgentRole,
  capabilities: Vec<Capability>,
  model: ModelId,
  permissions: PermissionLevel,
  workspace: WorkspaceId,
  parent: Option<AgentId>,
  team: Option<TeamId>,
  current_task: Option<TaskId>,
  state: AgentState,
  progress: f32,
  memory: MemoryScope,
  metrics: AgentMetrics,
  children: Vec<AgentId>,
}
```

Agent runtime supports: cancellation, timeout, retry, pause/resume, subagent spawning, tool calls, streaming, progress, messaging, checkpoints, recovery.

## Coding-Agent Loop

```
User request
  → Load project instructions (AGENTS.md, nested)
  → Load session state
  → Inspect repo context (Indexer → File tree → Symbols → Search → Relevance)
  → Reason about task
  → Create/update plan (Task Graph)
  → Choose tool (Tool Registry)
  → Execute tool (with permissions)
  → Observe result
  → Update state
  → Continue
  → Run validation (tests, build, lint)
  → Review changes
  → Report result
```

## Context Engine

```
Repository → Indexer → File tree → Symbols → Search → Task relevance → Context builder → Model
```

Context is bounded, prioritized, deduplicated, token-aware, cache-friendly, inspectable, resumable.

Tracks: system context, project instructions (AGENTS.md), user request, task context, selected files, tool output, agent messages, memory, previous decisions.

Provides compaction and statistics: `42,380 / 128,000 tokens`.

## Memory

Scopes: Global, Project, Session, Team, Agent, Task

Project Memory includes: Architecture decisions, API contracts, Important files, Known bugs, Design decisions, User preferences, Environment notes.

Memory is searchable, not blindly injected.

## Tool System

All capabilities are tools. Registry with schema, description, permissions, timeout, cancellation, structured result, errors, telemetry.

Initial tools:
- read_file, write_file, edit_file, create_file, delete_file, list_directory
- search_files, search_symbols
- shell, git_status, git_diff, git_log, git_add, git_commit, git_branch
- run_tests, run_build, run_linter, inspect_environment
- http_request, browser, python, node, package_manager

Tools never mutate global state without Core.

## Shell Execution

Robust abstraction: Linux/macOS/Windows, captures stdout/stderr/exit code/duration/signal/cwd/env, streaming, cancellation, no UI freeze.

## Permissions

Autonomy levels: read-only, workspace-write, full-workspace, unrestricted

Approval policies: always, on-risky-commands, on-new-command, never

Dangerous commands detection: rm -rf, credential access, destructive Git, prod deploy, system-wide package changes.

## Model Providers

Trait `ModelProvider` with: text generation, streaming, tool calling, vision, structured output, reasoning config, context window, token accounting.

Adapters: OpenAI-compatible, Anthropic, Google, Azure, OpenRouter, Ollama, LM Studio, Generic OpenAI-compatible, Custom local.

Each model advertises capabilities.

## Model Router

Input: task, agent role, complexity, context size, required capabilities, latency, budget, provider health, local hardware, user preference

Output: provider, model, fallback chain, reasoning level, timeout

Strategies: manual, priority, lowest_cost, lowest_latency, highest_quality, adaptive, local_first, cloud_first

Health: available, degraded, rate_limited, offline

Retries with exponential backoff, fallback chain: Primary → Secondary → Local

## Multi-Agent

Modes: solo, supervisor, parallel, pipeline, debate, review-loop, swarm, company

Example:
```
User → Manager → [Architect, Frontend, Backend, QA, Security, Reviewer]
```

## Subagents

Agents spawn subagents with controlled context inheritance. Operations: spawn, delegate, wait, cancel, inspect, message, handoff, merge.

## Teams

Team: id, name, manager, members, roles, shared context, goals, channel, task queue, workspace policy. Dynamically configurable.

## Agent Communication

Real message protocol:
```json
{
  "id": "msg_123",
  "from": "nova",
  "to": "forge",
  "type": "request",
  "task_id": "task_42",
  "subject": "API contract",
  "body": "Need product endpoint schema."
}
```

Types: request, response, question, answer, status, handoff, warning, blocked, approval, broadcast

## Company Simulation

Optional org mode: CEO/Director → Engineering Manager → [FE, Backend, QA], with roles: Director, PM, Architect, Eng Manager, Frontend, Backend, DB, DevOps, Security, QA, Reviewer, Researcher, Tech Writer. Managers aggregate progress, blockers, completed work, dependencies, quality, test failures, resource allocation.

## Task Graph

DAG execution. Tasks: id, title, description, status, priority, dependencies, owner, team, progress, timestamps, artifacts, errors. Scheduler understands dependencies, parallel when safe.

## Shared Workspace

Main, agent, team, temporary workspaces. Prefer Git worktrees for isolation. Track changed files per agent: who, which task, which agent, which checkpoint.

## Checkpoints + Rollback

Checkpoint: Git state, workspace state, task state, agent state, memory state, event position. Commands: create, list, restore, rollback. Safe rollback.

## Event Bus

Structured events: session.created/resumed, agent.created/started/progress/waiting/blocked/paused/completed/failed, agent.message.sent/received, task.created/started/updated/completed/failed, tool.started/output/completed/failed, file.created/modified/deleted, model.requested/started/stream/completed/failed/fallback, git.changed, test.started/failed/passed, checkpoint.created/restored

Events: structured, timestamped, ordered, streamable, replayable.

## Real-time UI

No aggressive polling. Clients subscribe to event streams via WebSocket. Same stream powers Electron, Tauri, CLI, Web, future IDE.

## Protocol

JSON-RPC 2.0 + WebSocket for events. Methods: create_session, resume_session, send_message, list_agents, create_agent, pause_agent, resume_agent, stop_agent, list_tasks, create_task, assign_task, list_teams, create_team, send_agent_message, stream_events, execute_command, inspect_workspace, read_file, write_file, get_diff, get_model_status, configure_model, create_checkpoint, restore_checkpoint. Versioned.

## Security

Permissions, sandbox policies, path validation, env isolation, command classification, approval prompts, secret redaction, secure IPC, Web auth when remote.

## Performance

Target: 4GB min, 8GB recommended, 16GB+ for heavy multi-agent. Lazy loading, bounded caches, SQLite persistence, streaming, incremental indexing, backpressure, task queues. Low Resource Mode configurable.

## Plugin System

Plugins add: models, providers, tools, commands, agents, roles, integrations, UI panels, hooks. Versioned protocol, no Core source modification required.

## Implementation

Rust workspace for Core (Tokio, Axum, Serde, SQLite, tracing, WebSocket, JSON-RPC). TypeScript React for frontends. Tauri primary lightweight desktop, Electron full compatibility. CLI Rust TUI using same Core APIs. Web same backend protocol.

No duplicated orchestration. No UI state as source of truth.

