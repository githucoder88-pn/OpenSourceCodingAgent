use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MemoryScope {
    Global,
    Project,
    Session(Uuid),
    Team(Uuid),
    Agent(Uuid),
    Task(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: Uuid,
    pub scope: MemoryScope,
    pub category: String,
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

impl MemoryEntry {
    pub fn new(scope: MemoryScope, category: String, key: String, value: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            scope,
            category,
            key,
            value,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
        }
    }
}

pub struct MemoryStore {
    entries: HashMap<Uuid, MemoryEntry>,
    project_path: Option<PathBuf>,
}

impl MemoryStore {
    pub fn new(project_path: Option<PathBuf>) -> Self {
        Self { entries: HashMap::new(), project_path }
    }

    pub fn add(&mut self, entry: MemoryEntry) -> Uuid {
        let id = entry.id;
        self.entries.insert(id, entry);
        id
    }

    pub fn get(&self, id: Uuid) -> Option<&MemoryEntry> {
        self.entries.get(&id)
    }

    pub fn search(&self, query: &str, scope: Option<MemoryScope>) -> Vec<&MemoryEntry> {
        let q = query.to_lowercase();
        self.entries.values().filter(|e| {
            let scope_match = scope.as_ref().map(|s| &e.scope == s).unwrap_or(true);
            scope_match && (e.key.to_lowercase().contains(&q) || e.value.to_lowercase().contains(&q) || e.category.to_lowercase().contains(&q))
        }).collect()
    }

    pub fn list_by_scope(&self, scope: &MemoryScope) -> Vec<&MemoryEntry> {
        self.entries.values().filter(|e| &e.scope == scope).collect()
    }

    pub fn list_by_category(&self, category: &str) -> Vec<&MemoryEntry> {
        self.entries.values().filter(|e| e.category == category).collect()
    }

    pub fn relevant_for_task(&self, task_description: &str, scope: &MemoryScope) -> Vec<&MemoryEntry> {
        // Simple relevance: search by keywords in task
        let keywords: Vec<&str> = task_description.split_whitespace().collect();
        let mut relevant = Vec::new();
        for entry in self.entries.values().filter(|e| &e.scope == scope || matches!(e.scope, MemoryScope::Global) || matches!(e.scope, MemoryScope::Project)) {
            for kw in &keywords {
                if kw.len() > 3 && (entry.key.to_lowercase().contains(&kw.to_lowercase()) || entry.value.to_lowercase().contains(&kw.to_lowercase())) {
                    relevant.push(entry);
                    break;
                }
            }
        }
        relevant
    }
}

impl Default for MemoryStore {
    fn default() -> Self { Self::new(None) }
}
