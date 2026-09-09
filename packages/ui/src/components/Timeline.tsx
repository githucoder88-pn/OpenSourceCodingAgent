import React from 'react';

interface TimelineEvent {
  id: string;
  timestamp: string;
  type: string;
  message: string;
}

export function Timeline({ events }: { events: TimelineEvent[] }) {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
      {events.map(event => (
        <div key={event.id} style={{ display: 'flex', gap: '12px', fontSize: '12px' }}>
          <div style={{ color: '#666', fontSize: '11px', minWidth: '60px' }}>{new Date(event.timestamp).toLocaleTimeString()}</div>
          <div style={{ flex: 1 }}>
            <div style={{ color: '#e5e5e5' }}>{event.message}</div>
            <div style={{ color: '#666', fontSize: '11px' }}>{event.type}</div>
          </div>
        </div>
      ))}
    </div>
  );
}
