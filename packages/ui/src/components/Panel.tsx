import React from 'react';

export function Panel({ title, children, style }: { title?: string; children: React.ReactNode; style?: React.CSSProperties }) {
  return (
    <div style={{ border: '1px solid #1f1f1f', background: '#111', ...style }}>
      {title && (
        <div style={{ padding: '8px 12px', borderBottom: '1px solid #1f1f1f', fontSize: '11px', fontWeight: 600, color: '#666', textTransform: 'uppercase', letterSpacing: '0.08em' }}>
          {title}
        </div>
      )}
      <div style={{ padding: '12px' }}>{children}</div>
    </div>
  );
}
