# Permissions

Autonomy levels:

- read-only: Can only read, search, git status/diff/log
- workspace-write: Can write files in workspace, run tests/build (default)
- full-workspace: Can execute shell, git operations, package managers
- unrestricted: Full system access

Approval policies:

- always: Require approval for all tool calls
- on-risky-commands: Require approval for risky commands (rm -rf, sudo, git push, etc)
- on-new-command: Require approval for new command patterns
- never: Never require approval (use with caution)

Dangerous commands detection:

- rm -rf /
- format disks
- credential access
- destructive Git (reset --hard, push --force)
- production deployment
- system-wide package changes

Never silently bypass permissions. Expose clearly in UI.
