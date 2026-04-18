# Architecture

This document describes the current architecture baseline of the project.

It is intentionally separate from `ROADMAP.md`:

- `ARCHITECTURE.md` describes what exists now and the technical direction already established
- `ROADMAP.md` describes the feature milestones that are still planned

## Current Goals

The project is still mechanically simple, but the codebase is already being shaped toward a larger-game architecture. The current implementation is designed to support future work on:

- health and damage systems
- difficulty progression
- AI-controlled birds
- multiplayer

## Core Design Principles

### 1. Simulation vs Presentation

Simulation state should not live primarily in rendering-facing components.

Current direction:

- simulation state is represented with components such as `Position`, `Velocity`, `BirdIntent`, and `Collider`
- presentation follows simulation through sync systems
- `Transform` and `Sprite` are treated as presentation state

This keeps the game easier to extend toward AI, health, replay, and multiplayer.

### 2. Intent-Based Control

Player input is not applied directly to simulation state from the input system.

Current direction:

- input systems write `BirdIntent`
- simulation systems consume `BirdIntent` during fixed-step gameplay updates

This keeps the control path compatible with future AI and multiplayer input sources.

### 3. Explicit World Scroll

World movement is modeled explicitly rather than being hidden in one-off systems or shader time.

Current direction:

- gameplay uses `world_scroll_speed`
- background parallax reads explicit scroll state
- shader behavior is driven by material uniforms, not global shader time

This is important for future difficulty scaling and synchronized obstacle generation.

### 4. Message-Driven Transient Facts

Transient gameplay facts should be modeled explicitly.

Current direction:

- coarse restart behavior is expressed through `RunRestartRequested`
- scoring is expressed through `ScorePoint`

This is still a simple model, but it is more extensible than embedding all control flow in direct system side effects.
The current restart behavior is an intentional placeholder for the current game mechanics, not an accidental partial implementation of the final run lifecycle.

### 5. State-Ready Game Flow

The game already has a minimal `GameState` scaffold even though only `Playing` is currently active.

This keeps the codebase ready for:

- ready/countdown states
- game over
- pause
- menu/lobby flows

## Current Module Roles

The `src/game/` directory is organized by concern.

- `background.rs`
  - background rendering model
  - parallax data and sync logic
- `bird.rs`
  - player/bird spawning
  - input capture
  - movement, bounds, collision, rotation
- `pipes.rs`
  - timed obstacle spawning
  - obstacle positioning and movement
- `run.rs`
  - current coarse restart behavior
- `score.rs`
  - score resource and score message handling
- `model.rs`
  - shared gameplay components and transform sync
- `config.rs`
  - game tuning values
- `messages.rs`
  - transient gameplay messages
- `state.rs`
  - high-level game state

## Current Baseline Systems

### FixedUpdate

Fixed-step systems own core simulation behavior.

The current flow is roughly:

1. consume bird intent
2. apply gravity
3. integrate velocity into position
4. move and spawn pipes
5. sync transforms from simulation state
6. check bounds and collisions
7. apply score and restart side effects

This is the main simulation path and should remain the place where future gameplay rules are applied.

### Update

Frame-rate systems own visual responsiveness and input capture.

Current responsibilities include:

- capturing player input into intent
- updating parallax offsets and syncing them into materials
- updating bird presentation state such as rotation
- syncing transforms for rendering

## Current Temporary Constraints

Some parts of the implementation are intentionally still transitional:

- collisions still trigger a coarse run restart rather than damage/death semantics
- obstacle generation is still simple and mostly pipe-specific
- only one bird exists in the simulation
- the current `GameState` is minimal

These are expected to evolve, but the code should continue to move toward explicit, reusable simulation concepts rather than ad hoc feature logic.
They should not be replaced preemptively with broader abstractions before the current milestone actually needs them.

## Near-Future Direction

The next major architectural evolution should build on the current baseline rather than replacing it:

- restart semantics should eventually split into collision, damage, elimination, and run-lifecycle concepts such as start, respawn, and reset
- obstacle generation should evolve toward a clearer separation between generation policy/state and entity spawning
- bird simulation should remain shared across human, AI, and network-controlled birds
- world scroll should remain the common source for obstacle movement and background motion

Until those steps are active, the current coarse restart behavior and simple obstacle generation logic remain valid placeholders.

## Testing Strategy

The project now includes lightweight unit tests for small pure helpers that are likely to evolve, such as:

- bounds checks
- restart positioning
- obstacle placement math
- parallax offset math

As the game grows, continue extracting testable pure logic instead of pushing more behavior into opaque ECS-only systems.
