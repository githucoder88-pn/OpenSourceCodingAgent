import React from 'react';

interface SidebarProps {
  selected: string;
  onSelect: (id: string) => void;
  sessions: any[];
}

export function Sidebar({ selected, onSelect, sessions }: SidebarProps) {
  const items = [
    { id: 'project', label: 'Project', icon: '◧' },
    { id: 'agents', label: 'Agents', icon: '◩', count: 6 },
    { id: 'tasks', label: 'Tasks', icon: '☰', count: 12 },
    { id: 'teams', label: 'Teams', icon: '⧉' },
    { id: 'models', label: 'Models', icon: '◐' },
    { id: 'tools', label: 'Tools', icon: '⚒' },
    { id: 'workspace', label: 'Workspace', icon: '◫' },
    { id: 'memory', label: 'Memory', icon: '◑' },
    { id: 'logs', label: 'Logs', icon: '≡' },
    { id: 'settings', label: 'Settings', icon: '⚙' },
  ];

  return (
    <div style={{ padding: '12px 0' }}>
      <div style={{ padding: '0 12px 8px', fontSize: '10px', fontWeight: 600, color: '#666', letterSpacing: '0.08em', textTransform: 'uppercase' }}>Navigation</div>
      {items.map(item => (
        <div
          key={item.id}
          onClick={() => onSelect(item.id)}
          style={{
            padding: '8px 12px',
            fontSize: '13px',
            cursor: 'pointer',
            background: selected === item.id ? '#1a1a1a' : 'transparent',
            borderLeft: selected === item.id ? '2px solid #fff' : '2px solid transparent',
            display: 'flex',
            alignItems: 'center',
            gap: '8px',
            color: selected === item.id ? '#fff' : '#aaa',
          }}
        >
          <span style={{ width: '16px', textAlign: 'center' }}>{item.icon}</span>
          <span>{item.label}</span>
          {item.count && <span style={{ marginLeft: 'auto', fontSize: '11px', background: '#222', padding: '1px 6px', borderRadius: '10px' }}>{item.count}</span>}
        </div>
      ))}

      <div style={{ marginTop: '24px', padding: '0 12px 8px', fontSize: '10px', fontWeight: 600, color: '#666', letterSpacing: '0.08em', textTransform: 'uppercase' }}>Sessions</div>
      {sessions.slice(0, 5).map(s => (
        <div key={s.id} style={{ padding: '6px 12px', fontSize: '12px', color: '#888', cursor: 'pointer', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
          {s.name}
        </div>
      ))}
    </div>
  );
}
