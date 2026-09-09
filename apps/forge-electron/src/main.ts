import { app, BrowserWindow, ipcMain } from 'electron';
import * as path from 'path';

let mainWindow: BrowserWindow | null = null;

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1400,
    height: 900,
    backgroundColor: '#0a0a0a',
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, 'preload.js'),
    },
    title: 'Forge - AI Engineering Platform',
  });

  // In dev, load vite server, in prod load dist
  const isDev = !app.isPackaged;
  if (isDev) {
    mainWindow.loadURL('http://localhost:5174');
  } else {
    mainWindow.loadFile(path.join(__dirname, '../renderer/index.html'));
  }

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
}

app.whenReady().then(() => {
  createWindow();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit();
});

// IPC handlers that proxy to Forge Core
// The Core is the source of truth - Electron is just a client
ipcMain.handle('forge:get-status', async () => {
  try {
    const res = await fetch('http://localhost:3000/api/health');
    return await res.json();
  } catch (e: any) {
    return { error: e.message, status: 'offline' };
  }
});

ipcMain.handle('forge:list-sessions', async () => {
  try {
    const res = await fetch('http://localhost:3000/api/sessions');
    return await res.json();
  } catch (e: any) {
    return { error: e.message };
  }
});

console.log('Forge Electron main process started - connecting to Forge Core at http://localhost:3000');
