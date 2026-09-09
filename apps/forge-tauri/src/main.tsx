import React from 'react'
import ReactDOM from 'react-dom/client'

function TauriApp() {
  const [status, setStatus] = React.useState('Connecting to Forge Core...');

  React.useEffect(() => {
    fetch('http://localhost:3000/api/health')
      .then(r => r.json())
      .then(() => setStatus('Connected to Forge Core'))
      .catch(() => setStatus('Forge Core not running - start with: forge server'));
  }, []);

  return (
    <div style={{ padding: '24px', background: '#0a0a0a', color: '#e5e5e5', minHeight: '100vh', fontFamily: 'monospace' }}>
      <h1 style={{ fontSize: '18px', marginBottom: '16px' }}>Forge - Tauri (Lightweight Desktop)</h1>
      <div style={{ padding: '12px', background: '#111', border: '1px solid #222', marginBottom: '16px' }}>
        Status: {status}
      </div>
      <p style={{ fontSize: '13px', color: '#888' }}>
        This is the Tauri client. It connects to the same Forge Core as Electron, CLI, and Web.
        The Core is the product. Every interface is a client of the Core.
      </p>
      <div style={{ marginTop: '24px', fontSize: '12px', color: '#666' }}>
        <div>Architecture:</div>
        <div style={{ marginTop: '8px', whiteSpace: 'pre', background: '#111', padding: '12px', border: '1px solid #1f1f1f' }}>
Tauri App → IPC/HTTP → Forge Core → Agent Runtime → Tools → Workspace
                    ↕ WebSocket for real-time events
        </div>
      </div>
    </div>
  );
}

ReactDOM.createRoot(document.getElementById('root')!).render(<TauriApp />)
