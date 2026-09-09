import { useEffect, useState, useRef } from 'react';
import { forgeClient, ForgeEvent } from '../lib/protocol';

export function useForgeEvents(sessionId?: string) {
  const [events, setEvents] = useState<ForgeEvent[]>([]);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const ws = forgeClient.connectWebSocket();
    
    ws.onopen = () => setConnected(true);
    ws.onclose = () => setConnected(false);

    const unsubscribe = forgeClient.onAllEvents((event) => {
      if (!sessionId || event.session_id === sessionId) {
        setEvents(prev => [...prev.slice(-500), event]);
      }
    });

    return () => {
      unsubscribe();
      ws.close();
    };
  }, [sessionId]);

  return { events, connected };
}

export function useSessions() {
  const [sessions, setSessions] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    forgeClient.listSessions()
      .then(setSessions)
      .catch(console.error)
      .finally(() => setLoading(false));
  }, []);

  return { sessions, loading, refresh: () => forgeClient.listSessions().then(setSessions) };
}

export function useAgents(sessionId?: string) {
  const [agents, setAgents] = useState<any[]>([]);
  
  useEffect(() => {
    if (!sessionId) return;
    forgeClient.listAgents(sessionId).then(setAgents).catch(console.error);
    
    const interval = setInterval(() => {
      forgeClient.listAgents(sessionId).then(setAgents).catch(() => {});
    }, 2000);

    return () => clearInterval(interval);
  }, [sessionId]);

  return agents;
}

export function useTasks(sessionId?: string) {
  const [tasks, setTasks] = useState<any[]>([]);
  
  useEffect(() => {
    if (!sessionId) return;
    forgeClient.listTasks(sessionId).then(setTasks).catch(console.error);
    
    const interval = setInterval(() => {
      forgeClient.listTasks(sessionId).then(setTasks).catch(() => {});
    }, 2000);

    return () => clearInterval(interval);
  }, [sessionId]);

  return tasks;
}
