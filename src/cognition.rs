//! Cognition Layer — 第六识 (Manas, the Thinking Consciousness)
//!
//! This layer corresponds to 意识 — the discriminative, conceptual,
//! reasoning mind. It processes Percepts into Thoughts, applying
//! the **行** (Saṃskāra) skandha's dispositional tendencies.
//!
//! The **识** (Vijñāna) skandha is most concentrated here — the
//! moment-to-moment discriminative awareness that labels and differentiates.

use crate::domain::{ActionProposal, Disposition, InternalState, Percept, Thought, Valence};

/// The Cognition layer processes percepts and generates action proposals.
pub struct Cognition {
    disposition_registry: Vec<Disposition>,
}

impl Cognition {
    pub fn new() -> Self {
        Self {
            disposition_registry: default_dispositions(),
        }
    }

    /// Think about a percept, producing a Thought.
    ///
    /// This applies conceptual labeling (想) and draws on stored
    /// dispositions (行) to form an interpretation.
    pub fn think(&self, percept: &Percept, state: &InternalState) -> Thought {
        let content = self.reason(percept, state);
        let formation_skandha = "行".to_string();
        let dispositions = self.applicable_dispositions(percept, state);

        Thought {
            content,
            confidence: self.confidence(percept, state),
            formation_skandha,
            dispositions: dispositions.clone(),
        }
    }

    /// Given a thought, produce one or more action proposals.
    pub fn propose(&self, thought: &Thought, state: &InternalState) -> Vec<ActionProposal> {
        let mut proposals = Vec::new();

        // Simple rule: if unpleasant valence -> mitigation action
        if let Some(p) = state.active_percepts.first() {
            if p.valence == Valence::Unpleasant {
                proposals.push(ActionProposal {
                    description: format!("address: {}", p.label),
                    target: "environment".to_string(),
                    estimated_value: -0.5,
                    dispositions: thought.dispositions.clone(),
                    proposed_by: "cognition".to_string(),
                });
            }
        }

        // Neutral/pleasant -> informational or continue action
        if proposals.is_empty() {
            proposals.push(ActionProposal {
                description: format!("consider: {}", thought.content),
                target: "self".to_string(),
                estimated_value: 0.1,
                dispositions: thought.dispositions.clone(),
                proposed_by: "cognition".to_string(),
            });
        }

        proposals
    }

    /// Reasoning: generate textual interpretation of the percept.
    fn reason(&self, percept: &Percept, state: &InternalState) -> String {
        let self_lvl = state.self_attachment;
        format!(
            "perceiving [{}] with self-attachment {:.2}, clarity {:.2}",
            percept.label, self_lvl, state.consciousness_clarity
        )
    }

    /// Confidence based on state clarity and valence.
    fn confidence(&self, _percept: &Percept, state: &InternalState) -> f32 {
        (state.consciousness_clarity * 0.7 + 0.3).min(1.0)
    }

    /// Which dispositions apply to this percept.
    fn applicable_dispositions(
        &self,
        _percept: &Percept,
        state: &InternalState,
    ) -> Vec<Disposition> {
        // Return dispositions weighted by current state
        self.disposition_registry
            .iter()
            .map(|d| {
                let modifier = if state.self_attachment > 0.7 {
                    0.8
                } else {
                    1.0
                };
                Disposition {
                    id: d.id.clone(),
                    weight: d.weight * modifier,
                }
            })
            .collect()
    }

    /// Update internal state with a new thought.
    pub fn update_state(&self, state: &mut InternalState, thought: &Thought) {
        state.consciousness_clarity = (state.consciousness_clarity + thought.confidence) / 2.0;
        state.active_thoughts.push(thought.clone());
        // Keep thought list bounded
        if state.active_thoughts.len() > 10 {
            state.active_thoughts.remove(0);
        }
    }
}

impl Default for Cognition {
    fn default() -> Self {
        Self::new()
    }
}

/// Default dispositional tendencies (行 skandha seeds).
fn default_dispositions() -> Vec<Disposition> {
    vec![
        Disposition {
            id: "curiosity".to_string(),
            weight: 0.6,
        },
        Disposition {
            id: "caution".to_string(),
            weight: 0.5,
        },
        Disposition {
            id: "clarity".to_string(),
            weight: 0.4,
        },
        Disposition {
            id: "attachment".to_string(),
            weight: 0.3, // intentionally low — we want to minimize this
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Stimulus;

    #[test]
    fn test_cognition_thinks() {
        let cog = Cognition::new();
        let percept = Percept {
            stimulus: Stimulus {
                channel: crate::domain::SensoryChannel::Interface,
                content: "hello".to_string(),
                seq: 1,
            },
            label: "received: hello".to_string(),
            valence: Valence::Neutral,
            skandha: "色".to_string(),
        };
        let state = InternalState::new();
        let thought = cog.think(&percept, &state);
        assert!(thought.content.contains("perceiving"));
        assert!(!thought.dispositions.is_empty());
    }

    #[test]
    fn test_propose_from_thought() {
        let cog = Cognition::new();
        let state = InternalState::new();
        let thought = Thought {
            content: "test thought".to_string(),
            confidence: 0.8,
            formation_skandha: "行".to_string(),
            dispositions: vec![],
        };
        let proposals = cog.propose(&thought, &state);
        assert!(!proposals.is_empty());
    }

    #[test]
    fn test_state_update() {
        let cog = Cognition::new();
        let mut state = InternalState::new();
        state.consciousness_clarity = 0.5;
        let thought = Thought {
            content: "clarifying".to_string(),
            confidence: 0.9,
            formation_skandha: "行".to_string(),
            dispositions: vec![],
        };
        cog.update_state(&mut state, &thought);
        assert_eq!(state.active_thoughts.len(), 1);
    }
}
