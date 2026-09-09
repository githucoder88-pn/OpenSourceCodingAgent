import { Team, SessionId, TeamId, AgentId } from "../types.js";

export class TeamManager {
  private teams = new Map<TeamId, Team>();
  private sessionTeams = new Map<SessionId, Set<TeamId>>();

  createTeam(params: {
    session_id: SessionId;
    name: string;
    mode?: string;
    members?: AgentId[];
    manager?: AgentId;
    description?: string;
  }): Team {
    const team: Team = {
      id: crypto.randomUUID(),
      session_id: params.session_id,
      name: params.name,
      manager: params.manager,
      members: params.members || [],
      created_at: new Date().toISOString(),
      description: params.description,
      mode: params.mode || "supervisor",
    };

    this.teams.set(team.id, team);
    if (!this.sessionTeams.has(team.session_id)) {
      this.sessionTeams.set(team.session_id, new Set());
    }
    this.sessionTeams.get(team.session_id)!.add(team.id);

    return team;
  }

  getTeam(id: TeamId): Team | undefined {
    return this.teams.get(id);
  }

  listTeams(sessionId: SessionId): Team[] {
    const ids = this.sessionTeams.get(sessionId);
    if (!ids) return [];
    return Array.from(ids).map(id => this.teams.get(id)!).filter(Boolean);
  }

  addMember(teamId: TeamId, agentId: AgentId): Team | undefined {
    const team = this.teams.get(teamId);
    if (team && !team.members.includes(agentId)) {
      team.members.push(agentId);
    }
    return team;
  }

  removeMember(teamId: TeamId, agentId: AgentId): Team | undefined {
    const team = this.teams.get(teamId);
    if (team) {
      team.members = team.members.filter(id => id !== agentId);
    }
    return team;
  }
}
