# AGENTS.md

This file contains repository-specific guidance for coding agents working in this project.

## Project Intent

This repository is a Bevy-based Flappy Bird-style game that is being used as a foundation for learning scalable ECS architecture in Rust.

Planned progression:

1. Single-player foundations with health, damage, and difficulty progression
2. AI-controlled competing birds
3. Multiplayer

When making changes, prefer decisions that keep those future milestones easy to support.

## Current Architectural Guardrails

- Keep simulation state separate from presentation state.
- Do not make `Transform`, `Sprite`, or UI state the authoritative gameplay model.
- Input should flow through intent-style components/resources rather than mutating simulation directly from input systems.
- Background/parallax behavior should be driven by ECS data, not by implicit shader-global time.
- Prefer bird-centric concepts over player-special-case logic where practical.
- Use explicit messages, resources, and states for game flow rather than hiding control flow in reactive chains.

## Current Naming Direction

Prefer these terms where applicable:

- `bird` instead of player-specific naming for shared simulation concepts
- `world_scroll_speed` instead of `pipe_speed`
- `restart` for the current coarse run restart behavior
- `run` for session-level flow
- `intent` for control inputs that are later consumed by simulation

## Scope Guidance

- Do not change gameplay behavior unless explicitly requested.
- Prefer extracting pure helper functions for logic that is likely to evolve.
- When introducing architecture improvements, keep the current mechanics intact unless the user asks otherwise.
- If a behavior is intentionally temporary, do not over-engineer it, but avoid naming that blocks future evolution.
- Treat placeholder behavior as intentional unless the user explicitly asks to replace it.
- Do not "complete", generalize, or replace placeholder implementations just because their future direction is known.
- During Milestone 1 work, do not proactively introduce AI- or multiplayer-oriented abstractions unless the current requested step directly requires them.
- Prefer the smallest complete and reviewable change set that satisfies the requested step.
- If a requested step has prerequisites that would expand scope too far, stop, explain the prerequisites, and let the user choose rather than silently broadening the implementation.

## Verification

From the repository root, use:

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features
cargo test
```

## Documentation

- `README.md` describes the project at a high level.
- `ROADMAP.md` describes future milestones and sequencing.
- `ARCHITECTURE.md` documents the current baseline architecture.

Keep those documents in sync when architectural direction changes materially.
If milestone granularity or sequencing changes in a meaningful way, update the roadmap and any affected architecture guidance in the same task.
