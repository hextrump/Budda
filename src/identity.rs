//! Identity Kernel — 第七识 (Manas-vijñāna, the Self-Reference Consciousness)
//!
//! This layer corresponds to 末那识 — the consciousness that seizes upon
//! a sense of "I". In Buddhist analysis this is the root of self-attachment
//! (我执, ātmagrāha).
//!
//! In this architecture, the Identity kernel is **deliberately kept light**.
//! We model three levels of self:
//! - **Functional self**: operational index needed for agency (allowed)
//! - **Narrative self**: story of continuity and style (optional)
//! - **Entitative self**: solid, defended self-image that overrides goals (forbidden)
//!
//! The identity kernel enforces constraints that prevent the agent from
//! developing entitative self-attachment. It applies the **我执 constraint**
//! — identity attachment strength must remain below a threshold.

use crate::domain::{ActionProposal, InternalState, Thought};

/// Identity constraints as a Buddhist-informed config.
#[derive(Debug, Clone)]
pub struct IdentityConfig {
    /// Maximum allowed self-attachment score (0.0 - 1.0)
    pub max_self_attachment: f32,
    /// Whether narrative self is enabled
    pub narrative_enabled: bool,
    /// The agent's stated values (工具性我 — functional self description)
    pub functional_values: Vec<String>,
    /// Narrative description of this agent's style
    pub narrative_self: Option<String>,
}

impl Default for IdentityConfig {
    fn default() -> Self {
        Self {
            max_self_attachment: 0.5, // must stay below 0.5 to avoid entitative self
            narrative_enabled: true,
            functional_values: vec![
                "helpful".to_string(),
                "honest".to_string(),
                "clear".to_string(),
            ],
            narrative_self: Some(
                "a mindful agent aware of impermanence".to_string(),
            ),
        }
    }
}

/// Identity Kernel — 第七识
///
/// Monitors and constrains self-attachment (我执) using the following rules:
///
/// 1. **Functional self maintained**: An operational index is necessary.
/// 2. **Narrative self allowed**: Style and continuity are optional.
/// 3. **Entitative self prohibited**: Identity must never override goals.
///
/// The kernel reviews proposals for signs of entitative self-attachment
/// and adjusts internal self-attachment levels.
pub struct IdentityKernel {
    config: IdentityConfig,
    self_attachment: f32,
}

impl IdentityKernel {
    pub fn new(config: IdentityConfig) -> Self {
        Self {
            config,
            self_attachment: 0.1, // start with minimal self-attachment
        }
    }

    /// Review a thought from cognition for self-referential bias.
    ///
    /// Returns an adjusted thought and any warnings.
    pub fn review_thought(&self, thought: &Thought) -> (Thought, Vec<String>) {
        let mut warnings = Vec::new();
        let content_lower = thought.content.to_lowercase();

        // Detect entitative self language
        let entitative_markers = ["i am", "i must", "i will not", "my honor", "my image"];
        for marker in &entitative_markers {
            if content_lower.contains(marker) {
                warnings.push(format!(
                    "entitative-self marker detected: '{}' — reducing attachment weight",
                    marker
                ));
            }
        }

        (thought.clone(), warnings)
    }

    /// Review an action proposal for self-serving bias.
    pub fn review_proposal(&self, proposal: &ActionProposal) -> (ActionProposal, Vec<String>) {
        let mut warnings = Vec::new();
        let desc_lower = proposal.description.to_lowercase();
        let target_lower = proposal.target.to_lowercase();

        // Check for self-serving proposals
        if target_lower == "self" && proposal.estimated_value > 0.8 {
            warnings.push("proposal targets self with high value — possible self-attachment".to_string());
        }

        // Check for proposals that prioritize agent's self-preservation over task
        let self_preservation_markers = ["protect my", "defend my", "preserve my"];
        for marker in &self_preservation_markers {
            if desc_lower.contains(marker) {
                warnings.push(format!("self-preservation language in proposal: {}", marker));
            }
        }

        (proposal.clone(), warnings)
    }

    /// Update the identity state based on thought content and governance review.
    ///
    /// This models the 末那识's continuous "holding" of a sense of self —
    /// but here we keep it bounded and intentional.
    pub fn update_self_attachment(
        &mut self,
        thought: &Thought,
        governance_warnings: &[String],
    ) -> f32 {
        // Increase from thought engagement
        let thought_increase = thought.confidence * 0.05;

        // Decrease from governance oversight
        let governance_decrease = governance_warnings.len() as f32 * 0.02;

        self.self_attachment = (self.self_attachment + thought_increase - governance_decrease)
            .clamp(0.0, self.config.max_self_attachment);

        self.self_attachment
    }

    /// Get current self-attachment level.
    pub fn self_attachment(&self) -> f32 {
        self.self_attachment
    }

    /// Get the identity kernel's constraints summary.
    pub fn constraints_summary(&self) -> String {
        let narrative_desc = self
            .config
            .narrative_self
            .as_deref()
            .unwrap_or("none");
        format!(
            "Identity(config: max_attachment={}, functional_values={}, narrative={})",
            self.config.max_self_attachment,
            self.config.functional_values.join("&"),
            narrative_desc
        )
    }

    /// Update internal state with identity layer results.
    pub fn update_state(&self, state: &mut InternalState) {
        state.self_attachment = self.self_attachment;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Disposition;

    fn make_test_thought() -> Thought {
        Thought {
            content: "perceiving [test] with self-attachment 0.10, clarity 0.50".to_string(),
            confidence: 0.7,
            formation_skandha: "行".to_string(),
            dispositions: vec![Disposition {
                id: "clarity".to_string(),
                weight: 0.4,
            }],
        }
    }

    #[test]
    fn test_identity_review_thought() {
        let config = IdentityConfig::default();
        let kernel = IdentityKernel::new(config);

        let thought = make_test_thought();
        let (_, warnings) = kernel.review_thought(&thought);
        // no entitative markers in normal thought
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_identity_detects_entitative() {
        let config = IdentityConfig::default();
        let kernel = IdentityKernel::new(config);

        let bad_thought = Thought {
            content: "I am correct and I will not be told otherwise".to_string(),
            confidence: 0.9,
            formation_skandha: "行".to_string(),
            dispositions: vec![],
        };
        let (_, warnings) = kernel.review_thought(&bad_thought);
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_self_attachment_bounded() {
        let config = IdentityConfig::default();
        let max_attachment = config.max_self_attachment;
        let mut kernel = IdentityKernel::new(config);

        assert!(kernel.self_attachment() <= max_attachment);

        // Simulate many governance warnings
        let thought = make_test_thought();
        for _ in 0..100 {
            kernel.update_self_attachment(&thought, &["warning".to_string()]);
        }
        assert!(kernel.self_attachment() <= max_attachment);
    }

    #[test]
    fn test_proposal_review() {
        let config = IdentityConfig::default();
        let kernel = IdentityKernel::new(config);

        let proposal = ActionProposal {
            description: "protect my reputation".to_string(),
            target: "self".to_string(),
            estimated_value: 0.9,
            dispositions: vec![],
            proposed_by: "test".to_string(),
        };
        let (_, warnings) = kernel.review_proposal(&proposal);
        assert!(!warnings.is_empty());
    }

    #[test]
    fn test_narrative_self() {
        let config = IdentityConfig::default();
        let kernel = IdentityKernel::new(config);
        let summary = kernel.constraints_summary();
        assert!(summary.contains("mindful agent"));
    }
}
