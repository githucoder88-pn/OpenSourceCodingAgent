# Agents

Every agent is a real runtime object.

State: Created, Idle, Planning, Executing, WaitingForTool, WaitingForAgent, Blocked, Reviewing, Paused, Failed, Completed, Cancelled

Agent contains: identity, role, capabilities, model, permissions, workspace, parent_agent, team, current_task, state, progress, memory, metrics, children, message channels

Supports: cancellation, timeout, retry, pause, resume, subagent creation, tool calls, streaming, progress, messages, checkpoints, recovery.
