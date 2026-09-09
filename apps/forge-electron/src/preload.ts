import { contextBridge, ipcRenderer } from 'electron';

contextBridge.exposeInMainWorld('forge', {
  getStatus: () => ipcRenderer.invoke('forge:get-status'),
  listSessions: () => ipcRenderer.invoke('forge:list-sessions'),
  // All other operations go through HTTP/WebSocket to Forge Core
  // This keeps Electron as a thin client
});

console.log('Forge Electron preload loaded');
