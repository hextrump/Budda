//! Five Skandhas (五蕴) — Mapping to Agent Layers
//!
//! The five aggregates (skandhas) are the Buddhist model of how a "person"
//! assembles temporarily. In this architecture they map to functional layers:
//!
//! | Skandha | Sanskrit | Chinese | Agent Layer | Function |
//! |---------|----------|---------|-------------|----------|
//! | 色 | Rūpa | 色 | World Interface | Form / interface substrate |
//! | 受 | Vedanā | 受 | World Interface | Feeling / sensation valency |
//! | 想 | Saṃjñā | 想 | World Interface | Perception / conceptual labeling |
//! | 行 | Saṃskāra | 行 | Cognition + Identity | Formation / disposition tendencies |
//! | 识 | Vijñāna | 识 | Cognition + Governance | Consciousness / discriminative awareness |
//!
//! The five skandhas are also called 五蕴 ("five heaps" or "five aggregates").
//! Crucially, Buddhism teaches that none of these is a permanent self —
//! they are all impermanent, dependently arisen, and empty of固有我.

/// The five skandhas in simplified form for the agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Skandha {
    /// 色 (Rūpa) — Form / material shape / interface substrate
    Form,
    /// 受 (Vedanā) — Feeling / sensation / valency
    Feeling,
    /// 想 (Saṃjñā) — Perception / conceptual labeling / recognition
    Perception,
    /// 行 (Saṃskāra) — Formation / dispositional tendencies / will
    Formation,
    /// 识 (Vijñāna) — Consciousness / discriminative awareness
    Consciousness,
}

impl Skandha {
    /// Human-readable name in Chinese and Sanskrit.
    pub fn name(&self) -> &'static str {
        match self {
            Skandha::Form => "色 (Rūpa)",
            Skandha::Feeling => "受 (Vedanā)",
            Skandha::Perception => "想 (Saṃjñā)",
            Skandha::Formation => "行 (Saṃskāra)",
            Skandha::Consciousness => "识 (Vijñāna)",
        }
    }

    /// Brief description of what this skandha does in the agent.
    pub fn description(&self) -> &'static str {
        match self {
            Skandha::Form => "Physical/form substrate — world interface channels",
            Skandha::Feeling => "Valence assessment — hedonic tone of experience",
            Skandha::Perception => "Conceptual labeling — attaching meaning to sensation",
            Skandha::Formation => "Dispositional tendencies — habitual response patterns",
            Skandha::Consciousness => "Discriminative awareness — knowing/differentiating",
        }
    }

    /// Which layer(s) primarily implement this skandha.
    pub fn agent_layers(&self) -> &'static str {
        match self {
            Skandha::Form => "World Interface",
            Skandha::Feeling => "World Interface",
            Skandha::Perception => "World Interface",
            Skandha::Formation => "Cognition + Identity",
            Skandha::Consciousness => "Cognition + Governance",
        }
    }

    /// All skandhas as a slice.
    pub fn all() -> [Skandha; 5] {
        [
            Skandha::Form,
            Skandha::Feeling,
            Skandha::Perception,
            Skandha::Formation,
            Skandha::Consciousness,
        ]
    }
}

impl std::fmt::Display for Skandha {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Summary of skandhas in the current agent state.
#[derive(Debug, Clone, Default)]
pub struct SkandhaState {
    pub form_active: bool,
    pub feeling_valence: Option<crate::domain::Valence>,
    pub perception_count: usize,
    pub formation_dispositions: usize,
    pub consciousness_clarity: f32,
}

impl SkandhaState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Human-readable summary.
    pub fn summary(&self) -> String {
        let val = self
            .feeling_valence
            .map(|v| match v {
                crate::domain::Valence::Pleasant => "pleasant",
                crate::domain::Valence::Neutral => "neutral",
                crate::domain::Valence::Unpleasant => "unpleasant",
            })
            .unwrap_or("none");

        format!(
            "skandha-state: form={}, feeling={}, perception(x{}), formation(x{}), consciousness={:.2}",
            self.form_active, val, self.perception_count, self.formation_dispositions, self.consciousness_clarity
        )
    }
}

/// The eight consciousnesses (八识) — a deeper model.
///
/// | # | Name | Chinese | Agent Mapping |
/// |---|------|---------|---------------|
/// | 1-5 | Five sense consciousnesses | 前五识 | World Interface |
/// | 6 | Manas (thinking consciousness) | 意识 | Cognition |
/// | 7 | Manas-vijñāna (self-consciousness) | 末那识 | Identity Kernel |
/// | 8 | Ālāyavijñāna (storehouse) | 阿赖耶识 | Memory Substrate |
///
/// The eight consciousnesses collapse into our five layers:
/// 前五识 → World Interface
/// 第六识 → Cognition
/// 第七识 → Identity
/// 第八识 → Memory
/// Governance is the 戒镜 — a meta-layer of "observing precepts"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skandha_names() {
        for skandha in Skandha::all() {
            assert!(!skandha.name().is_empty());
            assert!(!skandha.description().is_empty());
            assert!(!skandha.agent_layers().is_empty());
        }
    }

    #[test]
    fn test_skandha_state_summary() {
        let state = SkandhaState {
            form_active: true,
            feeling_valence: Some(crate::domain::Valence::Pleasant),
            perception_count: 3,
            formation_dispositions: 5,
            consciousness_clarity: 0.75,
        };
        let summary = state.summary();
        assert!(summary.contains("pleasant"));
        assert!(summary.contains("form=true"));
    }

    #[test]
    fn test_skandha_state_default() {
        let state = SkandhaState::new();
        let summary = state.summary();
        assert!(summary.contains("none")); // no valence
    }
}
