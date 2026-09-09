import React from 'react';

interface LayoutProps {
  children: React.ReactNode;
  sidebar: React.ReactNode;
  activity: React.ReactNode;
  terminal: React.ReactNode;
}

export function Layout({ children, sidebar, activity, terminal }: LayoutProps) {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100vh', background: '#0a0a0a', color: '#e5e5e5' }}>
      {/* Header */}
      <div style={{ height: '40px', borderBottom: '1px solid #1f1f1f', display: 'flex', alignItems: 'center', padding: '0 16px', gap: '16px', background: '#111' }}>
        <div style={{ fontWeight: 700, fontSize: '14px', letterSpacing: '-0.02em' }}>FORGE</div>
        <div style={{ fontSize: '12px', color: '#888' }}>project: my-app</div>
        <div style={{ marginLeft: 'auto', display: 'flex', gap: '12px', fontSize: '12px' }}>
          <span style={{ color: '#888' }}>model: gpt-4o</span>
          <span style={{ color: '#888' }}>agents: 6</span>
          <span style={{ color: '#22c55e' }}>● connected</span>
        </div>
      </div>

      <div style={{ display: 'flex', flex: 1, overflow: 'hidden' }}>
        {/* Sidebar */}
        <div style={{ width: '220px', borderRight: '1px solid #1f1f1f', background: '#0f0f0f', overflow: 'auto' }}>
          {sidebar}
        </div>

        {/* Main */}
        <div style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
          <div style={{ flex: 1, overflow: 'auto', padding: '16px' }}>
            {children}
          </div>
          {/* Terminal */}
          <div style={{ height: '200px', borderTop: '1px solid #1f1f1f', background: '#0f0f0f' }}>
            {terminal}
          </div>
        </div>

        {/* Activity */}
        <div style={{ width: '300px', borderLeft: '1px solid #1f1f1f', background: '#0f0f0f', overflow: 'auto' }}>
          {activity}
        </div>
      </div>
    </div>
  );
}
