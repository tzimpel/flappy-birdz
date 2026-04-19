# flappy-birdz

Small Flappy Bird-style prototype built with Bevy.

The project is being used as a learning and architecture sandbox for:

- scalable ECS-style gameplay code in Bevy
- a health/difficulty single-player mode with HUD health output, safe pipe-passage scoring, boundary impact/contact damage, regeneration, and delayed game-over restart
- AI-controlled competing birds
- eventual multiplayer support

## Run

```bash
cargo run
```

## Controls

Use the left or right mouse button to flap.

## Current Gameplay Baseline

- one human-controlled bird
- health-based survival instead of instant death on pipe collision
- speed-scaled impact damage and continuous contact damage at top and bottom bounds
- passive healing after avoiding damage for a short delay
- safe-passage scoring per pipe couple
- difficulty ramping over time through explicit run and obstacle director resources
- automatic restart after a short `GameOver` delay

## Current Technical Direction

- simulation state is kept separate from presentation state
- input is captured as bird intent and consumed in simulation
- world scrolling is modeled explicitly and reused by the background parallax system
- the roadmap is single-player foundations first, AI second, multiplayer third

## Structure

- `src/main.rs`: minimal application bootstrap
- `src/lib.rs`: library entrypoint exporting `FlappyBirdPlugin`
- `src/game/`: gameplay modules for simulation, UI, background, pipes, run flow, and shared config
- `assets/`: textures and shader source
- `ROADMAP.md`: future milestones and feature sequencing
- `ARCHITECTURE.md`: current architecture baseline and guiding technical model
- `AGENTS.md`: repo-local instructions for coding agents

## Quality Checks

```bash
cargo fmt --check
cargo check
cargo clippy --all-targets --all-features
cargo test
```
