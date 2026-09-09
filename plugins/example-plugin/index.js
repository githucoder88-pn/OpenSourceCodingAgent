// Example Forge Plugin
export const my_custom_tool = {
  definition: {
    name: "my_custom_tool",
    description: "Example custom tool that processes input",
    parameters: {
      type: "object",
      properties: {
        input: { type: "string", description: "Input to process" }
      },
      required: ["input"]
    },
    permissions: ["read"],
    timeout_ms: 10000,
  },
  async execute(input, context) {
    return {
      processed: input.input.toUpperCase(),
      workspace: context.workspace_path,
      timestamp: new Date().toISOString(),
    };
  }
};

export const commands = {
  "/mycommand": {
    description: "My custom command",
    handler: async (args, context) => {
      return `Executed mycommand with args: ${args.join(" ")}`;
    }
  }
};

export function activate(forge) {
  console.log("Example plugin activated");
  forge.toolRegistry.register(my_custom_tool);
}

export function deactivate() {
  console.log("Example plugin deactivated");
}
