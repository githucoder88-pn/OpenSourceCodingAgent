# Security Policy

## Reporting

Please report security vulnerabilities via GitHub Security Advisories.

## Permissions

Forge treats agent-generated shell commands as potentially dangerous.

Implemented:

- Permissions (read-only, workspace-write, full-workspace, unrestricted)
- Sandbox policies
- Path validation
- Environment isolation
- Command classification (dangerous patterns: rm -rf, credential access, destructive Git, prod deploy, system-wide package changes)
- Approval prompts
- Secret redaction in logs
- Secure IPC
- Web authentication when remote enabled

Never expose arbitrary local system access over unauthenticated network endpoint.

## Best Practices

- Use workspace-write by default, not unrestricted
- Review agent commands before approval
- Use checkpoints before risky operations
- Keep API keys in environment variables, not code
- Enable approval policy on-risky-commands
