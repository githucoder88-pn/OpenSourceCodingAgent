# Example Forge Plugin

This demonstrates how to create a Forge plugin.

Plugins can add:

- models
- providers
- tools
- commands
- agents
- roles
- integrations
- UI panels
- hooks

## Structure

```json
{
  "name": "example-plugin",
  "version": "0.1.0",
  "description": "Example plugin",
  "main": "index.js",
  "forge": {
    "minVersion": "0.1.0",
    "tools": ["my_custom_tool"],
    "commands": ["/mycommand"],
    "models": ["my-model"]
  }
}
```

## Example Tool

```js
export const my_custom_tool = {
  definition: {
    name: "my_custom_tool",
    description: "My custom tool",
    parameters: { type: "object", properties: { input: { type: "string" } } }
  },
  async execute(input, context) {
    return { result: `Processed ${input.input}` };
  }
}
```
