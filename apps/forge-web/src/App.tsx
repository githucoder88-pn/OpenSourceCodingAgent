import React, { useState } from 'react';
import { Layout } from './components/Layout';
import { Sidebar } from './components/Sidebar';
import { ActivityPanel } from './components/ActivityPanel';
import { Terminal } from './components/Terminal';
import { useForgeEvents, useSessions, useAgents, useTasks } from './hooks/useForge';

function ProjectView({ sessionId }: { sessionId?: string }) {
  const agents = useAgents(sessionId);
  const tasks = useTasks(sessionId);
  
  if (!sessionId) {
    return (
      <div style={{ padding: '24px' }}>
        <h1 style={{ fontSize: '20px', fontWeight: 700, marginBottom: '8px' }}>Welcome to Forge</h1>
        <p style={{ color: '#888', fontSize: '13px', marginBottom: '24px' }}>Open-source AI engineering platform for autonomous coding agents and collaborative agent teams.</p>
        
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px', marginBottom: '24px' }}>
          <div style={{ border: '1px solid #1f1f1f', padding: '16px', background: '#111' }}>
            <div style={{ fontSize: '12px', color: '#666', marginBottom: '8px', textTransform: 'uppercase', letterSpacing: '0.08em' }}>Quick Start</div>
            <div style={{ fontSize: '13px', lineHeight: '1.6' }}>
              <div>1. Create a session with your project path</div>
              <div>2. Add agents with different roles</div>
              <div>3. Create tasks and assign to agents</div>
              <div>4. Watch real-time progress</div>
            </div>
          </div>
          <div style={{ border: '1px solid #1f1f1f', padding: '16px', background: '#111' }}>
            <div style={{ fontSize: '12px', color: '#666', marginBottom: '8px', textTransform: 'uppercase', letterSpacing: '0.08em' }}>Features</div>
            <div style={{ fontSize: '13px', lineHeight: '1.6' }}>
              <div>• Multi-agent orchestration</div>
              <div>• Task graph with dependencies</div>
              <div>• Real-time event streaming</div>
              <div>• Model routing & failover</div>
            </div>
          </div>
        </div>

        <div style={{ border: '1px solid #1f1f1f', background: '#111', padding: '16px' }}>
          <div style={{ fontSize: '13px', fontWeight: 600, marginBottom: '12px' }}>Demo Project</div>
          <div style={{ fontSize: '12px', color: '#888', marginBottom: '8px' }}>Build authentication system</div>
          <div style={{ display: 'flex', gap: '16px', fontSize: '11px', color: '#666' }}>
            <span>6 agents</span>
            <span>12 tasks</span>
            <span>3 running</span>
            <span>2 blocked</span>
            <span>Overall 68%</span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div>
      <div style={{ marginBottom: '24px' }}>
        <h2 style={{ fontSize: '16px', fontWeight: 600, marginBottom: '4px' }}>Project Overview</h2>
        <div style={{ fontSize: '12px', color: '#888' }}>Session: {sessionId}</div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px', marginBottom: '24px' }}>
        <div style={{ border: '1px solid #1f1f1f', background: '#111', padding: '16px' }}>
          <div style={{ fontSize: '11px', color: '#666', textTransform: 'uppercase', letterSpacing: '0.08em', marginBottom: '12px' }}>Agents ({agents.length})</div>
          {agents.map(a => (
            <div key={a.id} style={{ display: 'flex', justifyContent: 'space-between', padding: '6px 0', borderBottom: '1px solid #1a1a1a', fontSize: '12px' }}>
              <span>{a.name} - {a.role}</span>
              <span style={{ color: a.state === 'Executing' ? '#22c55e' : '#666' }}>{a.state} {a.progress}%</span>
            </div>
          ))}
          {agents.length === 0 && <div style={{ color: '#666', fontSize: '12px' }}>No agents yet</div>}
        </div>

        <div style={{ border: '1px solid #1f1f1f', background: '#111', padding: '16px' }}>
          <div style={{ fontSize: '11px', color: '#666', textTransform: 'uppercase', letterSpacing: '0.08em', marginBottom: '12px' }}>Tasks ({tasks.length})</div>
          {tasks.slice(0, 8).map(t => (
            <div key={t.id} style={{ display: 'flex', justifyContent: 'space-between', padding: '6px 0', borderBottom: '1px solid #1a1a1a', fontSize: '12px' }}>
              <span style={{ whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis', maxWidth: '180px' }}>{t.title}</span>
              <span style={{ color: t.status === 'Completed' ? '#22c55e' : t.status === 'Running' ? '#eab308' : '#666' }}>{t.status}</span>
            </div>
          ))}
          {tasks.length === 0 && <div style={{ color: '#666', fontSize: '12px' }}>No tasks yet</div>}
        </div>
      </div>
    </div>
  );
}

function AgentsView({ agents }: { agents: any[] }) {
  return (
    <div>
      <h2 style={{ fontSize: '16px', fontWeight: 600, marginBottom: '16px' }}>Agents</h2>
      <div style={{ display: 'grid', gap: '12px' }}>
        {agents.map(agent => (
          <div key={agent.id} style={{ border: '1px solid #1f1f1f', background: '#111', padding: '16px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
              <div>
                <div style={{ fontWeight: 600, fontSize: '14px' }}>{agent.name}</div>
                <div style={{ fontSize: '12px', color: '#888' }}>{agent.role}</div>
              </div>
              <div style={{ textAlign: 'right' }}>
                <div style={{ fontSize: '11px', padding: '2px 8px', background: agent.state === 'Executing' ? '#22c55e20' : '#1a1a1a', color: agent.state === 'Executing' ? '#22c55e' : '#888', borderRadius: '10px' }}>{agent.state}</div>
                <div style={{ fontSize: '11px', color: '#666', marginTop: '4px' }}>{agent.model}</div>
              </div>
            </div>
            <div style={{ fontSize: '12px', color: '#aaa', marginBottom: '8px' }}>{agent.current_action || 'Idle'}</div>
            <div style={{ height: '4px', background: '#1f1f1f', borderRadius: '2px', overflow: 'hidden', marginBottom: '8px' }}>
              <div style={{ height: '100%', width: `${agent.progress}%`, background: '#fff' }} />
            </div>
            <div style={{ display: 'flex', gap: '12px', fontSize: '11px', color: '#666' }}>
              <span>Files: {agent.files_changed?.length || 0}</span>
              <span>Tools: {agent.tools_used?.length || 0}</span>
              <span>Tokens: {agent.metrics?.tokens_used || 0}</span>
            </div>
          </div>
        ))}
        {agents.length === 0 && <div style={{ color: '#666' }}>No agents. Create a session and add agents via API or CLI.</div>}
      </div>
    </div>
  );
}

function TasksView({ tasks }: { tasks: any[] }) {
  return (
    <div>
      <h2 style={{ fontSize: '16px', fontWeight: 600, marginBottom: '16px' }}>Task Graph</h2>
      <div style={{ border: '1px solid #1f1f1f', background: '#111', padding: '16px', marginBottom: '16px' }}>
        <div style={{ display: 'flex', gap: '16px', fontSize: '12px' }}>
          <span>Total: {tasks.length}</span>
          <span style={{ color: '#22c55e' }}>Completed: {tasks.filter(t => t.status === 'Completed').length}</span>
          <span style={{ color: '#eab308' }}>Running: {tasks.filter(t => t.status === 'Running').length}</span>
          <span style={{ color: '#ef4444' }}>Blocked: {tasks.filter(t => t.status === 'Blocked').length}</span>
        </div>
      </div>
      <div style={{ display: 'grid', gap: '8px' }}>
        {tasks.map(task => (
          <div key={task.id} style={{ border: '1px solid #1f1f1f', background: '#111', padding: '12px', display: 'flex', gap: '12px', alignItems: 'center' }}>
            <div style={{ width: '8px', height: '8px', borderRadius: '50%', background: task.status === 'Completed' ? '#22c55e' : task.status === 'Running' ? '#eab308' : task.status === 'Blocked' ? '#ef4444' : '#444', flexShrink: 0 }} />
            <div style={{ flex: 1 }}>
              <div style={{ fontSize: '13px', fontWeight: 500 }}>{task.title}</div>
              {task.description && <div style={{ fontSize: '11px', color: '#888' }}>{task.description.slice(0, 100)}</div>}
            </div>
            <div style={{ fontSize: '11px', color: '#666' }}>{task.status} {task.progress}%</div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default function App() {
  const [selectedView, setSelectedView] = useState('project');
  const [selectedSession, setSelectedSession] = useState<string | undefined>();
  const { sessions } = useSessions();
  const { events, connected } = useForgeEvents(selectedSession);
  const agents = useAgents(selectedSession);
  const tasks = useTasks(selectedSession);

  React.useEffect(() => {
    if (sessions.length > 0 && !selectedSession) {
      setSelectedSession(sessions[0].id);
    }
  }, [sessions, selectedSession]);

  let mainContent: React.ReactNode = null;
  switch (selectedView) {
    case 'project':
      mainContent = <ProjectView sessionId={selectedSession} />;
      break;
    case 'agents':
      mainContent = <AgentsView agents={agents} />;
      break;
    case 'tasks':
      mainContent = <TasksView tasks={tasks} />;
      break;
    default:
      mainContent = (
        <div style={{ padding: '24px' }}>
          <h2 style={{ fontSize: '16px', fontWeight: 600, marginBottom: '8px', textTransform: 'capitalize' }}>{selectedView}</h2>
          <p style={{ color: '#888', fontSize: '13px' }}>View for {selectedView} - implementation in progress. This panel shows real data from Forge Core via WebSocket.</p>
          <div style={{ marginTop: '16px', padding: '12px', background: '#111', border: '1px solid #1f1f1f', fontSize: '12px' }}>
            <div>Connected: {connected ? 'yes' : 'no'}</div>
            <div>Events: {events.length}</div>
            <div>Session: {selectedSession || 'none'}</div>
          </div>
        </div>
      );
  }

  return (
    <Layout
      sidebar={<Sidebar selected={selectedView} onSelect={setSelectedView} sessions={sessions} />}
      activity={<ActivityPanel events={events} agents={agents} tasks={tasks} />}
      terminal={<Terminal />}
    >
      {mainContent}
    </Layout>
  );
}
