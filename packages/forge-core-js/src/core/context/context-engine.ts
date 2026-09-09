import * as fs from "fs";
import * as path from "path";

export interface ContextStats {
  total_tokens: number;
  max_tokens: number;
  files_included: number;
  system_tokens: number;
  instruction_tokens: number;
  task_tokens: number;
  memory_tokens: number;
  usage_percent: number;
}

export class ContextEngine {
  private maxTokens: number;
  private systemContext: string;
  private instructions = new Map<string, string>();
  private selectedFiles: { path: string; content: string }[] = [];
  private taskContext = "";
  private memoryContext = "";

  constructor(maxTokens = 128000) {
    this.maxTokens = maxTokens;
    this.systemContext = "You are Forge, an expert AI coding agent. You help users build, fix, and improve software. You have access to tools for reading, writing, searching, and executing commands.";
  }

  loadInstructions(root: string) {
    this.loadRecursive(root, root);
  }

  private loadRecursive(root: string, current: string) {
    try {
      const agentsPath = path.join(current, "AGENTS.md");
      if (fs.existsSync(agentsPath)) {
        const content = fs.readFileSync(agentsPath, "utf-8");
        this.instructions.set(agentsPath, content);
      }
      const entries = fs.readdirSync(current, { withFileTypes: true });
      for (const entry of entries) {
        if (entry.isDirectory()) {
          const name = entry.name;
          if (!name.startsWith(".") && !["node_modules", "target", "dist", "build", ".next", "__pycache__", ".git"].includes(name)) {
            this.loadRecursive(root, path.join(current, name));
          }
        }
      }
    } catch {}
  }

  addFile(filePath: string, content: string) {
    this.selectedFiles.push({ path: filePath, content });
  }

  setTask(task: string) {
    this.taskContext = task;
  }

  setMemory(memory: string) {
    this.memoryContext = memory;
  }

  build(): { context: string; stats: ContextStats } {
    let context = "";
    let tokens = 0;

    context += `SYSTEM: ${this.systemContext}\n\n`;
    tokens += Math.ceil(this.systemContext.length / 4);

    let instructionTokens = 0;
    for (const [p, content] of this.instructions) {
      context += `INSTRUCTIONS ${p}:\n${content}\n\n`;
      const t = Math.ceil(content.length / 4);
      tokens += t;
      instructionTokens += t;
    }

    let memoryTokens = 0;
    if (this.memoryContext) {
      context += `MEMORY:\n${this.memoryContext}\n\n`;
      memoryTokens = Math.ceil(this.memoryContext.length / 4);
      tokens += memoryTokens;
    }

    let taskTokens = 0;
    if (this.taskContext) {
      context += `TASK:\n${this.taskContext}\n\n`;
      taskTokens = Math.ceil(this.taskContext.length / 4);
      tokens += taskTokens;
    }

    for (const file of this.selectedFiles) {
      const truncated = file.content.length > 8000 ? file.content.slice(0, 8000) + `... [truncated ${file.content.length - 8000} chars]` : file.content;
      context += `FILE ${file.path}:\n${truncated}\n\n`;
      tokens += Math.ceil(truncated.length / 4);
    }

    const stats: ContextStats = {
      total_tokens: tokens,
      max_tokens: this.maxTokens,
      files_included: this.selectedFiles.length,
      system_tokens: Math.ceil(this.systemContext.length / 4),
      instruction_tokens: instructionTokens,
      task_tokens: taskTokens,
      memory_tokens: memoryTokens,
      usage_percent: this.maxTokens ? (tokens / this.maxTokens) * 100 : 0,
    };

    return { context, stats };
  }

  compact() {
    if (this.selectedFiles.length > 10) {
      this.selectedFiles = this.selectedFiles.slice(-10);
    }
  }

  getStats(): ContextStats {
    return this.build().stats;
  }
}
