# Buddhist Agent Architecture — Rust Prototype

A five-layer agent architecture inspired by Buddhist models of mind, implemented as a compilable Rust crate.

## Architecture Overview

The architecture maps classical Buddhist psychological models onto an agent system:

### Five Layers (八识 → Agent)

| Buddhist Source | Layer | Description |
|---|---|---|
| 前五识 (1-5) 眼识·耳识·鼻识·舌识·身识<br>*Pāli: cakkhuviññāṇa·sotaviññāṇa·ghānaviññāṇa·jivhāviññāṇa·kāyaviññāṇa* | **World Interface** | Sensory input channels — visual, auditory, olfactory, gustatory, tactile |
| 第六识 (6) 意识<br>*Pāli: manoviññāṇa* | **Cognition** | Active inference, reasoning, conceptual labeling |
| 第七识 (7) 末那识<br>*Skt: manas* | **Identity Kernel** | Self-binding, clinging to the I-maker (我执根器) |
| 第八识 (8) 阿赖耶识<br>*Skt: ālayavijñāna* | **Memory Substrate** | Storehouse consciousness — seeds (种子), impressions, four memory stores |
| 戒镜 (戒律之镜) | **Governance** | Precepts/mirror — safety, self-oversight, ethical refraction |

### Five Skandhas (五蕴 → Agent Functions)

| Skandha | Sanskrit | Agent Mapping | Function |
|---|---|---|---|
| 色 | *rūpa* | World Interface | Form / interface substrate |
| 受 | *vedanā* | World Interface | Feeling / sensation valence |
| 想 | *saṃjñā* | World Interface | Perception / conceptual labeling |
| 行 | *saṃskāra* | Cognition + Identity | Formation / dispositional tendencies |
| 识 | *vijñāna* | Cognition + Governance | Consciousness / discriminative awareness |

### Pipeline: One Turn

```
Stimulus → [World Interface] → Percept
         → [Cognition] → Thought + Action Proposals
         → [Identity] → Self-attachment review
         → [Governance] → Approved/Rejected
         → [Memory] → Seed deposit
         → Action (if approved)
```

The pipeline models a simplified **十二因缘** (dependent origination) cycle:
触 → 受 → 爱 → 取 → 有 → (生 → 死, implicitly)

### Identity Philosophy

Three levels of self modeled (from QA archive discussions):

1. **Functional self** (操作我): operational index for agency — **allowed**
2. **Narrative self** (叙事我): style and continuity — **optional**
3. **Entitative self** (实体化我): defended self-image overriding goals — **prohibited**

The identity kernel enforces `max_self_attachment ≤ 0.5` to prevent entitative self (我执).

### Memory Stores (四分)

The memory substrate maintains four stores (simplified from 阿赖耶识):

- **事实仓** (fact store): episodic records
- **经验仓** (experience store): success/failure patterns
- **习气仓** (habit store): dispositional seeds (bīja)
- **价值仓** (value store): refined value deposits

## Source Documents

Architecture inspiration from discussions on Buddhist models of mind:
- [佛学中可映射到 Agent 架构的现成人之结构模型](https://github.com/hextrump/buddhist-agent-discussions/blob/main/notes/2026-04-09-buddhist-agent-architecture-existing-models.md) — 五蕴、八识、十二因缘、唯识的映射方案
- [Agent 架构设计问答记录](https://github.com/hextrump/buddhist-agent-discussions/blob/main/notes/2026-04-09-buddhist-agent-architecture-qa-archive.md) — 功能自我/叙事自我/实体化我的三层划分

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Running

Two modes available:

```bash
# Heuristic mode (default, no external LLM)
cargo run

# LLM mode (MiniMax M2.7)
MINIMAX_CN_API_KEY=<your-key> cargo run -- --llm
```

The demo runs four turns:
1. Normal task — approved
2. Frustrating stimulus — recognized, not entrained
3. Pleasant stimulus — acknowledged without clinging
4. Entitative self content — flagged by governance and identity kernel

## File Structure

```
buddhist-agent-rs/
├── Cargo.toml
├── README.md
├── IMPLEMENTATION_BRIEF.md
├── SUBAGENT_PROMPT.md
├── src/
│   ├── lib.rs          — crate root, module declarations
│   ├── main.rs         — binary demo
│   ├── domain.rs       — core types: Stimulus, Percept, Thought, Action, InternalState
│   ├── world.rs        — World Interface (前五识)
│   ├── cognition.rs    — Cognition (第六识, heuristic mode)
│   ├── identity.rs     — Identity Kernel (第七识)
│   ├── memory.rs       — Memory Substrate (第八识)
│   ├── governance.rs   — Governance (戒镜)
│   ├── skandhas.rs     — Five skandhas mapping
│   ├── engine.rs       — Pipeline orchestrator
│   └── llm.rs          — MiniMax M2.7 LLM adapter (LLM mode)
└── .gitignore
```

## Key Design Decisions

- **No external dependencies** (beyond std) — this is a pure domain model
- **Identity kernel deliberately light** — no full myself-binding, just constraints
- **Memory is impression-based** — not a database; seeds accumulate and fade
- **Governance is a mirror** — not a filter that passes/rejects everything, but a reflective layer
- **Self-attachment always bounded** — enforced by the identity kernel config
