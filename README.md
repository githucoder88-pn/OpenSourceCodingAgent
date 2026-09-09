# Forge

**Open-source AI engineering platform for autonomous coding agents and collaborative agent teams.**

> The Core is the product. Every interface is a client of the Core.

Forge is an original open-source implementation inspired by the observable capabilities and workflows of modern coding agents such as OpenAI Codex CLI and Anthropic Claude Code, extended substantially with multi-agent orchestration, subagents, agent teams, organizational simulation, real-time progress, intelligent model routing, and more.

## Architecture

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

## Features

- **Agent Runtime**: Real agent lifecycle (Created → Planning → Executing → Completed/Failed) with cancellation, pause/resume, subagents
- **Multi-Agent**: solo, supervisor, parallel, pipeline, debate, review-loop, swarm, company modes
- **Teams**: First-class Team abstraction with manager, members, shared context, communication channels
- **Agent Communication**: Real message protocol (request, response, question, answer, status, handoff, broadcast)
- **Task Graph**: DAG-based execution with dependencies, parallel when safe
- **Model Routing**: Intelligent routing (manual, priority, lowest_cost, latency, quality, adaptive, local_first) with fallback chains and health monitoring
- **Model Providers**: OpenAI, Anthropic, Google, Azure, OpenRouter, Ollama, LM Studio, compatible endpoints
- **Tool System**: All capabilities as tools (filesystem, shell, git, search, http, package managers) with permissions and telemetry
- **Context Engine**: Bounded, prioritized, deduplicated, token-aware, with AGENTS.md support and compaction
- **Memory**: Global, Project, Session, Team, Agent, Task scopes, searchable
- **Workspace**: Main, agent, team, temporary with Git worktree support and file attribution
- **Checkpoints**: Git + workspace + task + agent + memory + event position, with rollback
- **Event Bus**: Structured, timestamped, ordered, streamable events powering all clients
- **Permissions**: read-only, workspace-write, full-workspace, unrestricted with approval policies
- **Observability**: Tokens, latency, provider health, tool calls, file changes

## Quick Start

### CLI (Recommended)

```bash
# Install (requires Node.js 18+)
npm install -g forge-core-js
# Or from source:
cd packages/forge-core-js && npm install && npm run build

# Initialize project
forge init

# Run a task
forge run "Fix the authentication bug"

# Plan mode
forge plan "Refactor authentication"

# Server mode
forge server --port 3000

# Demo
forge demo
```

### Core Server

```bash
# Start Forge Core daemon
node packages/forge-core-js/dist/server.js
# Or
forge server

# Core listens on http://localhost:3000
# WebSocket at ws://localhost:3000/ws
# API at http://localhost:3000/api
```

### Web Client

```bash
cd apps/forge-web
npm install
npm run dev
# Opens at http://localhost:5173, proxies to Core at :3000
```

### Tauri (Lightweight Desktop)

```bash
cd apps/forge-tauri
npm install
npm run tauri
```

### Electron (Full Compatibility)

```bash
cd apps/forge-electron
npm install
npm run dev
```

### Rust Core (Primary Implementation)

```bash
# Requires Rust toolchain
cargo build --workspace
cargo run -p forge-cli -- server
cargo run -p forge-cli -- run "Fix the bug"
```

## Project Structure

```
forge/
├── crates/
│   ├── forge-core/       # Agent, orchestration, scheduler, task graph, teams, messaging, context, memory, workspace, permissions, checkpoints, sessions, events
│   ├── forge-models/     # Model providers (openai, anthropic, google, openrouter, azure, ollama, lmstudio, compatible) + router
│   ├── forge-tools/      # Tool registry (filesystem, shell, git, search, http, package_manager)
│   ├── forge-protocol/   # RPC, events, agents, teams, tasks, tools, models
│   ├── forge-server/     # API, WebSocket, IPC, runtime
│   ├── forge-cli/        # CLI binary
│   └── forge-search/     # Indexer, file tree, symbols, search
├── apps/
│   ├── forge-tauri/      # Tauri lightweight desktop
│   ├── forge-electron/   # Electron full compatibility
│   └── forge-web/        # Web client (React + WebSocket)
├── packages/
│   ├── ui/               # Design system, components
│   ├── protocol-client/  # JS client for Forge protocol
│   ├── icons/            # Icon set
│   ├── design-system/    # Theme, tokens
│   └── forge-core-js/    # JS implementation of Core (runnable without Rust)
├── plugins/              # Plugin system
├── docs/                 # Documentation
├── examples/             # Example projects
└── tests/                # Tests
```

## Core Concepts

### Agent Loop

```
User request → Load AGENTS.md → Load session → Inspect context → Reason → Plan → Choose tool → Execute → Observe → Update → Validate → Review → Report
```

### Task Graph Example

```json
{
  "goal": "Implement authentication",
  "tasks": [
    { "id": "T1", "title": "Inspect auth architecture", "depends_on": [] },
    { "id": "T2", "title": "Implement backend auth", "depends_on": ["T1"] },
    { "id": "T3", "title": "Implement frontend auth", "depends_on": ["T1"] },
    { "id": "T4", "title": "Integration tests", "depends_on": ["T2", "T3"] }
  ]
}
```

### Permissions

- `read-only`: Can only read files, search, git status/diff/log
- `workspace-write`: Can write files in workspace, run tests/build
- `full-workspace`: Can execute shell, git operations, package managers
- `unrestricted`: Full system access (use with caution)

Approval policies: `always`, `on-risky-commands`, `on-new-command`, `never`

### Model Routing

Strategies: `manual`, `priority`, `lowest_cost`, `lowest_latency`, `highest_quality`, `adaptive`, `local_first`, `cloud_first`

Fallback chain: Primary → Secondary → Local (Ollama)

### Company Simulation

```
CEO / Director
      │
      ▼
Engineering Manager
      │
 ┌────┼─────────┐
 ▼    ▼         ▼
FE   Backend    QA
```

Roles: Director, Product Manager, Architect, Engineering Manager, Frontend, Backend, Database, DevOps, Security, QA, Reviewer, Researcher, Technical Writer

## Configuration

` .forge/config.yaml`:

```yaml
project:
  name: my-project

orchestration:
  mode: supervisor
  max_agents: 6
  max_parallel_tasks: 4

models:
  primary:
    provider: openai
    model: gpt-4o

  fast:
    provider: ollama
    model: llama3.1

  reviewer:
    provider: anthropic
    model: claude-3-5-sonnet-20241022

routing:
  strategy: adaptive
  fallback: true
  cost_aware: true
  latency_aware: true

autonomy:
  default: workspace-write
  approval_policy: on-risky-commands

performance:
  low_resource_mode: false
  max_parallel_agents: 6
  max_context_cache_mb: 512
```

## Commands

```
/help /init /project /session /resume /new /agent /team /task /model /models /provider /tool /tools /plan /run /test /build /review /debug /search /read /edit /git /diff /commit /branch /checkpoint /rollback /memory /context /logs /events /status /watch /spawn /message /broadcast /pause /resume /stop /retry /config /plugins
```

## CLI

```bash
forge
forge run "Fix the login bug"
forge plan "Refactor authentication"
forge agents
forge teams
forge tasks
forge status
forge logs
forge watch
forge model list
forge checkpoint create
forge rollback
forge session list
forge session resume <id>
forge demo
```

## Development

```bash
# JS Core
cd packages/forge-core-js
npm install
npm run build
npm run dev

# Web
cd apps/forge-web
npm install
npm run dev

# Rust (if toolchain available)
cargo test --workspace
cargo build --workspace
```

## Testing

- Unit tests: scheduler, task graph, router, permissions, context, memory, event bus, protocol
- Integration tests: agent loop, tool execution, multi-agent communication, subagent spawning, task dependencies, model failover, session resume, checkpoints, rollback, Git workflows
- E2E tests: User → Core → Agent → Tool → File → Test → Result

## License

MIT OR Apache-2.0

## Security

See SECURITY.md

## Contributing

See CONTRIBUTING.md
