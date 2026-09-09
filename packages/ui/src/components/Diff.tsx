import React from 'react';

interface DiffProps {
  diff: string;
  mode?: 'unified' | 'split';
}

export function Diff({ diff, mode = 'unified' }: DiffProps) {
  const lines = diff.split('\n');
  return (
    <div style={{ fontFamily: 'JetBrains Mono, monospace', fontSize: '12px', background: '#0a0a0a', border: '1px solid #1f1f1f', overflow: 'auto' }}>
      {lines.map((line, i) => {
        let bg = 'transparent';
        let color = '#ccc';
        if (line.startsWith('+')) { bg = '#22c55e15'; color = '#22c55e'; }
        if (line.startsWith('-')) { bg = '#ef444415'; color = '#ef4444'; }
        if (line.startsWith('@@')) { bg = '#1a1a1a'; color = '#888'; }
        return (
          <div key={i} style={{ background: bg, color, padding: '1px 12px', whiteSpace: 'pre' }}>
            {line || ' '}
          </div>
        );
      })}
    </div>
  );
}
