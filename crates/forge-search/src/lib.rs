use std::path::{Path, PathBuf};
use anyhow::Result;
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: std::time::SystemTime,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub line: usize,
    pub column: usize,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub kind: SymbolKind,
    pub path: PathBuf,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Class,
    Struct,
    Enum,
    Interface,
    Variable,
    Constant,
    Module,
    Unknown,
}

pub struct Indexer {
    root: PathBuf,
    ignore_patterns: Vec<String>,
}

impl Indexer {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            ignore_patterns: vec![
                "node_modules".into(),
                ".git".into(),
                "target".into(),
                "dist".into(),
                "build".into(),
                ".next".into(),
                "__pycache__".into(),
            ],
        }
    }

    pub fn should_ignore(&self, path: &Path) -> bool {
        let s = path.to_string_lossy();
        self.ignore_patterns.iter().any(|p| s.contains(p))
    }

    pub fn list_files(&self) -> Result<Vec<FileEntry>> {
        let mut entries = Vec::new();
        self.walk_dir(&self.root, &mut entries)?;
        Ok(entries)
    }

    fn walk_dir(&self, dir: &Path, entries: &mut Vec<FileEntry>) -> Result<()> {
        if self.should_ignore(dir) {
            return Ok(());
        }
        let read = std::fs::read_dir(dir)?;
        for entry in read {
            let entry = entry?;
            let path = entry.path();
            if self.should_ignore(&path) {
                continue;
            }
            let metadata = entry.metadata()?;
            entries.push(FileEntry {
                path: path.clone(),
                is_dir: metadata.is_dir(),
                size: metadata.len(),
                modified: metadata.modified().unwrap_or(std::time::SystemTime::now()),
            });
            if metadata.is_dir() {
                self.walk_dir(&path, entries)?;
            }
        }
        Ok(())
    }

    pub fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        let files = self.list_files()?;
        for file_entry in files.iter().filter(|e| !e.is_dir) {
            if let Ok(content) = std::fs::read_to_string(&file_entry.path) {
                for (idx, line) in content.lines().enumerate() {
                    if line.to_lowercase().contains(&query.to_lowercase()) {
                        if let Some(col) = line.to_lowercase().find(&query.to_lowercase()) {
                            results.push(SearchResult {
                                path: file_entry.path.clone(),
                                line: idx + 1,
                                column: col + 1,
                                content: line.trim().to_string(),
                            });
                            if results.len() > 500 {
                                return Ok(results);
                            }
                        }
                    }
                }
            }
        }
        Ok(results)
    }

    pub fn search_symbols(&self, query: &str) -> Result<Vec<SymbolInfo>> {
        let mut symbols = Vec::new();
        let files = self.list_files()?;
        for file_entry in files.iter().filter(|e| !e.is_dir) {
            if let Ok(content) = std::fs::read_to_string(&file_entry.path) {
                for (idx, line) in content.lines().enumerate() {
                    let trimmed = line.trim();
                    // Simple heuristic for symbols
                    if trimmed.starts_with("fn ") || trimmed.starts_with("function ") || trimmed.contains("class ") || trimmed.contains("struct ") || trimmed.contains("enum ") {
                        if trimmed.to_lowercase().contains(&query.to_lowercase()) {
                            let kind = if trimmed.starts_with("fn ") || trimmed.starts_with("function ") {
                                SymbolKind::Function
                            } else if trimmed.contains("class ") {
                                SymbolKind::Class
                            } else if trimmed.contains("struct ") {
                                SymbolKind::Struct
                            } else if trimmed.contains("enum ") {
                                SymbolKind::Enum
                            } else {
                                SymbolKind::Unknown
                            };
                            symbols.push(SymbolInfo {
                                name: trimmed.to_string(),
                                kind,
                                path: file_entry.path.clone(),
                                line: idx + 1,
                                column: 1,
                            });
                        }
                    }
                }
            }
        }
        Ok(symbols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_indexer_new() {
        let idx = Indexer::new(PathBuf::from("/tmp"));
        assert!(!idx.ignore_patterns.is_empty());
    }
}
