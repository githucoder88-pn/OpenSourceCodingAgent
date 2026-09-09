import React from 'react';

export function Terminal({ lines }: { lines: string[] }) {
  return (
    <div style={{ background: '#0a0a0a', color: '#e5e5e5', fontFamily: 'JetBrains Mono, monospace', fontSize: '12px', padding: '12px', overflow: 'auto' }}>
      {lines.map((line, i) => (
        <div key={i} style={{ lineHeight: '1.5' }}>{line}</div>
      ))}
    </div>
  );
}
