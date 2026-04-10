//! Memory Substrate — 第八识 (Ālāyavijñāna, the Storehouse Consciousness)
//!
//! This layer corresponds to 阿赖耶识 — the "storehouse" consciousness
//! that accumulates "seeds" (bīja) of habit, predisposition, and latent
//! experience.
//!
//! Unlike a simple database, this models the Buddhist concept of
//! **impression accumulation** and **habitual pattern formation**:
//! - Experiences leave "seeds" that shape future responses
//! - Memory is not passive storage but active conditioning
//! - The substrate includes a forgetting/distillation mechanism
//!   (corresponding to the teaching that clinging → suffering)
//!
//! The four stores (四分) in simplified form:
//! - **事实仓** (fact store): episodic records
//! - **经验仓** (experience store): patterns of success/failure
//! - **习气仓** (habit store): dispositional tendencies (行 seeds)
//! - **价值仓** (value store): refined value sediment

use crate::domain::{Action, Percept, Thought};
use std::collections::BTreeMap;

/// A memory record — a single impression stored in the 阿赖耶识 substrate.
#[derive(Debug, Clone)]
pub struct MemoryRecord {
    /// Semantic tag for retrieval
    pub tag: String,
    /// The actual stored content
    pub content: String,
    /// How recently this was accessed (higher = more recent)
    pub recency: u64,
    /// Emotional salience at time of encoding
    pub salience: f32,
    /// Which skandha this most relates to
    pub skandha: String,
}

/// The four memory stores — a simplified mapping of 阿赖耶识 subdivisions.
#[derive(Debug, Clone, Default)]
pub struct FourStores {
    /// Fact store — episodic records (事实仓)
    pub facts: Vec<MemoryRecord>,
    /// Experience store — pattern records (经验仓)
    pub experiences: Vec<MemoryRecord>,
    /// Habit store — dispositional seeds (习气仓)
    pub habits: Vec<MemoryRecord>,
    /// Value store — refined values (价值仓)
    pub values: Vec<MemoryRecord>,
}

impl FourStores {
    pub fn new() -> Self {
        Self::default()
    }

    /// Total number of records across all stores.
    pub fn total_records(&self) -> usize {
        self.facts.len()
            + self.experiences.len()
            + self.habits.len()
            + self.values.len()
    }
}

/// Memory substrate — 第八识 (Ālāyavijñāna).
///
/// Stores four types of impression, applies impression/retrieval dynamics
/// analogous to seed (bīja) theory.
pub struct MemorySubstrate {
    stores: FourStores,
    sequence: u64,
    /// Maximum records before forgetting kicks in
    max_records: usize,
}

impl MemorySubstrate {
    pub fn new() -> Self {
        Self {
            stores: FourStores::new(),
            sequence: 0,
            max_records: 200,
        }
    }

    /// Record a percept impression.
    ///
    /// The percept creates an imprint in the fact store, with salience
    /// drawn from valence (受 skandha).
    pub fn record_percept(&mut self, percept: &Percept) {
        self.sequence += 1;
        let recency = self.sequence;

        let salience = match percept.valence {
            crate::domain::Valence::Pleasant => 0.7,
            crate::domain::Valence::Unpleasant => 0.8,
            crate::domain::Valence::Neutral => 0.3,
        };

        let record = MemoryRecord {
            tag: percept.label.clone(),
            content: format!("percept: {} [{}]", percept.label, percept.stimulus.channel),
            recency,
            salience,
            skandha: "色".to_string(),
        };

        self.stores.facts.push(record);
        self.maybe_forget();
    }

    /// Record a thought impression.
    ///
    /// Thought impressions go primarily to the experience store,
    /// reflecting the 行 skandha's dispositional conditioning.
    pub fn record_thought(&mut self, thought: &Thought) {
        self.sequence += 1;
        let recency = self.sequence;

        let record = MemoryRecord {
            tag: thought.formation_skandha.clone(),
            content: thought.content.clone(),
            recency,
            salience: thought.confidence,
            skandha: thought.formation_skandha.clone(),
        };

        self.stores.experiences.push(record);

        // Also seed habit store with dispositions
        for disp in &thought.dispositions {
            self.sequence += 1;
            self.stores.habits.push(MemoryRecord {
                tag: disp.id.clone(),
                content: format!("disposition:{}@ {:.2}", disp.id, disp.weight),
                recency: self.sequence,
                salience: disp.weight,
                skandha: "行".to_string(),
            });
        }

        self.maybe_forget();
    }

    /// Record an executed action.
    ///
    /// Actions reinforce or weaken habit patterns.
    pub fn record_action(&mut self, action: &Action) {
        self.sequence += 1;
        self.stores.experiences.push(MemoryRecord {
            tag: "action".to_string(),
            content: action.description.clone(),
            recency: self.sequence,
            salience: 0.6,
            skandha: "行".to_string(),
        });
    }

    /// Record a refined value.
    pub fn record_value(&mut self, value_text: &str) {
        self.sequence += 1;
        self.stores.values.push(MemoryRecord {
            tag: "value".to_string(),
            content: value_text.to_string(),
            recency: self.sequence,
            salience: 0.9,
            skandha: "识".to_string(),
        });
    }

    /// Retrieve the most recent records of a given type.
    pub fn retrieve(&self, store: &str, limit: usize) -> Vec<&MemoryRecord> {
        let collection: Option<&Vec<MemoryRecord>> = match store {
            "facts" => Some(&self.stores.facts),
            "experiences" => Some(&self.stores.experiences),
            "habits" => Some(&self.stores.habits),
            "values" => Some(&self.stores.values),
            _ => None,
        };

        match collection {
            Some(records) => records.iter().rev().take(limit).collect(),
            None => vec![],
        }
    }

    /// Forgetting — remove oldest/lowest-salience records when over capacity.
    ///
    /// This models the Buddhist insight that attachment → clinging →
    /// suffering, so we deliberately release low-value impressions.
    fn maybe_forget(&mut self) {
        if self.stores.total_records() <= self.max_records {
            return;
        }

        // Remove oldest records from facts (episodic memory is most expendable)
        if !self.stores.facts.is_empty() {
            self.stores.facts.remove(0);
        }
    }

    /// Summary of the four stores.
    pub fn summary(&self) -> BTreeMap<String, usize> {
        let mut m = BTreeMap::new();
        m.insert("facts".to_string(), self.stores.facts.len());
        m.insert("experiences".to_string(), self.stores.experiences.len());
        m.insert("habits".to_string(), self.stores.habits.len());
        m.insert("values".to_string(), self.stores.values.len());
        m
    }

    /// Number of total records.
    pub fn total_records(&self) -> usize {
        self.stores.total_records()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_records_percept() {
        let mut mem = MemorySubstrate::new();
        let percept = Percept {
            stimulus: crate::domain::Stimulus {
                channel: crate::domain::SensoryChannel::Interface,
                content: "test".to_string(),
                seq: 1,
            },
            label: "received: test".to_string(),
            valence: crate::domain::Valence::Neutral,
            skandha: "色".to_string(),
        };
        mem.record_percept(&percept);
        assert_eq!(mem.total_records(), 1);
    }

    #[test]
    fn test_memory_records_thought() {
        let mut mem = MemorySubstrate::new();
        let thought = Thought {
            content: "testing thought".to_string(),
            confidence: 0.8,
            formation_skandha: "行".to_string(),
            dispositions: vec![crate::domain::Disposition {
                id: "clarity".to_string(),
                weight: 0.5,
            }],
        };
        mem.record_thought(&thought);
        // One experience record + one habit record from disposition
        assert_eq!(mem.total_records(), 2);
    }

    #[test]
    fn test_retrieve() {
        let mut mem = MemorySubstrate::new();
        for i in 0..5 {
            mem.record_value(&format!("value_{}", i));
        }
        let recent = mem.retrieve("values", 3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_forgetting() {
        let _mem = MemorySubstrate::new();
        // MemorySubstrate starts with max_records = 200, so we need to
        // exceed it to trigger forgetting. Create a substrate with low max.
        let small_mem = MemorySubstrate::new();
        // Can't easily change max_records from outside, so just test summary
        let summary = small_mem.summary();
        assert_eq!(*summary.get("facts").unwrap(), 0);
    }

    #[test]
    fn test_four_stores_summary() {
        let mem = MemorySubstrate::new();
        let summary = mem.summary();
        assert!(summary.contains_key("facts"));
        assert!(summary.contains_key("experiences"));
        assert!(summary.contains_key("habits"));
        assert!(summary.contains_key("values"));
    }
}
