export type RoutingStrategy = "manual" | "priority" | "lowest_cost" | "lowest_latency" | "highest_quality" | "adaptive" | "local_first" | "cloud_first";

export interface ModelInfo {
  id: string;
  provider: string;
  name: string;
  context_window: number;
  capabilities: {
    text_generation: boolean;
    streaming: boolean;
    tool_calling: boolean;
    vision: boolean;
    structured_output: boolean;
    reasoning: boolean;
  };
  pricing?: {
    input_per_mtok?: number;
    output_per_mtok?: number;
  };
}

export interface ProviderStatus {
  provider: string;
  health: "available" | "degraded" | "rate_limited" | "offline";
  latency_ms?: number;
  error_rate: number;
  last_check: string;
}

export interface RoutingRequest {
  task_complexity: number;
  required_capabilities: ModelInfo["capabilities"];
  context_size: number;
  latency_requirement?: number;
  budget?: number;
  preferred_provider?: string;
  strategy: RoutingStrategy;
}

export interface RoutingDecision {
  provider: string;
  model: string;
  fallback_chain: [string, string][];
  reasoning: string;
  estimated_cost?: number;
}

export class ModelRouter {
  constructor(private strategy: RoutingStrategy = "adaptive", private fallbackEnabled = true) {}

  route(request: RoutingRequest, providers: Map<string, ProviderStatus>, models: ModelInfo[]): RoutingDecision {
    let candidates = models.filter(m => {
      const status = providers.get(m.provider);
      return status?.health === "available";
    });

    if (candidates.length === 0) {
      candidates = models;
    }

    // Sort by strategy
    switch (request.strategy || this.strategy) {
      case "lowest_cost":
        candidates.sort((a, b) => (a.pricing?.input_per_mtok || 999) - (b.pricing?.input_per_mtok || 999));
        break;
      case "lowest_latency":
        candidates.sort((a, b) => {
          const aLat = providers.get(a.provider)?.latency_ms || 9999;
          const bLat = providers.get(b.provider)?.latency_ms || 9999;
          return aLat - bLat;
        });
        break;
      case "highest_quality":
        candidates.sort((a, b) => b.context_window - a.context_window);
        break;
      case "local_first":
        candidates.sort((a, b) => {
          const aLocal = ["ollama", "lmstudio"].includes(a.provider) ? 0 : 1;
          const bLocal = ["ollama", "lmstudio"].includes(b.provider) ? 0 : 1;
          return aLocal - bLocal;
        });
        break;
      case "cloud_first":
        candidates.sort((a, b) => {
          const aLocal = ["ollama", "lmstudio"].includes(a.provider) ? 1 : 0;
          const bLocal = ["ollama", "lmstudio"].includes(b.provider) ? 1 : 0;
          return aLocal - bLocal;
        });
        break;
      default:
        // adaptive: prefer larger context for complex tasks
        if (request.task_complexity > 7) {
          candidates.sort((a, b) => b.context_window - a.context_window);
        }
        break;
    }

    if (request.preferred_provider) {
      const preferred = candidates.find(c => c.provider === request.preferred_provider);
      if (preferred) {
        candidates = [preferred, ...candidates.filter(c => c !== preferred)];
      }
    }

    const primary = candidates[0];
    const fallbackChain: [string, string][] = candidates.slice(1, 3).map(m => [m.provider, m.id]);

    if (primary) {
      return {
        provider: primary.provider,
        model: primary.id,
        fallback_chain: fallbackChain,
        reasoning: `Selected via ${request.strategy} strategy, primary: ${primary.id} from ${primary.provider}`,
      };
    }

    return {
      provider: "ollama",
      model: "llama3.1",
      fallback_chain: [],
      reasoning: "No healthy providers, falling back to local ollama",
    };
  }

  shouldFallback(error: string): boolean {
    if (!this.fallbackEnabled) return false;
    return error.includes("rate_limited") || error.includes("unavailable") || error.includes("offline") || error.includes("429") || error.includes("503");
  }
}

export const DEFAULT_MODELS: ModelInfo[] = [
  { id: "gpt-4o", provider: "openai", name: "GPT-4o", context_window: 128000, capabilities: { text_generation: true, streaming: true, tool_calling: true, vision: true, structured_output: true, reasoning: false }, pricing: { input_per_mtok: 2.5, output_per_mtok: 10 } },
  { id: "gpt-4o-mini", provider: "openai", name: "GPT-4o Mini", context_window: 128000, capabilities: { text_generation: true, streaming: true, tool_calling: true, vision: true, structured_output: true, reasoning: false }, pricing: { input_per_mtok: 0.15, output_per_mtok: 0.6 } },
  { id: "claude-3-5-sonnet-20241022", provider: "anthropic", name: "Claude 3.5 Sonnet", context_window: 200000, capabilities: { text_generation: true, streaming: true, tool_calling: true, vision: true, structured_output: true, reasoning: true }, pricing: { input_per_mtok: 3, output_per_mtok: 15 } },
  { id: "gemini-1.5-pro", provider: "google", name: "Gemini 1.5 Pro", context_window: 1000000, capabilities: { text_generation: true, streaming: true, tool_calling: true, vision: true, structured_output: true, reasoning: false }, pricing: { input_per_mtok: 1.25, output_per_mtok: 5 } },
  { id: "llama3.1", provider: "ollama", name: "Llama 3.1", context_window: 32768, capabilities: { text_generation: true, streaming: true, tool_calling: true, vision: false, structured_output: false, reasoning: false } },
  { id: "qwen2.5-coder", provider: "ollama", name: "Qwen 2.5 Coder", context_window: 32768, capabilities: { text_generation: true, streaming: true, tool_calling: true, vision: false, structured_output: false, reasoning: false } },
];
