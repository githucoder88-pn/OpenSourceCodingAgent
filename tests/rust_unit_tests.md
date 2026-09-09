# Rust Unit Tests

These tests would run with `cargo test --workspace` when Rust toolchain is available.

## Coverage

- scheduler: task queue, priority, dependencies, max parallel
- task_graph: DAG, ready tasks, blocked tasks, edges
- router: strategies (lowest_cost, latency, quality, local_first), fallback, health
- permissions: can_execute, requires_approval, denied commands, read-only enforcement
- context: token counting, AGENTS.md loading, compaction, stats
- memory: search, scopes, relevance
- event_bus: publish, subscribe, history, replay
- protocol: RPC serialization, event serialization

All implemented in respective crates with #[cfg(test)] modules.
