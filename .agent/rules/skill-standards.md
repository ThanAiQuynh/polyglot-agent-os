# Skill & Tooling Standards
@docs/skills.md

## Agent Awareness
Before starting a technical task, the agent MUST review the available skills in `docs/skills.md`.

## Tool Usage
- If a tool exists in `docs/skills.md` but is not yet implemented (Phase 1), the agent MUST prioritize its implementation in the corresponding service (Rust/Python).
- All new tools MUST be appended to `docs/skills.md` to maintain the "Universal Skill Map".

## Cross-Service Skills
- **Linguistic Skills**: Always seek for them in the Rust Core Linguistic service via gRPC.
- **Reasoning Skills**: Orchestrated in Python AI Tutor.
