import * as React from 'react';
import * as ReactDOM from 'react-dom/client';

function ElectronApp() {
  const [status, setStatus] = React.useState<any>(null);

  React.useEffect(() => {
    // @ts-ignore
    if (window.forge) {
      // @ts-ignore
      window.forge.getStatus().then(setStatus).catch(console.error);
      const interval = setInterval(() => {
        // @ts-ignore
        window.forge.getStatus().then(setStatus).catch(() => {});
      }, 3000);
      return () => clearInterval(interval);
    } else {
      fetch('http://localhost:3000/api/health').then(r => r.json()).then(setStatus).catch(() => setStatus({ status: 'offline' }));
    }
  }, []);

  return (
    <div style={{ padding: '24px', background: '#0a0a0a', color: '#e5e5e5', minHeight: '100vh', fontFamily: 'monospace' }}>
      <h1 style={{ fontSize: '18px', marginBottom: '16px' }}>Forge - Electron (Full Compatibility)</h1>
      <div style={{ padding: '12px', background: '#111', border: '1px solid #222', marginBottom: '16px', fontSize: '13px' }}>
        <div>Core Status: {status ? JSON.stringify(status) : 'Loading...'}</div>
      </div>
      <p style={{ fontSize: '13px', color: '#888', lineHeight: '1.6' }}>
        Electron client provides full desktop integration while remaining a client of Forge Core.
        Same protocol as Tauri, Web, and CLI. No duplicated orchestration logic.
      </p>
    </div>
  );
}

const root = document.getElementById('root');
if (root) {
  ReactDOM.createRoot(root).render(<ElectronApp />);
}
