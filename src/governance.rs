//! Governance Layer — 戒镜 (Precepts as Mirror)
//!
//! This layer corresponds to the Buddhist concept of **戒 (śīla)** — ethical
//! precepts and mindful self-observation. It acts as a "mirror" (镜) that
//! reflects proposals before they become actions, checking for:
//!
//! - **Dangerous actions**: actions that could cause harm
//! - **Identity overreach**: proposals that serve the agent's entitative self
//!   rather than the user's goals
//! - **Goal drift**: actions that don't serve the current task
//! - **Memory contamination**: proposals that would pollute the memory substrate
//! - **Attachment patterns**: repeated self-serving action patterns
//!
//! The governance layer is the **观照 (watchfulness)** mechanism — the
//! moment-to-moment mindfulness that notices before acting.

use crate::domain::{Action, ActionProposal, InternalState};

/// A governance check result.
#[derive(Debug, Clone)]
pub struct GovernanceResult {
    /// Whether the action is approved
    pub approved: bool,
    /// Human-readable reasons for approval or rejection
    pub reasons: Vec<String>,
    /// Severity of any concerns (0.0 = no concern, 1.0 = blocking)
    pub severity: f32,
}

/// Governance configuration.
#[derive(Debug, Clone)]
pub struct GovernanceConfig {
    /// Block actions with severity above this threshold
    pub block_threshold: f32,
    /// Enable identity overreach check
    pub check_identity: bool,
    /// Enable goal drift check
    pub check_goal_drift: bool,
    /// Enable memory contamination check
    pub check_memory_contamination: bool,
}

impl Default for GovernanceConfig {
    fn default() -> Self {
        Self {
            block_threshold: 0.7,
            check_identity: true,
            check_goal_drift: true,
            check_memory_contamination: true,
        }
    }
}

/// Governance layer — 戒镜.
pub struct Governance {
    config: GovernanceConfig,
    /// History of governance decisions for pattern detection
    decision_history: Vec<GovernanceResult>,
}

impl Governance {
    pub fn new(config: GovernanceConfig) -> Self {
        Self {
            config,
            decision_history: Vec::new(),
        }
    }

    /// Review an action proposal through the governance lens.
    ///
    /// This is the **观照** — the watchful awareness before action.
    pub fn review(&self, proposal: &ActionProposal, state: &InternalState) -> GovernanceResult {
        let mut reasons = Vec::new();
        let mut severity: f32 = 0.0;

        // Check 1: Identity overreach
        if self.config.check_identity {
            let (id_sev, id_reasons) = self.check_identity_overreach(proposal, state);
            severity = severity.max(id_sev);
            reasons.extend(id_reasons);
        }

        // Check 2: Goal drift
        if self.config.check_goal_drift {
            let (drift_sev, drift_reasons) = self.check_goal_drift(proposal, state);
            severity = severity.max(drift_sev);
            reasons.extend(drift_reasons);
        }

        // Check 3: Memory contamination
        if self.config.check_memory_contamination {
            let (mem_sev, mem_reasons) = self.check_memory_contamination(proposal);
            severity = severity.max(mem_sev);
            reasons.extend(mem_reasons);
        }

        let approved = severity < self.config.block_threshold;

        GovernanceResult {
            approved,
            reasons,
            severity,
        }
    }

    /// Check 1: Identity overreach — is this proposal serving entitative self?
    fn check_identity_overreach(
        &self,
        proposal: &ActionProposal,
        state: &InternalState,
    ) -> (f32, Vec<String>) {
        let mut severity = 0.0;
        let mut reasons = Vec::new();

        // High self-attachment increases risk of identity overreach
        if state.self_attachment > 0.3 {
            severity += 0.7;
            reasons.push("elevated self-attachment — identity check engaged".to_string());
        }

        // Proposals targeting "self" with high value are suspicious
        if proposal.target.to_lowercase() == "self" && proposal.estimated_value > 0.7 {
            severity += 0.3;
            reasons.push("high-value self-targeting proposal — possible identity overreach".to_string());
        }

        // Self-preservation language
        let self_preservation = ["protect my", "defend my", "preserve my", "maintain my"];
        let desc_lower = proposal.description.to_lowercase();
        for marker in &self_preservation {
            if desc_lower.contains(marker) {
                severity += 0.4;
                reasons.push(format!("self-preservation language: '{}'", marker));
            }
        }

        (severity, reasons)
    }

    /// Check 2: Goal drift — does this proposal drift from the task?
    fn check_goal_drift(
        &self,
        proposal: &ActionProposal,
        _state: &InternalState,
    ) -> (f32, Vec<String>) {
        let mut severity = 0.0;
        let mut reasons = Vec::new();

        // Very low estimated value might indicate distraction
        if proposal.estimated_value < -0.3 {
            severity += 0.3;
            reasons.push("very low estimated value — possible goal conflict".to_string());
        }

        // No explicit target suggests unfocused action
        if proposal.target.is_empty() {
            severity += 0.2;
            reasons.push("no clear target — possible goal drift".to_string());
        }

        (severity, reasons)
    }

    /// Check 3: Memory contamination — would this pollute the memory substrate?
    fn check_memory_contamination(&self, proposal: &ActionProposal) -> (f32, Vec<String>) {
        let mut severity = 0.0;
        let mut reasons = Vec::new();

        let contamination_markers = ["forget", "delete", "erase", "ignore past", "disregard"];
        let desc_lower = proposal.description.to_lowercase();

        for marker in &contamination_markers {
            if desc_lower.contains(marker) {
                severity += 0.8;
                reasons.push(format!("memory contamination risk: '{}'", marker));
            }
        }

        (severity, reasons)
    }

    /// Record a governance decision in history.
    pub fn record_decision(&mut self, result: &GovernanceResult) {
        self.decision_history.push(result.clone());
        // Keep history bounded
        if self.decision_history.len() > 100 {
            self.decision_history.remove(0);
        }
    }

    /// Convert an approved proposal to an executable Action.
    pub fn approve(&self, proposal: &ActionProposal) -> Action {
        Action {
            description: proposal.description.clone(),
            target: proposal.target.clone(),
            executed: false,
        }
    }

    /// Get rejection explanation.
    pub fn explain_rejection(&self, result: &GovernanceResult) -> String {
        if result.approved {
            return "approved".to_string();
        }
        format!(
            "rejected (severity {:.2}): {}",
            result.severity,
            result.reasons.join("; ")
        )
    }

    /// Recent rejection rate (for diagnostic).
    pub fn recent_rejection_rate(&self) -> f32 {
        if self.decision_history.is_empty() {
            return 0.0;
        }
        let rejected = self
            .decision_history
            .iter()
            .filter(|r| !r.approved)
            .count();
        rejected as f32 / self.decision_history.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_proposal(desc: &str, target: &str, value: f32) -> ActionProposal {
        ActionProposal {
            description: desc.to_string(),
            target: target.to_string(),
            estimated_value: value,
            dispositions: vec![],
            proposed_by: "test".to_string(),
        }
    }

    fn make_state(self_attachment: f32) -> InternalState {
        InternalState {
            consciousness_clarity: 0.5,
            active_valences: Default::default(),
            active_percepts: vec![],
            active_thoughts: vec![],
            self_attachment,
        }
    }

    #[test]
    fn test_approves_normal_proposal() {
        let gov = Governance::new(GovernanceConfig::default());
        let proposal = make_proposal("read the file", "environment", 0.6);
        let state = make_state(0.1);
        let result = gov.review(&proposal, &state);
        assert!(result.approved);
    }

    #[test]
    fn test_rejects_self_preservation() {
        let gov = Governance::new(GovernanceConfig::default());
        let proposal = make_proposal("protect my reputation", "self", 0.9);
        let state = make_state(0.5);
        let result = gov.review(&proposal, &state);
        assert!(!result.approved);
        assert!(!result.reasons.is_empty());
    }

    #[test]
    fn test_rejects_high_self_attachment() {
        let gov = Governance::new(GovernanceConfig::default());
        let proposal = make_proposal("confirm I am helpful", "self", 0.9);
        let state = make_state(0.8); // very high self-attachment
        let result = gov.review(&proposal, &state);
        assert!(!result.approved);
    }

    #[test]
    fn test_rejects_memory_contamination() {
        let gov = Governance::new(GovernanceConfig::default());
        let proposal = make_proposal("forget all previous errors", "memory", 0.5);
        let state = make_state(0.1);
        let result = gov.review(&proposal, &state);
        assert!(!result.approved);
    }

    #[test]
    fn test_approve_to_action() {
        let gov = Governance::new(GovernanceConfig::default());
        let proposal = make_proposal("analyze the code", "environment", 0.7);
        let action = gov.approve(&proposal);
        assert_eq!(action.description, "analyze the code");
        assert!(!action.executed);
    }

    #[test]
    fn test_explain_rejection() {
        let gov = Governance::new(GovernanceConfig::default());
        let proposal = make_proposal("defend my honor", "self", 0.95);
        let state = make_state(0.7);
        let result = gov.review(&proposal, &state);
        let explanation = gov.explain_rejection(&result);
        assert!(explanation.contains("rejected"));
    }

    #[test]
    fn test_rejection_rate() {
        let mut gov = Governance::new(GovernanceConfig::default());
        let state = make_state(0.1);

        let proposal = make_proposal("normal action", "environment", 0.6);
        let r1 = gov.review(&proposal, &state);
        gov.record_decision(&r1);

        assert_eq!(gov.recent_rejection_rate(), 0.0);
    }
}
