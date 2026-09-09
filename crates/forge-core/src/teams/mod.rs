use forge_protocol::{TeamId, SessionId, AgentId};
use forge_protocol::teams::{TeamInfo, OrchestrationMode};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Team {
    pub info: TeamInfo,
    pub mode: OrchestrationMode,
}

pub struct TeamManager {
    teams: Arc<RwLock<HashMap<TeamId, Team>>>,
}

impl TeamManager {
    pub fn new() -> Self {
        Self { teams: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub async fn create_team(&self, session_id: SessionId, name: String, mode: OrchestrationMode, members: Vec<AgentId>, manager: Option<AgentId>) -> Team {
        let team = Team {
            info: TeamInfo {
                id: Uuid::new_v4(),
                session_id,
                name,
                manager,
                members,
                created_at: Utc::now(),
                description: None,
            },
            mode,
        };
        {
            let mut teams = self.teams.write().await;
            teams.insert(team.info.id, team.clone());
        }
        team
    }

    pub async fn get_team(&self, id: TeamId) -> Option<Team> {
        let teams = self.teams.read().await;
        teams.get(&id).cloned()
    }

    pub async fn list_teams(&self, session_id: SessionId) -> Vec<Team> {
        let teams = self.teams.read().await;
        teams.values().filter(|t| t.info.session_id == session_id).cloned().collect()
    }

    pub async fn add_member(&self, team_id: TeamId, agent_id: AgentId) -> Option<Team> {
        let mut teams = self.teams.write().await;
        if let Some(team) = teams.get_mut(&team_id) {
            if !team.info.members.contains(&agent_id) {
                team.info.members.push(agent_id);
            }
            Some(team.clone())
        } else { None }
    }
}

impl Default for TeamManager {
    fn default() -> Self { Self::new() }
}
