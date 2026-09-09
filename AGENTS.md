# AGENTS.md

This file provides instructions for AI coding agents working in the Forge repository itself.

## Project Overview

Forge is an open-source AI engineering platform for autonomous coding agents and collaborative agent teams.

## Architecture

- Core: Rust (primary) + TypeScript (JS implementation) - long-running daemon
- Protocol: JSON-RPC 2.0 + WebSocket
- Clients: Tauri (lightweight), Electron (full compatibility), CLI, Web (React)
- Core is the product, every interface is a client

## Key Directories

- `crates/forge-core/`: Agent runtime, orchestration, scheduler, task graph, teams, messaging, context, memory, workspace, permissions, checkpoints, sessions, events
- `crates/forge-protocol/`: RPC, events, agents, teams, tasks, tools, models
- `crates/forge-models/`: Model providers and router
- `crates/forge-tools/`: Tool registry
- `crates/forge-server/`: API, WebSocket, runtime
- `crates/forge-cli/`: CLI binary
- `crates/forge-search/`: Indexer
- `packages/forge-core-js/`: Runnable JS Core implementation
- `apps/forge-web/`: Web client
- `apps/forge-tauri/`: Tauri desktop
- `apps/forge-electron/`: Electron desktop
- `packages/ui/`: Design system

## Development Workflow

1. Understand the Core is the source of truth
2. No duplicated orchestration logic in clients
3. Real agents, real progress, real events - no fake UI
4. Test with `forge demo` and `forge run`
5. Use structured events for all important actions

## Coding Standards

- Rust: cargo fmt, clippy, strong typing
- TypeScript: Strict, professional, IDE-like UI (no excessive gradients/glassmorphism)
- Every visible item should help developer understand/control/execute engineering work
- Keyboard-first, terminal-first discipline

## Important Files

- `ARCHITECTURE.md`: Full architecture
- `packages/forge-core-js/src/server/forge-server.ts`: JS Core server
- `crates/forge-core/src/lib.rs`: Rust Core entry
- `apps/forge-web/src/App.tsx`: Web UI

## Running

- JS Core: `node packages/forge-core-js/dist/server.js` or `forge server`
- Web: `cd apps/forge-web && npm run dev`
- CLI: `forge run "task"`
- Rust: `cargo run -p forge-cli`

## Testing

- Unit: scheduler, task graph, router, permissions, context, memory, event bus, protocol
- Integration: agent loop, multi-agent, task dependencies, failover, checkpoints
- E2E: User → Core → Agent → Tool → File → Test → Result
