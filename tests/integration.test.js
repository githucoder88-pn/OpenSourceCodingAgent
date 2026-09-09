import { ForgeServer } from "../packages/forge-core-js/dist/index.js";
import assert from "assert";

async function testServerCreation() {
  console.log("Test: Server creation");
  const server = new ForgeServer();
  assert(server.eventBus, "Event bus should exist");
  assert(server.agentRuntime, "Agent runtime should exist");
  assert(server.sessionManager, "Session manager should exist");
  console.log("✓ Server creation");
}

async function testSessionLifecycle() {
  console.log("Test: Session lifecycle");
  const server = new ForgeServer();
  const session = server.sessionManager.createSession("Test", ".");
  assert(session.id, "Session should have ID");
  const retrieved = server.sessionManager.getSession(session.id);
  assert(retrieved, "Should retrieve session");
  const list = server.sessionManager.listSessions();
  assert(list.length === 1, "Should list sessions");
  console.log("✓ Session lifecycle");
}

async function testAgentLifecycle() {
  console.log("Test: Agent lifecycle");
  const server = new ForgeServer();
  const session = server.sessionManager.createSession("Test", ".");
  const agent = server.agentRuntime.createAgent({
    session_id: session.id,
    name: "TestAgent",
    role: "Backend Engineer",
    workspace: ".",
  });
  assert(agent.id, "Agent should have ID");
  assert(agent.state === "Created", "Agent should be Created");
  
  const agents = server.agentRuntime.listAgents(session.id);
  assert(agents.length === 1, "Should list agents");
  
  server.agentRuntime.setState(agent.id, "Executing");
  const updated = server.agentRuntime.getAgent(agent.id);
  assert(updated.state === "Executing", "Agent should be Executing");
  
  server.agentRuntime.pauseAgent(agent.id);
  assert(server.agentRuntime.getAgent(agent.id).state === "Paused", "Agent should be Paused");
  
  server.agentRuntime.resumeAgent(agent.id);
  assert(server.agentRuntime.getAgent(agent.id).state === "Idle", "Agent should be Idle after resume");
  
  console.log("✓ Agent lifecycle");
}

async function testTaskGraph() {
  console.log("Test: Task graph");
  const server = new ForgeServer();
  const session = server.sessionManager.createSession("Test", ".");
  
  const task1 = server.taskManager.createTask({
    session_id: session.id,
    title: "Task 1",
    priority: 1,
  });
  
  const task2 = server.taskManager.createTask({
    session_id: session.id,
    title: "Task 2",
    depends_on: [task1.id],
    priority: 2,
  });
  
  assert(task1.id, "Task1 should have ID");
  assert(task2.dependencies.includes(task1.id), "Task2 should depend on Task1");
  
  let ready = server.taskManager.getReadyTasks(session.id);
  assert(ready.length === 1 && ready[0].id === task1.id, "Only Task1 should be ready");
  
  server.taskManager.updateTaskStatus(task1.id, "Completed", 100);
  ready = server.taskManager.getReadyTasks(session.id);
  assert(ready.length === 1 && ready[0].id === task2.id, "Task2 should be ready after Task1 completed");
  
  console.log("✓ Task graph");
}

async function testTeamSystem() {
  console.log("Test: Team system");
  const server = new ForgeServer();
  const session = server.sessionManager.createSession("Test", ".");
  
  const team = server.teamManager.createTeam({
    session_id: session.id,
    name: "Engineering",
  });
  
  assert(team.id, "Team should have ID");
  
  const agent = server.agentRuntime.createAgent({
    session_id: session.id,
    name: "TestAgent",
    role: "Backend Engineer",
    workspace: ".",
    team_id: team.id,
  });
  
  server.teamManager.addMember(team.id, agent.id);
  const updated = server.teamManager.getTeam(team.id);
  assert(updated.members.includes(agent.id), "Team should contain agent");
  
  console.log("✓ Team system");
}

async function testMessaging() {
  console.log("Test: Agent messaging");
  const server = new ForgeServer();
  const session = server.sessionManager.createSession("Test", ".");
  
  const agent1 = server.agentRuntime.createAgent({
    session_id: session.id,
    name: "Agent1",
    role: "Frontend Engineer",
    workspace: ".",
  });
  
  const agent2 = server.agentRuntime.createAgent({
    session_id: session.id,
    name: "Agent2",
    role: "Backend Engineer",
    workspace: ".",
  });
  
  const msg = server.messageBus.sendMessage({
    from: agent1.id,
    to: agent2.id,
    message_type: "request",
    subject: "API contract",
    body: "Need product endpoint schema",
  });
  
  assert(msg.id, "Message should have ID");
  assert(msg.from === agent1.id, "Message from should be agent1");
  assert(msg.to === agent2.id, "Message to should be agent2");
  
  const messages = server.messageBus.listMessages({ agent_id: agent1.id });
  assert(messages.length === 1, "Should list messages for agent");
  
  console.log("✓ Agent messaging");
}

async function testToolRegistry() {
  console.log("Test: Tool registry");
  const server = new ForgeServer();
  const tools = server.toolRegistry.list();
  assert(tools.length > 0, "Should have tools");
  assert(tools.some(t => t.name === "read_file"), "Should have read_file tool");
  assert(tools.some(t => t.name === "shell"), "Should have shell tool");
  assert(tools.some(t => t.name === "git_status"), "Should have git_status tool");
  console.log("✓ Tool registry");
}

async function testModelRouter() {
  console.log("Test: Model router");
  const server = new ForgeServer();
  const decision = server.modelRouter.route({
    task_complexity: 5,
    required_capabilities: { text_generation: true, streaming: true, tool_calling: true, vision: false, structured_output: true, reasoning: false },
    context_size: 1000,
    strategy: "adaptive",
  }, server.providerStatus, [
    { id: "gpt-4o", provider: "openai", name: "GPT-4o", context_window: 128000, capabilities: { text_generation: true, streaming: true, tool_calling: true, vision: true, structured_output: true, reasoning: false } },
    { id: "llama3.1", provider: "ollama", name: "Llama 3.1", context_window: 32768, capabilities: { text_generation: true, streaming: true, tool_calling: true, vision: false, structured_output: false, reasoning: false } },
  ]);
  
  assert(decision.provider, "Should have provider");
  assert(decision.model, "Should have model");
  console.log("✓ Model router");
}

async function testSubagents() {
  console.log("Test: Subagents");
  const server = new ForgeServer();
  const session = server.sessionManager.createSession("Test", ".");
  
  const parent = server.agentRuntime.createAgent({
    session_id: session.id,
    name: "Parent",
    role: "Architect",
    workspace: ".",
  });
  
  const child = server.agentRuntime.spawnSubagent(parent.id, "Child", "Frontend Engineer");
  assert(child, "Should create subagent");
  assert(child.parent_id === parent.id, "Child should have parent_id");
  
  const parentUpdated = server.agentRuntime.getAgent(parent.id);
  assert(parentUpdated.children.includes(child.id), "Parent should contain child");
  
  console.log("✓ Subagents");
}

async function testCheckpoints() {
  console.log("Test: Checkpoints");
  const server = new ForgeServer();
  const session = server.sessionManager.createSession("Test", ".");
  
  const cp = server.checkpointManager.createCheckpoint(session.id, ".", "Test checkpoint");
  assert(cp.id, "Checkpoint should have ID");
  
  const list = server.checkpointManager.listCheckpoints(session.id);
  assert(list.length === 1, "Should list checkpoints");
  
  console.log("✓ Checkpoints");
}

async function runAll() {
  console.log("Running Forge integration tests...\n");
  await testServerCreation();
  await testSessionLifecycle();
  await testAgentLifecycle();
  await testTaskGraph();
  await testTeamSystem();
  await testMessaging();
  await testToolRegistry();
  await testModelRouter();
  await testSubagents();
  await testCheckpoints();
  console.log("\nAll tests passed! ✓");
}

runAll().catch(err => {
  console.error("Test failed:", err);
  process.exit(1);
});
