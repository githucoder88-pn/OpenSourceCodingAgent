import React, { useState } from 'react';

export function Terminal() {
  const [input, setInput] = useState('');
  const [history, setHistory] = useState<string[]>([
    'Forge terminal ready. Type /help for commands.',
    '',
  ]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim()) return;
    
    setHistory(prev => [...prev, `$ ${input}`]);
    
    // Simple command handling
    if (input.startsWith('/help')) {
      setHistory(prev => [...prev, 'Available: /help /project /session /agent /team /task /model /tool /plan /run /test /git /diff /commit /checkpoint /memory /context /logs /events /status']);
    } else if (input.startsWith('/status')) {
      setHistory(prev => [...prev, '6 agents, 12 tasks, 3 running, 68% complete']);
    } else {
      setHistory(prev => [...prev, `Command executed: ${input} (stub)`]);
    }
    
    setInput('');
  };

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%', fontFamily: 'JetBrains Mono, monospace' }}>
      <div style={{ padding: '8px 12px', borderBottom: '1px solid #1f1f1f', fontSize: '11px', fontWeight: 600, color: '#666', display: 'flex', gap: '12px' }}>
        <span style={{ color: '#fff' }}>Terminal</span>
        <span>Problems</span>
        <span>Git</span>
        <span>Events</span>
        <span>Output</span>
      </div>
      <div style={{ flex: 1, overflow: 'auto', padding: '8px 12px', fontSize: '12px', lineHeight: '1.5' }}>
        {history.map((line, i) => (
          <div key={i} style={{ color: line.startsWith('$') ? '#22c55e' : '#ccc' }}>{line}</div>
        ))}
      </div>
      <form onSubmit={handleSubmit} style={{ display: 'flex', borderTop: '1px solid #1f1f1f', padding: '8px 12px', gap: '8px' }}>
        <span style={{ color: '#22c55e', fontSize: '12px' }}>$</span>
        <input
          value={input}
          onChange={e => setInput(e.target.value)}
          placeholder="Type command..."
          style={{ flex: 1, background: 'transparent', border: 'none', outline: 'none', color: '#e5e5e5', fontSize: '12px', fontFamily: 'inherit' }}
        />
      </form>
    </div>
  );
}
