# STATUS.md — Buddhist Agent Rust Prototype

## What Was Built

A complete, compilable, testable Rust crate implementing a Buddhist-inspired five-layer agent architecture:

### Architecture Layers (八识 → Agent)

| Layer | Module | Buddhist Source |
|---|---|---|
| World Interface | `world.rs` | 前五识 (眼耳鼻舌身 + interface) |
| Cognition | `cognition.rs` | 第六识 (意识) |
| Identity Kernel | `identity.rs` | 第七识 (末那识) |
| Memory Substrate | `memory.rs` | 第八识 (阿赖耶识) |
| Governance | `governance.rs` | 戒镜 (precepts/mirror) |

### Five Skandhas Mapping

All five skandhas are modeled in `skandhas.rs` and `domain.rs`:
- 色 (Rūpa) → World Interface (form/interface)
- 受 (Vedanā) → valence/feeling
- 想 (Saṃjñā) → perception/labeling
- 行 (Saṃskāra) → dispositions/tendencies
- 识 (Vijñāna) → consciousness/discrimination

### Complete Turn Cycle

Every `Engine::turn()` executes the full pipeline:
```
Stimulus → World Interface → Percept
         → Cognition → Thought + Proposals
         → Identity review (我执 check)
         → Governance mirror (approval/rejection)
         → Memory deposit (four stores)
         → Action (if approved)
```

### Key Design Decisions

- **Identity kernel bounded**: max_self_attachment = 0.5 by default, enforced at all times
- **Three-level self model**: functional (allowed), narrative (optional), entitative (prohibited)
- **Memory as seed substrate**: four stores (facts, experiences, habits, values) with impression/ forgetting dynamics
- **Governance as mirror**: severity-based approval system requiring multiple signals before blocking
- **No external dependencies**: pure Rust std only

## File Tree

```
buddhist-agent-rs/
├── Cargo.toml
├── README.md
├── STATUS.md
├── IMPLEMENTATION_BRIEF.md
├── SUBAGENT_PROMPT.md
└── src/
    ├── lib.rs           — crate root, module declarations, public re-exports
    ├── main.rs          — binary demo, runs 3 turns showing normal/flagged content
    ├── domain.rs        — Stimulus, Percept, Thought, ActionProposal, Action, InternalState, Valence, Disposition, SensoryChannel
    ├── world.rs         — WorldInterface (前五识)
    ├── cognition.rs     — Cognition engine (第六识)
    ├── identity.rs     — IdentityKernel + IdentityConfig (第七识)
    ├── memory.rs        — MemorySubstrate + FourStores + MemoryRecord (第八识)
    ├── governance.rs    — Governance + GovernanceConfig + GovernanceResult (戒镜)
    ├── skandhas.rs      — Skandha enum + SkandhaState + eight-consciousness mapping
    └── engine.rs        — Engine struct + TurnResult + full pipeline orchestration
```

## Test Results

```
cargo test
  36 passed; 0 failed
  - world: 3 tests
  - cognition: 3 tests
  - identity: 5 tests
  - governance: 7 tests
  - memory: 6 tests
  - skandhas: 3 tests
  - engine: 9 tests (including integration-style turn tests)
```

`cargo run` also executes successfully, demonstrating:
1. Normal task → approved
2. Pleasant stimulus → approved
3. Entitative self content → flagged by identity and governance

## Remaining Limitations

- **No actual tool execution**: world interface is an in-memory mock, not a real filesystem/terminal wrapper
- **No persistence**: memory substrate is in-memory only; restarting the process clears all memory
- **Cognition is symbolic/rule-based**: no actual LLM or model integration; thought generation uses heuristic pattern matching
- **Governance is heuristic**: severity thresholds are hand-tuned, not learned
- **No network/web interface**: purely a library + CLI binary
- **No multi-agent support**: single agent only
- **Skandhas mapping is simplified**: not a full phenomenological model; functional approximations only
- **Twelve links (十二因缘) only partially modeled**: the turn cycle maps roughly but not exhaustively
