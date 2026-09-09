# Contributing to Forge

## Development Setup

1. Clone repository
2. Install Node.js 18+
3. Install Rust toolchain (optional, for Rust Core)
4. Install dependencies: `npm install` in root and packages

## Architecture Principles

- **Core is the product**: Every interface is a client of the Core
- No duplicated orchestration logic
- No UI state as source of truth
- Real agents, real progress, real events
- No fake dashboards or mock buttons

## Code Style

- Rust: `cargo fmt`, `cargo clippy`
- TypeScript: Professional, dense, IDE-like
- No excessive rounded cards, gradients, glassmorphism
- Terminal-first discipline, keyboard-first

## Testing

- Unit tests for core subsystems
- Integration tests for agent loop, multi-agent, task dependencies
- E2E tests for full workflows

## Pull Requests

- Clear description
- Tests for new features
- Documentation updates
- No breaking changes without discussion
