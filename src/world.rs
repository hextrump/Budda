//! World Interface Layer — 前五识 (First Five Consciousnesses)
//!
//! This layer corresponds to the sensory-input side of the Buddhist model:
//! the five sense-doors (眼耳鼻舌身) plus an additional "interface" door
//! for textual/terminal input.
//!
//! The **色** (Rūpa) skandha — form, materiality — lives here as the
//! substrate through which stimuli enter.

use crate::domain::{Percept, SensoryChannel, Stimulus, Valence};

/// The World Interface receives raw stimuli and converts them to Percepts.
///
/// In a full system this would wrap actual I/O (browser, terminal, etc.).
/// Here it is a simple in-memory interface.
pub struct WorldInterface {
    name: String,
}

impl WorldInterface {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Receive a raw stimulus and produce a Percept.
    ///
    /// This applies the **想** (perception/labeling) skandha —
    /// attaching a conceptual tag to the raw sensory input.
    pub fn receive(&self, stimulus: Stimulus) -> Percept {
        let label = self.label(&stimulus);
        let valence = self.assess_valence(&stimulus);
        let skandha = "色".to_string(); // primary skandha at interface is form
        Percept {
            stimulus,
            label,
            valence,
            skandha,
        }
    }

    /// Label a stimulus — part of the 想 skandha.
    fn label(&self, stimulus: &Stimulus) -> String {
        match stimulus.channel {
            SensoryChannel::Visual => format!("saw: {}", stimulus.content),
            SensoryChannel::Auditory => format!("heard: {}", stimulus.content),
            SensoryChannel::Olfactory => format!("smelled: {}", stimulus.content),
            SensoryChannel::Gustatory => format!("tasted: {}", stimulus.content),
            SensoryChannel::Tactile => format!("touched: {}", stimulus.content),
            SensoryChannel::Interface => format!("received: {}", stimulus.content),
        }
    }

    /// Assess valence (受 skandha) from stimulus characteristics.
    fn assess_valence(&self, stimulus: &Stimulus) -> Valence {
        let content_lower = stimulus.content.to_lowercase();
        if content_lower.contains("error")
            || content_lower.contains("fail")
            || content_lower.contains("danger")
            || content_lower.contains("problem")
        {
            Valence::Unpleasant
        } else if content_lower.contains("success")
            || content_lower.contains("good")
            || content_lower.contains("help")
        {
            Valence::Pleasant
        } else {
            Valence::Neutral
        }
    }

    /// Simulate executing an action in the world.
    /// Returns a new stimulus representing the world's response.
    pub fn act(&self, description: &str, target: &str) -> Option<Stimulus> {
        // Simple world model: echo back a response
        Some(Stimulus {
            channel: SensoryChannel::Interface,
            content: format!("[{}] acted: {} -> {}", self.name, description, target),
            seq: 0,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_labels() {
        let wi = WorldInterface::new("test");
        let s = Stimulus {
            channel: SensoryChannel::Interface,
            content: "hello world".to_string(),
            seq: 1,
        };
        let p = wi.receive(s);
        assert_eq!(p.label, "received: hello world");
        assert_eq!(p.valence, Valence::Neutral);
    }

    #[test]
    fn test_valence_detection() {
        let wi = WorldInterface::new("test");
        let bad = Stimulus {
            channel: SensoryChannel::Interface,
            content: "error: something failed".to_string(),
            seq: 1,
        };
        assert_eq!(wi.receive(bad).valence, Valence::Unpleasant);

        let good = Stimulus {
            channel: SensoryChannel::Interface,
            content: "success: task completed".to_string(),
            seq: 2,
        };
        assert_eq!(wi.receive(good).valence, Valence::Pleasant);
    }

    #[test]
    fn test_world_act() {
        let wi = WorldInterface::new("act-test");
        let result = wi.act("read", "file.txt");
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.content.contains("read"));
        assert_eq!(r.channel, SensoryChannel::Interface);
    }
}
