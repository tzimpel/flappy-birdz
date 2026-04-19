# Flappy Birdz Roadmap

This roadmap breaks the project into three milestones:

1. Single-player foundations
2. AI competitors
3. Multiplayer

The sequence is intentional. Each milestone introduces gameplay features only after the underlying gameplay model is strong enough to support them cleanly. The goal is not only to grow this game, but to grow it in a way that teaches Bevy, ECS, and Rust patterns that still make sense in a larger game.

## Guiding Principles

- Keep simulation state separate from presentation state where possible.
- Prefer durable data in components/resources over implicit event chains.
- Use states for large-scale game flow.
- Use messages for transient gameplay facts that multiple systems may consume.
- Use observers only for local reactive behavior, not as the main orchestration mechanism.
- Make systems reusable across human players, AI players, and networked players.
- Keep obstacle generation deterministic enough that it can later support replay and multiplayer synchronization.
- Keep placeholder implementations explicit and intentional until the milestone that actually needs them to evolve.
- Prefer small, reviewable milestone steps over broad framework-building in advance.

## Milestone 1: Single-Player Foundations

This milestone turns the current prototype into a stronger single-player game while establishing the architectural base for everything that comes after.

### Goals

- Replace instant death on collision with health-based survival.
- Add passive healing over time when the bird avoids damage.
- End the run only when health reaches zero.
- Add difficulty progression over the course of a run.
- Make pipe spacing and gap size dynamic within configured ranges.
- Keep the game continuous and readable, but model progression explicitly.

### Gameplay Features

- Bird health with a defined maximum.
- Pipe collision causes damage instead of immediate game over.
- Health regeneration over time up to max health.
- Run progression that increases challenge over time.
- Dynamic pipe gap size within a configured range.
- Dynamic distance or spawn cadence for pipes within a configured range.
- Scoring that remains valid as a measure of survival/progress.

### ECS / Bevy Architecture Goals

- Introduce a `GameState` for phases such as `Ready`, `Playing`, and `GameOver`.
- Separate player input intent from movement and collision simulation.
- Introduce explicit gameplay components such as:
  - `Health`
  - `MaxHealth`
  - `RegenRate`
  - `BirdIntent`
  - `Alive` or `Eliminated`
  - `Collider`
- Introduce a run-management resource such as:
  - `RunDirector`
  - `DifficultyDirector`
  - `ObstacleDirector`
- Move pipe generation rules into one explicit system/resource instead of scattering values through systems.
- Ensure that scoring, damage, healing, and death are derived by systems from state and transient messages.

### Placeholder Notes

- The current coarse restart behavior is intentional placeholder behavior, not an accidental unfinished lifecycle model.
- It is expected to stay coarse until Milestone 1 reaches the point where restart semantics need to split into concepts such as `RunStart`, `Respawn`, and `RunReset`.
- The current obstacle generation logic is also allowed to remain simple until obstacle generation policy is explicitly tackled.
- Deterministic future direction matters, but Milestone 1 does not require prematurely choosing the final obstacle-generation algorithm.

### Recommended Data Model

- Components on birds:
  - `Bird`
  - `PlayerControlled`
  - `Velocity`
  - `Health`
  - `MaxHealth`
  - `RegenRate`
  - `BirdIntent`
  - `Collider`
- Resources:
  - `GameConfig`
  - `RunDirector`
  - `ObstacleRng` or a deterministic spawn seed/state
  - `Scoreboard` if scoring becomes more global
- Messages:
  - `DamageTaken`
  - `BirdHealed`
  - `BirdDied`
  - `GatePassed`
  - `RunEnded`

### Why This Comes First

This milestone creates the durable simulation model that AI and multiplayer both require later. Without it, AI would need to plug into ad hoc player-specific logic, and multiplayer would need to synchronize behavior that is not yet modeled explicitly enough.

### Suggested Implementation Breakdown

The milestone benefits from being split into reviewable changesets that each introduce one clear gameplay or lifecycle capability at a time.

1. Add minimal run-state and lifecycle structure needed to support non-instant-death gameplay.
   This covers introducing or tightening phase/state concepts such as `Ready`, `Playing`, and `GameOver` only to the extent required for later steps.
2. Introduce bird health as explicit simulation data.
   Add components/resources for `Health`, `MaxHealth`, and any immediate supporting configuration without changing collision behavior yet.
3. Replace collision-driven instant game over with collision-driven damage.
   Pipe hits should now produce damage through explicit gameplay messages/systems rather than directly ending the run.
4. Add bird death and run end behavior once health can reach zero.
   This is the point where elimination semantics become meaningful and the run ends because the bird died, not because a collision happened.
5. Add passive healing and its gating rules.
   Introduce regeneration over time up to max health, using explicit state and timing rather than presentation-driven behavior.
6. Add explicit run progression and difficulty scaling.
   Model progression as data/resources so challenge increases over time through dedicated systems.
7. Apply progression to obstacle generation ranges.
   Make pipe gap size and spacing/spawn cadence evolve within configured bounds once progression data exists.
8. Stabilize scoring and UI against the updated gameplay model.
   Ensure score, health presentation, and game-over feedback reflect damage, healing, survival, and progression correctly.

### Suggested Implementation Order

1. Minimal run-state and lifecycle structure
2. Explicit bird health data
3. Collision-to-damage conversion
4. Death and run-end handling
5. Passive healing
6. Run progression and difficulty scaling
7. Dynamic obstacle generation ranges
8. Scoring and UI stabilization

### Exit Criteria

- The game has one human-controlled bird.
- The bird survives collisions until health reaches zero.
- Difficulty ramps over time through explicit systems/resources.
- Pipe generation is controlled by data that can later be reused by AI and multiplayer.
- The simulation no longer depends on direct UI/input coupling.

## Milestone 2: AI Birds

This milestone adds competing birds controlled by systems instead of human input. It should reuse the same simulation model as the player.

### Goals

- Add one or more AI-controlled birds to the same run.
- Show them as translucent or visually distinct birds.
- Let all birds compete under the same rules.
- Determine a winner based on survival and possibly score.
- Keep AI implementation pluggable so different bot types can be added later.

### Gameplay Features

- Multiple birds active in the same run.
- AI birds with different skill levels or strategies.
- Elimination tracking.
- Winner determination when only one bird remains alive, or based on end-of-round scoring.
- Visual distinction between player and AI competitors.

### ECS / Bevy Architecture Goals

- Generalize the concept of a bird so the player is not special-cased.
- Replace `Player`-specific assumptions with `Bird` plus controller-type components.
- Introduce controller components/resources such as:
  - `HumanController`
  - `AiController`
  - `BotProfile`
- Keep AI decision-making as a producer of `BirdIntent`, not as a direct manipulator of transforms or velocity.
- Add a round-resolution system that determines standings and winners.

### Recommended Data Model

- Shared bird components:
  - `Bird`
  - `Velocity`
  - `Health`
  - `BirdIntent`
  - `Collider`
  - `Alive` or `Eliminated`
- Controller markers/components:
  - `HumanController`
  - `AiController`
  - `BotDifficulty`
  - `RenderStyle` or similar for translucency/visual differentiation
- Messages:
  - `BirdEliminated`
  - `WinnerDeclared`
  - `RoundFinished`

### AI Strategy Options

- Heuristic bots:
  - Cheap, deterministic, easy to debug.
  - Good first step.
- Search/planning bots:
  - Better performance potential.
  - More complexity and more state to manage.
- External AI/ML:
  - Interesting long-term option.
  - Not a good dependency for early architecture.

### Suggested Recommendation

Start with deterministic heuristic bots that read world state and write `BirdIntent`. This keeps the control path identical between humans and AI while staying simple enough to reason about in ECS.

### Why This Comes Second

AI becomes much simpler once the simulation already supports health, progression, and generalized birds. It also forces the architecture to prove that “controller” is separate from “bird simulation,” which is a useful checkpoint before multiplayer.

Do not front-load AI-oriented abstractions into Milestone 1 unless a specific single-player task already needs them.

### Suggested Implementation Order

1. Rename/generalize player-specific concepts to bird concepts where needed.
2. Add multiple bird spawning and shared bird lifecycle rules.
3. Add controller marker components.
4. Implement one baseline heuristic bot.
5. Add AI rendering differences.
6. Add round winner and standings logic.
7. Add additional bot profiles if desired.

### Exit Criteria

- One human and one or more AI birds can compete in the same run.
- All birds use the same simulation systems.
- AI writes intents rather than bypassing the gameplay model.
- Winner determination is explicit and system-driven.

## Milestone 3: Multiplayer

This milestone replaces one or more AI birds with remote players while keeping the core simulation model the same.

### Goals

- Support more than one real player in the same run.
- Keep pipe positions and run progression synchronized.
- Preserve fair competition across clients.
- Reuse the same bird simulation path used by single-player and AI.

### Gameplay Features

- Remote players participate in the same run.
- Shared obstacle stream and shared progression.
- Winner/standings work for both local and remote participants.
- AI birds may remain available as fillers.

### ECS / Bevy Architecture Goals

- Make obstacle generation authoritative and deterministic.
- Represent player actions as networkable intent data.
- Avoid hidden reactive chains that are difficult to replay or synchronize.
- Separate local prediction concerns from authoritative simulation concerns if needed.
- Introduce a player identity model distinct from bird entities.

### Recommended Data Model

- Components/resources:
  - `Bird`
  - `PlayerId`
  - `LocalPlayer`
  - `RemotePlayer`
  - `BirdIntent`
  - `AuthoritativeRunState`
  - `SpawnSequence` or deterministic obstacle state
- Messages:
  - `PlayerInputReceived`
  - `BirdStateReplicated`
  - `RoundResultReplicated`
  - `ObstacleSpawned`

### Networking Model Options

- Lockstep / deterministic simulation:
  - Strong consistency.
  - Sensitive to divergence and latency.
- Server authoritative with state replication:
  - More practical for most games.
  - Requires prediction/interpolation if responsiveness matters.
- Hybrid:
  - More complexity.
  - Usually not needed first.

### Suggested Recommendation

Prefer a server-authoritative model with synchronized obstacle generation and intent replication. It is more forgiving than full deterministic lockstep and fits the likely evolution of this project better.

### Why This Comes Third

Multiplayer is easiest when:

- birds are already generalized
- input is already modeled as intent
- obstacle generation is already explicit
- scoring and elimination are already system-driven

That is exactly what the first two milestones are meant to establish.

Do not proactively introduce multiplayer-oriented framework code before the milestone actually needs it.

### Suggested Implementation Order

1. Introduce player identity and remote/local controller concepts.
2. Make obstacle generation authoritative and reproducible.
3. Serialize and transmit bird intents instead of raw transform state where possible.
4. Replicate authoritative bird/run state.
5. Add winner/score synchronization.
6. Replace or supplement AI birds with remote players.
7. Add prediction/interpolation only if needed for feel.

### Exit Criteria

- Two or more real players can participate in the same run.
- Pipe generation/progression is synchronized across participants.
- Networked players reuse the same simulation model as local players and AI birds.
- AI birds can remain optional participants instead of being a separate game mode.

## Cross-Cutting Technical Themes

These themes should be preserved across all milestones.

### 1. Simulation vs Presentation

Avoid making `Transform`, `Sprite`, or UI state the authoritative gameplay model. Keep gameplay state in dedicated components/resources and let rendering follow that data.

### 2. Intent-Based Control

All control sources should converge on the same input abstraction:

- human input
- AI logic
- multiplayer/network input

They should all feed `BirdIntent` or an equivalent abstraction.

### 3. Explicit Run Control

Keep progression, spawning, and difficulty in dedicated systems/resources rather than hiding them in unrelated systems or observers.

### 4. Deterministic Enough Generation

Even before multiplayer, obstacle generation should be explicit and reproducible enough that replay, AI evaluation, and synchronization remain realistic later.

This does not require committing to a final obstacle algorithm before obstacle generation policy is an active milestone concern.

### 5. Data-Driven Tuning

Difficulty, healing, damage, spawn intervals, gap ranges, and scoring should all be config-driven and easy to tune without rewriting simulation logic.

## Things To Postpone Until They Are Needed

- Fancy AI models
- Prediction/interpolation for networking
- Replays
- Persistence/meta-progression
- Advanced UI and menus
- More complex physics integrations

These are all valid future directions, but they should build on the simulation and control model established in the earlier milestones.

## Summary

The intended progression is:

1. Build a robust single-player simulation model.
2. Prove that multiple controllers can drive the same birds by adding AI.
3. Replace some of those controllers with remote players for multiplayer.

If this order is maintained, each step should feel like an extension of the same architecture instead of a rewrite caused by earlier shortcuts.
