#!/usr/bin/env node
import { ForgeServer } from "./server/forge-server.js";
import { AgentRole, AgentState, TaskStatus } from "./core/types.js";
import * as fs from "fs";
import * as path from "path";

const args = process.argv.slice(2);
const command = args[0];

function printHelp() {
  console.log(`
Forge - Open-source AI engineering platform for autonomous coding agents

Usage:
  forge [command] [options]

Commands:
  server              Start Forge server
  run <prompt>        Run a task
  plan <prompt>       Plan a task
  agents              List agents
  teams               List teams
  tasks               List tasks
  status              Show status
  logs                Show logs
  watch               Watch events
  model list          List models
  model status        Show model status
  checkpoint create   Create checkpoint
  checkpoint list     List checkpoints
  session list        List sessions
  demo                Run demo mode
  init                Initialize project

Options:
  --help, -h          Show help
  --version, -v       Show version
  --project <path>    Project path (default: .)
  --verbose           Verbose output
`);
}

async function main() {
  if (!command || command === "--help" || command === "-h") {
    printHelp();
    return;
  }

  if (command === "--version" || command === "-v") {
    console.log("forge 0.1.0");
    return;
  }

  if (command === "server") {
    const portArg = args.find(a => a.startsWith("--port="))?.split("=")[1] || args[args.indexOf("--port") + 1] || "3000";
    const port = parseInt(portArg, 10);
    console.log(`Starting Forge server on 0.0.0.0:${port}`);
    const server = new ForgeServer();
    await server.start(port, "0.0.0.0");
    return;
  }

  if (command === "run") {
    const prompt = args.slice(1).join(" ");
    if (!prompt) {
      console.error("Usage: forge run <prompt>");
      process.exit(1);
    }
    console.log(`\nForge running: "${prompt}"\n`);
    
    const server = new ForgeServer();
    const session = server.sessionManager.createSession("CLI Session", process.cwd());
    server.workspaceManager.setMain(process.cwd());
    console.log(`Session: ${session.id} | Project: ${session.project_path}`);

    // Create a team
    const team = server.teamManager.createTeam({
      session_id: session.id,
      name: "Engineering",
      mode: "supervisor",
    });

    // Create agents
    const roles = [
      { name: "Atlas", role: AgentRole.Architect },
      { name: "Nova", role: AgentRole.FrontendEngineer },
      { name: "Forge", role: AgentRole.BackendEngineer },
      { name: "Echo", role: AgentRole.QAEngineer },
      { name: "Hawk", role: AgentRole.SecurityEngineer },
    ];

    const agents = roles.map(r => {
      const agent = server.agentRuntime.createAgent({
        session_id: session.id,
        name: r.name,
        role: r.role as any,
        workspace: process.cwd(),
        team_id: team.id,
      });
      server.sessionManager.addAgent(session.id, agent.id);
      server.teamManager.addMember(team.id, agent.id);
      return agent;
    });

    console.log(`\nTeam: ${team.name} | ${agents.length} agents`);
    agents.forEach(a => console.log(`  - ${a.name} (${a.role}) [${a.state}]`));

    // Create task graph
    const taskTitles = [
      "Inspect project structure and requirements",
      "Design architecture",
      "Implement core functionality",
      "Write tests",
      "Security review",
      "Final review and documentation"
    ];

    const tasks: any[] = [];
    for (let idx = 0; idx < taskTitles.length; idx++) {
      const title = taskTitles[idx];
      const task = server.taskManager.createTask({
        session_id: session.id,
        title,
        description: `Task for: ${prompt}`,
        depends_on: idx > 0 ? [tasks[idx-1]?.id].filter(Boolean) as string[] : [],
        priority: taskTitles.length - idx,
      });
      server.sessionManager.addTask(session.id, task.id);
      tasks.push(task);
    }

    console.log(`\nTask Graph: ${tasks.length} tasks`);
    tasks.forEach(t => console.log(`  - ${t.title} [${t.status}]`));

    // Simulate execution
    console.log("\n--- Execution ---\n");
    for (let i = 0; i < tasks.length; i++) {
      const task = tasks[i];
      const agent = agents[i % agents.length];
      
      console.log(`[${new Date().toLocaleTimeString()}] ${agent.name} started: ${task.title}`);
      server.taskManager.updateTaskStatus(task.id, TaskStatus.Running);
      server.agentRuntime.assignTask(agent.id, task.id);
      server.agentRuntime.setState(agent.id, AgentState.Executing);

      // Simulate work
      for (let p = 0; p <= 100; p += 25) {
        await new Promise(r => setTimeout(r, 200));
        server.agentRuntime.setProgress(agent.id, p, `Working on ${task.title}: ${p}%`);
        server.taskManager.updateTaskStatus(task.id, TaskStatus.Running, p);
        if (p % 50 === 0) {
          console.log(`  ${agent.name} progress: ${p}% - ${task.title}`);
        }
      }

      server.taskManager.updateTaskStatus(task.id, TaskStatus.Completed, 100);
      server.agentRuntime.setState(agent.id, AgentState.Completed);
      server.agentRuntime.setProgress(agent.id, 100, `Completed ${task.title}`);
      console.log(`[${new Date().toLocaleTimeString()}] ${agent.name} completed: ${task.title}\n`);
    }

    console.log("All tasks completed!");
    console.log(`\nSession ${session.id} finished.`);
    return;
  }

  if (command === "plan") {
    const prompt = args.slice(1).join(" ");
    console.log(`Planning: "${prompt}"\n`);
    console.log("Generated plan:");
    console.log(JSON.stringify({
      goal: prompt,
      tasks: [
        { id: "T1", title: "Inspect architecture", depends_on: [] },
        { id: "T2", title: "Implement backend", depends_on: ["T1"] },
        { id: "T3", title: "Implement frontend", depends_on: ["T1"] },
        { id: "T4", title: "Integration tests", depends_on: ["T2", "T3"] },
      ]
    }, null, 2));
    return;
  }

  if (command === "demo") {
    console.log(`
Forge Demo Mode
===============

Creating sample project with agents and tasks...

Project: my-app
Branch: main

Build authentication system

6 agents
12 tasks
3 running
2 blocked

Overall progress 68%

Agents:
  Atlas       Architect       84%  Working - Finalizing architecture
  Nova        Frontend        72%  Working - Implementing product page
  Forge       Backend         91%  Reviewing - Database schema ready
  Echo        QA              41%  Blocked - Waiting for checkout fix
  Hawk        Security        32%  Planning - Reviewing auth flow
  Orion       DevOps          15%  Idle

Tasks:
  [Done] Inspect auth architecture
  [Done] Design database schema
  [Running] Implement backend auth (Forge - 91%)
  [Running] Implement frontend auth (Nova - 72%)
  [Running] Build product page (Atlas - 84%)
  [Blocked] Cart test failed (Echo)
  [Pending] Security audit
  [Pending] Integration tests

Recent events:
  14:32 Atlas - Architecture finalized
  14:34 Forge - Database schema ready
  14:35 Nova - I need the product API contract
  14:35 Forge - Shared /api/products contract
  14:37 Echo - Cart test failed
  14:38 Nova - Fixing checkout state handling

This is demo data. Run 'forge run "your task"' for real execution.
`);
    return;
  }

  if (command === "init") {
    const projectPath = args[1] || ".";
    console.log(`Initializing Forge project in ${projectPath}`);
    
    const agentsPath = path.join(projectPath, "AGENTS.md");
    if (!fs.existsSync(agentsPath)) {
      const content = `# AGENTS.md

This file provides instructions for AI coding agents working in this repository.

## Project Overview

Describe your project here.

## Architecture

- Frontend: 
- Backend:
- Database:

## Development Workflow

1. Read relevant files before making changes
2. Run tests after changes
3. Keep commits focused and descriptive

## Coding Standards

- Follow existing code style
- Write tests for new features
- Document complex logic

## Important Files

- List important files and their purposes

## Environment

- Node version:
- Python version:
- Other dependencies:

## Known Issues

- List known bugs or limitations
`;
      fs.writeFileSync(agentsPath, content);
      console.log(`Created ${agentsPath}`);
    } else {
      console.log(`${agentsPath} already exists`);
    }

    const forgeDir = path.join(projectPath, ".forge");
    if (!fs.existsSync(forgeDir)) {
      fs.mkdirSync(forgeDir, { recursive: true });
      console.log(`Created ${forgeDir}`);
    }

    const configPath = path.join(forgeDir, "config.yaml");
    if (!fs.existsSync(configPath)) {
      const config = `project:
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
  background_indexing: true
  local_embeddings: false
  verbose_events: true
`;
      fs.writeFileSync(configPath, config);
      console.log(`Created ${configPath}`);
    }

    console.log("\nForge project initialized!");
    console.log("Next steps:");
    console.log("  1. Edit AGENTS.md with your project details");
    console.log("  2. Configure models in .forge/config.yaml");
    console.log('  3. Run: forge run "your first task"');
    return;
  }

  if (command === "status") {
    console.log("Forge Status:");
    console.log("  Core: JS implementation (Node.js)");
    console.log(`  Project: ${process.cwd()}`);
    console.log("  Server: not running (use 'forge server' to start)");
    console.log("  Models: ollama available, cloud providers need API keys");
    return;
  }

  if (command === "agents" || command === "teams" || command === "tasks" || command === "model" || command === "checkpoint" || command === "session") {
    console.log(`Command '${command}' requires a running server. Start with 'forge server' and use the web UI or API.`);
    console.log("For demo, try: forge demo");
    return;
  }

  console.error(`Unknown command: ${command}`);
  printHelp();
  process.exit(1);
}

main().catch(err => {
  console.error("Error:", err);
  process.exit(1);
});
