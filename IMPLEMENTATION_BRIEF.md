# Buddhist Agent Architecture — Rust Implementation Brief

目标：在这个目录下实现一个可运行、可测试的 Rust agent 架构原型，核心灵感来自我们沉淀的佛学 × agent 架构笔记。

必做要求：
- 使用 Rust
- 代码可编译
- 至少包含核心架构分层与状态流转
- 要有测试
- 产出 README，解释架构与运行方式

建议最小实现：
- world interface / cognition / identity / memory / governance 五层
- 支持一次 perception -> cognition -> decision -> memory update 的完整循环
- 支持 identity constraints 与 governance checks
- 用清晰的数据结构表达 Buddhist mapping

资料来源：
- ../our-discussions/notes/2026-04-09-buddhist-agent-architecture-existing-models.md
- ../our-discussions/notes/2026-04-09-buddhist-agent-architecture-qa-archive.md

输出要求：
- Rust crate
- Cargo.toml
- src/ 下实现
- tests/ 或内联测试
- README.md
