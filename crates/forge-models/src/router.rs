use crate::{ModelRegistry, ProviderHealth};
use forge_protocol::models::{RoutingRequest, RoutingDecision, RoutingStrategy, ProviderStatus, ModelCapabilities};
use std::collections::HashMap;

pub struct ModelRouter {
    strategy: RoutingStrategy,
    fallback_enabled: bool,
}

impl ModelRouter {
    pub fn new(strategy: RoutingStrategy, fallback_enabled: bool) -> Self {
        Self { strategy, fallback_enabled }
    }

    pub fn route(&self, request: &RoutingRequest, providers: &HashMap<String, ProviderStatus>, models: &[forge_protocol::models::ModelInfo]) -> RoutingDecision {
        // Filter by health and capabilities
        let mut candidates: Vec<&forge_protocol::models::ModelInfo> = models.iter().filter(|m| {
            if let Some(status) = providers.get(&m.provider) {
                status.health == forge_protocol::models::ProviderHealth::Available
            } else { false }
        }).collect();

        // Sort by strategy
        match self.strategy {
            RoutingStrategy::LowestCost => {
                candidates.sort_by(|a, b| {
                    let a_cost = a.pricing.as_ref().and_then(|p| p.input_per_mtok).unwrap_or(999.0);
                    let b_cost = b.pricing.as_ref().and_then(|p| p.input_per_mtok).unwrap_or(999.0);
                    a_cost.partial_cmp(&b_cost).unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            RoutingStrategy::LowestLatency => {
                candidates.sort_by(|a, b| {
                    let a_lat = providers.get(&a.provider).and_then(|p| p.latency_ms).unwrap_or(9999);
                    let b_lat = providers.get(&b.provider).and_then(|p| p.latency_ms).unwrap_or(9999);
                    a_lat.cmp(&b_lat)
                });
            }
            RoutingStrategy::HighestQuality => {
                candidates.sort_by(|a, b| b.context_window.cmp(&a.context_window));
            }
            RoutingStrategy::LocalFirst => {
                candidates.sort_by(|a, b| {
                    let a_local = if a.provider == "ollama" || a.provider == "lmstudio" { 0 } else { 1 };
                    let b_local = if b.provider == "ollama" || b.provider == "lmstudio" { 0 } else { 1 };
                    a_local.cmp(&b_local)
                });
            }
            _ => {}
        }

        let primary = candidates.first().cloned();
        let fallback_chain: Vec<(String, String)> = candidates.iter().skip(1).take(2).map(|m| (m.provider.clone(), m.id.clone())).collect();

        if let Some(model) = primary {
            RoutingDecision {
                provider: model.provider.clone(),
                model: model.id.clone(),
                fallback_chain,
                reasoning: format!("Selected via {:?} strategy, primary: {} from {}", self.strategy, model.id, model.provider),
                estimated_cost: None,
            }
        } else {
            RoutingDecision {
                provider: "ollama".into(),
                model: "llama3.1".into(),
                fallback_chain: vec![],
                reasoning: "No healthy providers, falling back to local ollama".into(),
                estimated_cost: None,
            }
        }
    }

    pub fn should_fallback(&self, error: &str) -> bool {
        if !self.fallback_enabled { return false; }
        error.contains("rate_limited") || error.contains("unavailable") || error.contains("offline") || error.contains("429") || error.contains("503")
    }
}

impl Default for ModelRouter {
    fn default() -> Self {
        Self::new(RoutingStrategy::Adaptive, true)
    }
}
