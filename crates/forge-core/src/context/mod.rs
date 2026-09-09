use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextStats {
    pub total_tokens: usize,
    pub max_tokens: usize,
    pub files_included: usize,
    pub system_tokens: usize,
    pub instruction_tokens: usize,
    pub task_tokens: usize,
    pub memory_tokens: usize,
}

impl ContextStats {
    pub fn usage_percent(&self) -> f32 {
        if self.max_tokens == 0 { 0.0 } else { (self.total_tokens as f32 / self.max_tokens as f32) * 100.0 }
    }
}

#[derive(Debug, Clone)]
pub struct ContextBuilder {
    max_tokens: usize,
    system_context: String,
    project_instructions: HashMap<PathBuf, String>,
    selected_files: Vec<(PathBuf, String)>,
    task_context: String,
    memory_context: String,
}

impl ContextBuilder {
    pub fn new(max_tokens: usize) -> Self {
        Self {
            max_tokens,
            system_context: "You are Forge, an AI coding agent. Help the user with engineering tasks.".into(),
            project_instructions: HashMap::new(),
            selected_files: Vec::new(),
            task_context: String::new(),
            memory_context: String::new(),
        }
    }

    pub fn load_instructions(&mut self, root: &Path) -> anyhow::Result<()> {
        // Load AGENTS.md recursively
        self.load_instructions_recursive(root, root)
    }

    fn load_instructions_recursive(&mut self, root: &Path, current: &Path) -> anyhow::Result<()> {
        let agents_path = current.join("AGENTS.md");
        if agents_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&agents_path) {
                self.project_instructions.insert(agents_path, content);
            }
        }
        if let Ok(entries) = std::fs::read_dir(current) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Skip hidden and common ignores
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        if !name.starts_with('.') && name != "node_modules" && name != "target" && name != "dist" {
                            let _ = self.load_instructions_recursive(root, &path);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn add_file(&mut self, path: PathBuf, content: String) {
        self.selected_files.push((path, content));
    }

    pub fn set_task(&mut self, task: String) {
        self.task_context = task;
    }

    pub fn set_memory(&mut self, memory: String) {
        self.memory_context = memory;
    }

    pub fn build(&self) -> (String, ContextStats) {
        let mut context = String::new();
        let mut tokens = 0;

        context.push_str(&format!("SYSTEM: {}\n\n", self.system_context));
        tokens += self.system_context.len() / 4;

        for (path, content) in &self.project_instructions {
            context.push_str(&format!("INSTRUCTIONS {}:\n{}\n\n", path.display(), content));
            tokens += content.len() / 4;
        }

        if !self.memory_context.is_empty() {
            context.push_str(&format!("MEMORY:\n{}\n\n", self.memory_context));
            tokens += self.memory_context.len() / 4;
        }

        if !self.task_context.is_empty() {
            context.push_str(&format!("TASK:\n{}\n\n", self.task_context));
            tokens += self.task_context.len() / 4;
        }

        for (path, content) in &self.selected_files {
            // Truncate if needed
            let truncated = if content.len() > 8000 {
                format!("{}... [truncated {} chars]", &content[..8000], content.len() - 8000)
            } else {
                content.clone()
            };
            context.push_str(&format!("FILE {}:\n{}\n\n", path.display(), truncated));
            tokens += truncated.len() / 4;
        }

        let stats = ContextStats {
            total_tokens: tokens,
            max_tokens: self.max_tokens,
            files_included: self.selected_files.len(),
            system_tokens: self.system_context.len() / 4,
            instruction_tokens: self.project_instructions.values().map(|c| c.len() / 4).sum(),
            task_tokens: self.task_context.len() / 4,
            memory_tokens: self.memory_context.len() / 4,
        };

        (context, stats)
    }

    pub fn compact(&mut self) {
        // Simple compaction: keep only most recent files, truncate old ones
        if self.selected_files.len() > 10 {
            self.selected_files = self.selected_files.split_off(self.selected_files.len() - 10);
        }
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        Self::new(128000)
    }
}
