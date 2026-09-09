# Models

Provider trait with: text generation, streaming, tool calling, vision, structured output, reasoning config, context window, token accounting.

Adapters: OpenAI, Anthropic, Google, Azure, OpenRouter, Ollama, LM Studio, compatible.

Router: manual, priority, lowest_cost, lowest_latency, highest_quality, adaptive, local_first, cloud_first

Health: available, degraded, rate_limited, offline

Fallback chain: Primary → Secondary → Local
