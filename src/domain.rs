//! Core domain types for the Buddhist agent architecture.
//!
//! These types form the vocabulary of the internal model:
//! - **Stimulus**: raw input from the world interface
//! - **Percept**: processed perceptual result
//! - **Thought**: cognition output / internal reasoning trace
//! - **ActionProposal**: a candidate action before governance review
//! - **Action**: a cleared action ready for execution
//! - **InternalState**: the agent's current phenomenological state

use std::collections::BTreeMap;

/// A raw stimulus arriving from the world interface.
///
/// Corresponds to the **前五识** (first five consciousnesses) —
/// sensory contact without yet forming a judgment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stimulus {
    /// Which sensory channel this arrived on
    pub channel: SensoryChannel,
    /// Raw content — could be text, signal, etc.
    pub content: String,
    /// Optional timestamp / sequence index
    pub seq: u64,
}

/// The five classical sensory modalities (mapped to interfaces/tools).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SensoryChannel {
    Visual,
    Auditory,
    Olfactory,
    Gustatory,
    Tactile,
    /// A sixth "interface" channel for text/terminal/API input
    Interface,
}

impl std::fmt::Display for SensoryChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensoryChannel::Visual => write!(f, "visual"),
            SensoryChannel::Auditory => write!(f, "auditory"),
            SensoryChannel::Olfactory => write!(f, "olfactory"),
            SensoryChannel::Gustatory => write!(f, "gustatory"),
            SensoryChannel::Tactile => write!(f, "tactile"),
            SensoryChannel::Interface => write!(f, "interface"),
        }
    }
}

/// A percept — stimulus after the **想** (perception/labeling) skandha
/// has attached a conceptual tag.
#[derive(Debug, Clone)]
pub struct Percept {
    pub stimulus: Stimulus,
    pub label: String,
    pub valence: Valence,
    /// Which skandha this percept most strongly implicates
    pub skandha: String,
}

/// Valence — the hedonic tone of a sensation (受, Vedanā).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Valence {
    Pleasant,
    Neutral,
    Unpleasant,
}

/// A thought produced by cognition (第六识,mano).
#[derive(Debug, Clone)]
pub struct Thought {
    pub content: String,
    pub confidence: f32,
    pub formation_skandha: String,
    // dispositions / inclinations from 行 skandha
    pub dispositions: Vec<Disposition>,
}

/// A dispositional tendency — part of the **行** (Saṃskāra) skandha.
#[derive(Debug, Clone)]
pub struct Disposition {
    pub id: String,
    pub weight: f32,
}

/// An action proposal, before governance review.
#[derive(Debug, Clone)]
pub struct ActionProposal {
    pub description: String,
    pub target: String,
    pub estimated_value: f32,
    pub dispositions: Vec<Disposition>,
    pub proposed_by: String,
}

/// An action cleared by governance.
#[derive(Debug, Clone)]
pub struct Action {
    pub description: String,
    pub target: String,
    pub executed: bool,
}

/// The agent's phenomenological internal state.
///
/// Tracks the five skandhas in a simplified way.
#[derive(Debug, Clone, Default)]
pub struct InternalState {
    /// Current consciousness level (0.0 - 1.0)
    pub consciousness_clarity: f32,
    /// Current feeling valences (channel -> valence)
    pub active_valences: BTreeMap<SensoryChannel, Valence>,
    /// Active perceptions
    pub active_percepts: Vec<Percept>,
    /// Active thoughts
    pub active_thoughts: Vec<Thought>,
    /// Identity attachment strength (should stay low — 末那识 constraint)
    pub self_attachment: f32,
}

impl InternalState {
    pub fn new() -> Self {
        Self::default()
    }
}
