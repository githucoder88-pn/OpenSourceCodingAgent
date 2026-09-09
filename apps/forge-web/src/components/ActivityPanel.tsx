import React from 'react';
import { ForgeEvent } from '../lib/protocol';

interface ActivityPanelProps {
  events: ForgeEvent[];
  agents: any[];
  tasks: any[];
}

export function ActivityPanel({ events, agents, tasks }: ActivityPanelProps) {
  const runningAgents = agents.filter(a => a.state === 'Executing' || a.state === 'Planning');
  const completedTasks = tasks.filter(t => t.status === 'Completed').length;

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
      <div style={{ padding: '12px', borderBottom: '1px solid #1f1f1f' }}>
        <div style={{ fontSize: '11px', fontWeight: 600, color: '#666', letterSpacing: '0.08em', textTransform: 'uppercase', marginBottom: '12px' }}>Agents</div>
        {agents.slice(0, 5).map(agent => (
          <div key={agent.id} style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '6px 0', fontSize: '12px', borderBottom: '1px solid #151515' }}>
            <div style={{ width: '6px', height: '6px', borderRadius: '50%', background: agent.state === 'Executing' ? '#22c55e' : agent.state === 'Paused' ? '#eab308' : '#444' }} />
            <div style={{ flex: 1 }}>
              <div style={{ color: '#e5e5e5', fontWeight: 500 }}>{agent.name}</div>
              <div style={{ color: '#888', fontSize: '11px' }}>{agent.role} • {agent.progress}%</div>
            </div>
            <div style={{ fontSize: '10px', color: '#666' }}>{agent.state}</div>
          </div>
        ))}
      </div>

      <div style={{ padding: '12px', borderBottom: '1px solid #1f1f1f' }}>
        <div style={{ fontSize: '11px', fontWeight: 600, color: '#666', letterSpacing: '0.08em', textTransform: 'uppercase', marginBottom: '8px' }}>Progress</div>
        <div style={{ fontSize: '24px', fontWeight: 700, color: '#fff' }}>{tasks.length ? Math.round((completedTasks / tasks.length) * 100) : 0}%</div>
        <div style={{ fontSize: '11px', color: '#888', marginTop: '4px' }}>{completedTasks}/{tasks.length} tasks • {runningAgents.length} running</div>
        <div style={{ marginTop: '8px', height: '4px', background: '#1f1f1f', borderRadius: '2px', overflow: 'hidden' }}>
          <div style={{ height: '100%', width: `${tasks.length ? (completedTasks / tasks.length) * 100 : 0}%`, background: '#fff', transition: 'width 0.3s' }} />
        </div>
      </div>

      <div style={{ flex: 1, overflow: 'auto', padding: '12px' }}>
        <div style={{ fontSize: '11px', fontWeight: 600, color: '#666', letterSpacing: '0.08em', textTransform: 'uppercase', marginBottom: '8px' }}>Events</div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
          {events.slice(-20).reverse().map(event => (
            <div key={event.id} style={{ fontSize: '11px', padding: '6px 8px', background: '#111', border: '1px solid #1f1f1f', borderRadius: '4px' }}>
              <div style={{ color: '#888', fontSize: '10px' }}>{new Date(event.timestamp).toLocaleTimeString()} • {event.payload.type}</div>
              <div style={{ color: '#ccc', marginTop: '2px', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                {JSON.stringify(event.payload.data).slice(0, 80)}
              </div>
            </div>
          ))}
          {events.length === 0 && <div style={{ color: '#666', fontSize: '11px' }}>No events yet. Start a session to see activity.</div>}
        </div>
      </div>
    </div>
  );
}
