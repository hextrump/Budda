//! Buddhist Agent Architecture — Rust Prototype
//!
//! A five-layer agent architecture inspired by Buddhist models of mind:
//! - **World Interface** (前五识): Perceptual/input channels
//! - **Cognition** (意识): Active inference and reasoning
//! - **Identity** (末那识): Self-binding / identity kernel
//! - **Memory** (阿赖耶识): Persistent substrate / habit store
//! - **Governance** (戒镜): Safety checks and self-oversight
//!
//! Also maps to the **five skandhas** (five aggregates):
//! - 色 (Rūpa) = form / interface substrate
//! - 受 (Vedanā) = feeling / sensation valency
//! - 想 (Saṃjñā) = perception / conceptual labeling
//! - 行 (Saṃskāra) = formation / dispositional tendencies
//! - 识 (Vijñāna) = consciousness / discriminative awareness
//!
//! # Architecture
//!
//! ```text
//! Input Stimulus
//!      |
//!      v
//! [World Interface]  -- 五识 (perception)
//!      |
//!      v
//! [Cognition]         -- 第六识 (reasoning)
//!      |
//!      v
//! [Identity]          -- 第七识 (self-binding)
//!      |
//!      v
//! [Governance]        -- 戒镜 (oversight)
//!      |
//!      v
//! [Memory]            -- 第八识 (deposit/habit)
//!      |
//!      v
//! Action Proposal
//! ```
//!
//! Each turn: perception → cognition → identity attachment → governance check → memory update

pub mod cognition;
pub mod domain;
pub mod engine;
pub mod governance;
pub mod identity;
pub mod llm;
pub mod memory;
pub mod skandhas;
pub mod world;

pub use domain::*;
pub use engine::Engine;
