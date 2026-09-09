export type MemoryScope = 
  | { type: "global" }
  | { type: "project"; path: string }
  | { type: "session"; id: string }
  | { type: "team"; id: string }
  | { type: "agent"; id: string }
  | { type: "task"; id: string };

export interface MemoryEntry {
  id: string;
  scope: MemoryScope;
  category: string;
  key: string;
  value: string;
  created_at: string;
  updated_at: string;
  tags: string[];
}

export class MemoryStore {
  private entries = new Map<string, MemoryEntry>();

  add(entry: Omit<MemoryEntry, "id" | "created_at" | "updated_at">): MemoryEntry {
    const now = new Date().toISOString();
    const full: MemoryEntry = {
      id: crypto.randomUUID(),
      created_at: now,
      updated_at: now,
      ...entry,
    };
    this.entries.set(full.id, full);
    return full;
  }

  get(id: string): MemoryEntry | undefined {
    return this.entries.get(id);
  }

  search(query: string, scope?: MemoryScope): MemoryEntry[] {
    const q = query.toLowerCase();
    return Array.from(this.entries.values()).filter(e => {
      const scopeMatch = scope ? JSON.stringify(e.scope) === JSON.stringify(scope) : true;
      return scopeMatch && (e.key.toLowerCase().includes(q) || e.value.toLowerCase().includes(q) || e.category.toLowerCase().includes(q));
    });
  }

  listByScope(scope: MemoryScope): MemoryEntry[] {
    return Array.from(this.entries.values()).filter(e => JSON.stringify(e.scope) === JSON.stringify(scope));
  }

  listByCategory(category: string): MemoryEntry[] {
    return Array.from(this.entries.values()).filter(e => e.category === category);
  }

  all(): MemoryEntry[] {
    return Array.from(this.entries.values());
  }
}
