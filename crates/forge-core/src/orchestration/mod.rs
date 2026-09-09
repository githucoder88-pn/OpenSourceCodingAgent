use forge_protocol::teams::OrchestrationMode;
use forge_protocol::{AgentId, TaskId};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum OrchestrationStrategy {
    Solo,
    Supervisor { manager: AgentId, workers: Vec<AgentId> },
    Parallel { agents: Vec<AgentId> },
    Pipeline { stages: Vec<Vec<AgentId>> },
    Debate { participants: Vec<AgentId>, rounds: usize },
    ReviewLoop { author: AgentId, reviewer: AgentId, max_iterations: usize },
    Swarm { agents: Vec<AgentId>, coordination: SwarmCoordination },
    Company { hierarchy: CompanyHierarchy },
}

#[derive(Debug, Clone)]
pub enum SwarmCoordination {
    Centralized { coordinator: AgentId },
    Decentralized,
}

#[derive(Debug, Clone)]
pub struct CompanyHierarchy {
    pub director: Option<AgentId>,
    pub managers: Vec<AgentId>,
    pub teams: HashMap<String, Vec<AgentId>>,
}

impl CompanyHierarchy {
    pub fn new() -> Self {
        Self { director: None, managers: Vec::new(), teams: HashMap::new() }
    }
}

impl Default for CompanyHierarchy {
    fn default() -> Self { Self::new() }
}

pub struct Orchestrator {
    mode: OrchestrationMode,
    strategy: Option<OrchestrationStrategy>,
}

impl Orchestrator {
    pub fn new(mode: OrchestrationMode) -> Self {
        Self { mode, strategy: None }
    }

    pub fn set_strategy(&mut self, strategy: OrchestrationStrategy) {
        self.strategy = Some(strategy);
    }

    pub fn next_task(&self, available_agents: &[AgentId], pending_tasks: &[TaskId]) -> Option<(AgentId, TaskId)> {
        if available_agents.is_empty() || pending_tasks.is_empty() {
            return None;
        }
        // Simple round-robin for now, more sophisticated scheduling in real impl
        Some((available_agents[0], pending_tasks[0]))
    }

    pub fn should_parallelize(&self, task_count: usize, agent_count: usize) -> bool {
        matches!(self.mode, OrchestrationMode::Parallel | OrchestrationMode::Swarm | OrchestrationMode::Company) && task_count > 1 && agent_count > 1
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new(OrchestrationMode::Supervisor)
    }
}
