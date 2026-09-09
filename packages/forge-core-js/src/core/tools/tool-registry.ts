import * as fs from "fs";
import * as path from "path";
import { execSync, spawn } from "child_process";

export interface ToolContext {
  agent_id: string;
  workspace_path: string;
  permissions: string;
}

export interface ToolResult {
  success: boolean;
  output: any;
  error?: string;
  duration_ms: number;
}

export interface Tool {
  definition: {
    name: string;
    description: string;
    parameters: any;
    permissions: string[];
    timeout_ms?: number;
  };
  execute(input: any, context: ToolContext): Promise<any>;
}

export class ToolRegistry {
  private tools = new Map<string, Tool>();

  constructor() {
    this.registerDefaults();
  }

  register(tool: Tool) {
    this.tools.set(tool.definition.name, tool);
  }

  get(name: string): Tool | undefined {
    return this.tools.get(name);
  }

  list() {
    return Array.from(this.tools.values()).map(t => t.definition);
  }

  private registerDefaults() {
    this.register({
      definition: {
        name: "read_file",
        description: "Read a file from workspace",
        parameters: { type: "object", properties: { path: { type: "string" } }, required: ["path"] },
        permissions: ["read"],
      },
      async execute(input, context) {
        const filePath = path.isAbsolute(input.path) ? input.path : path.join(context.workspace_path, input.path);
        const content = fs.readFileSync(filePath, "utf-8");
        return { path: input.path, content };
      }
    });

    this.register({
      definition: {
        name: "write_file",
        description: "Write content to file",
        parameters: { type: "object", properties: { path: { type: "string" }, content: { type: "string" } }, required: ["path", "content"] },
        permissions: ["write"],
      },
      async execute(input, context) {
        const filePath = path.isAbsolute(input.path) ? input.path : path.join(context.workspace_path, input.path);
        fs.mkdirSync(path.dirname(filePath), { recursive: true });
        fs.writeFileSync(filePath, input.content);
        return { path: input.path, written: true, bytes: input.content.length };
      }
    });

    this.register({
      definition: {
        name: "edit_file",
        description: "Edit file by replacing old_text with new_text",
        parameters: { type: "object", properties: { path: { type: "string" }, old_text: { type: "string" }, new_text: { type: "string" } }, required: ["path", "old_text", "new_text"] },
        permissions: ["write"],
      },
      async execute(input, context) {
        const filePath = path.isAbsolute(input.path) ? input.path : path.join(context.workspace_path, input.path);
        const content = fs.readFileSync(filePath, "utf-8");
        if (!content.includes(input.old_text)) throw new Error(`old_text not found in ${input.path}`);
        const newContent = content.replace(input.old_text, input.new_text);
        fs.writeFileSync(filePath, newContent);
        return { path: input.path, edited: true };
      }
    });

    this.register({
      definition: {
        name: "create_file",
        description: "Create new file",
        parameters: { type: "object", properties: { path: { type: "string" }, content: { type: "string" } }, required: ["path"] },
        permissions: ["write"],
      },
      async execute(input, context) {
        const filePath = path.isAbsolute(input.path) ? input.path : path.join(context.workspace_path, input.path);
        if (fs.existsSync(filePath)) throw new Error(`File already exists: ${input.path}`);
        fs.mkdirSync(path.dirname(filePath), { recursive: true });
        fs.writeFileSync(filePath, input.content || "");
        return { path: input.path, created: true };
      }
    });

    this.register({
      definition: {
        name: "delete_file",
        description: "Delete file with safety controls",
        parameters: { type: "object", properties: { path: { type: "string" }, force: { type: "boolean" } }, required: ["path"] },
        permissions: ["write", "delete"],
      },
      async execute(input, context) {
        const protectedPaths = [".git", "node_modules", "target"];
        if (protectedPaths.some(p => input.path.includes(p)) && !input.force) {
          throw new Error(`Refusing to delete protected path: ${input.path}. Use force=true`);
        }
        const filePath = path.isAbsolute(input.path) ? input.path : path.join(context.workspace_path, input.path);
        fs.unlinkSync(filePath);
        return { path: input.path, deleted: true };
      }
    });

    this.register({
      definition: {
        name: "list_directory",
        description: "List directory contents",
        parameters: { type: "object", properties: { path: { type: "string" } } },
        permissions: ["read"],
      },
      async execute(input, context) {
        const dirPath = input.path ? (path.isAbsolute(input.path) ? input.path : path.join(context.workspace_path, input.path)) : context.workspace_path;
        const entries = fs.readdirSync(dirPath, { withFileTypes: true }).map(e => ({
          name: e.name,
          is_dir: e.isDirectory(),
          is_file: e.isFile(),
        }));
        return { path: input.path || ".", entries };
      }
    });

    this.register({
      definition: {
        name: "search_files",
        description: "Search files for query",
        parameters: { type: "object", properties: { query: { type: "string" }, limit: { type: "number" } }, required: ["query"] },
        permissions: ["read"],
      },
      async execute(input, context) {
        const results: any[] = [];
        const walk = (dir: string) => {
          try {
            const entries = fs.readdirSync(dir, { withFileTypes: true });
            for (const entry of entries) {
              if (entry.name.startsWith(".") || ["node_modules", "target", "dist"].includes(entry.name)) continue;
              const full = path.join(dir, entry.name);
              if (entry.isDirectory()) {
                walk(full);
              } else {
                try {
                  const content = fs.readFileSync(full, "utf-8");
                  const lines = content.split("\n");
                  lines.forEach((line, idx) => {
                    if (line.toLowerCase().includes(input.query.toLowerCase())) {
                      if (results.length < (input.limit || 100)) {
                        results.push({ path: full.replace(context.workspace_path, ""), line: idx + 1, content: line.trim() });
                      }
                    }
                  });
                } catch {}
              }
              if (results.length >= (input.limit || 100)) return;
            }
          } catch {}
        };
        walk(context.workspace_path);
        return { query: input.query, count: results.length, results };
      }
    });

    this.register({
      definition: {
        name: "shell",
        description: "Execute shell command",
        parameters: { type: "object", properties: { command: { type: "string" }, cwd: { type: "string" } }, required: ["command"] },
        permissions: ["shell"],
        timeout_ms: 120000,
      },
      async execute(input, context) {
        const cwd = input.cwd || context.workspace_path;
        const dangerous = ["rm -rf /", "rm -rf ~", ":(){:|:&};:", "mkfs", "dd if=", "> /dev/sda"];
        for (const pattern of dangerous) {
          if (input.command.includes(pattern)) throw new Error(`Dangerous command detected: ${pattern}`);
        }
        const start = Date.now();
        try {
          const stdout = execSync(input.command, { cwd, encoding: "utf-8", timeout: input.timeout_ms || 120000 });
          return { command: input.command, cwd, exit_code: 0, success: true, stdout, stderr: "", duration_ms: Date.now() - start };
        } catch (e: any) {
          return { command: input.command, cwd, exit_code: e.status || 1, success: false, stdout: e.stdout?.toString() || "", stderr: e.stderr?.toString() || e.message, duration_ms: Date.now() - start };
        }
      }
    });

    this.register({
      definition: {
        name: "git_status",
        description: "Get git status",
        parameters: { type: "object", properties: {} },
        permissions: ["read", "git"],
      },
      async execute(_input, context) {
        try {
          const stdout = execSync("git status --porcelain -b", { cwd: context.workspace_path, encoding: "utf-8" });
          return { stdout, success: true };
        } catch (e: any) {
          return { stdout: "", stderr: e.message, success: false };
        }
      }
    });

    this.register({
      definition: {
        name: "git_diff",
        description: "Get git diff",
        parameters: { type: "object", properties: { staged: { type: "boolean" }, path: { type: "string" } } },
        permissions: ["read", "git"],
      },
      async execute(input, context) {
        const args = ["diff"];
        if (input.staged) args.push("--staged");
        if (input.path) args.push(input.path);
        try {
          const stdout = execSync(`git ${args.join(" ")}`, { cwd: context.workspace_path, encoding: "utf-8" });
          return { diff: stdout, success: true };
        } catch (e: any) {
          return { diff: "", stderr: e.message, success: false };
        }
      }
    });

    this.register({
      definition: {
        name: "git_commit",
        description: "Create git commit",
        parameters: { type: "object", properties: { message: { type: "string" } }, required: ["message"] },
        permissions: ["write", "git"],
      },
      async execute(input, context) {
        try {
          const stdout = execSync(`git commit -m "${input.message.replace(/"/g, '\\"')}"`, { cwd: context.workspace_path, encoding: "utf-8" });
          return { stdout, success: true };
        } catch (e: any) {
          return { stdout: "", stderr: e.message, success: false };
        }
      }
    });

    // Capture registry for run_tests tool that needs shell
    const registryRef = this;
    this.register({
      definition: {
        name: "run_tests",
        description: "Run tests",
        parameters: { type: "object", properties: { test_command: { type: "string" } } },
        permissions: ["shell", "test"],
      },
      async execute(input, context) {
        const cmd = input.test_command || (fs.existsSync(path.join(context.workspace_path, "Cargo.toml")) ? "cargo test" : fs.existsSync(path.join(context.workspace_path, "package.json")) ? "npm test" : "pytest");
        const shell = registryRef.get("shell")!;
        return shell.execute({ command: cmd, cwd: context.workspace_path }, context);
      }
    });
  }
}
