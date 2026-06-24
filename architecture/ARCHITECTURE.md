# Architecture — A Long Journey

## Current State

The project is structured as a Cargo workspace with three crates. No game systems are implemented yet — the crates contain placeholder entry points.

## Workspace Structure

```
a-long-journey/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── core/               # alj-core: shared game logic
│   │   ├── Cargo.toml
│   │   └── src/lib.rs
│   ├── client/             # alj-client: macroquad game client
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── server/             # alj-server: authoritative server
│       ├── Cargo.toml
│       └── src/main.rs
```

### alj-core (library)
Shared game logic compiled into both client and server. Contains:
- hecs World, components, and systems
- World/map generation (noise-based procedural generation)
- Game rules, survival mechanics
- Serialization of entities and map chunks

Dependencies: `hecs`, `glam`, `noise`, `rand`, `serde`

### alj-client (binary)
The player-facing game. Renders the world, handles input, communicates with server in multiplayer.

Dependencies: `alj-core`, `macroquad`

### alj-server (binary)
Headless authoritative server. Runs the same game logic as the client, arbitrates map generation, manages multiplayer state, runs AI systems.

Dependencies: `alj-core`

## Dependency Graph

```
alj-client ──┐
             ├── alj-core
alj-server ──┘       │
                      ├── hecs (ECS, column-serialize + serde)
                      ├── glam (math, serde)
                      ├── noise (procedural generation)
                      ├── rand (randomness)
                      └── serde (serialization)

alj-client
  └── macroquad (2D rendering, input, windowing)
```

## Key Design Decisions

- **Shared core:** All game logic lives in `alj-core` so client and server run identical simulation code
- **hecs ECS:** Minimal, archetype-based ECS with built-in serialization — chosen for multiplayer readiness
- **Map chunks as entity bundles:** Procedurally generated map regions serialize as groups of hecs entities for network transfer and persistence
- **Systems are functions:** No scheduler framework — systems are plain functions that take `&mut World`

## Procedural Generation Model

Three-scale deterministic generation driven by a single **world seed**.

```
world_seed
  ├── world_noise(seed)           → biome, affinity, rendering, difficulty fields
  ├── block_rng(seed, bx, by)     → change vectors, border interpolation
  └── chunk_rng(seed, cx, cy)     → 16x16 terrain, entity spawns, events
```

**Local (chunk 16x16):** Terrain tiles + entity spawning with affinity-weighted tables. Five spawn families (group-closed, group-open, group-vector, unique, recurring). Deterministic event rolls trigger spawn chains.

**Medium (block):** Border sync via shared edge noise. Change vectors modulate generation smoothly across chunk boundaries. Interior entrances (caves, buildings) seeded from position.

**Large (world):** Noise-based vector fields control biome, entity affinity, rendering variations (color/shape), and difficulty gradient across the whole world.

**Mutations:** Player-caused deviations from generated baseline persist globally on the server.

## Planned Components

_To be defined as features are implemented:_

- **Renderer** — Tile/sprite drawing via macroquad, pixel-perfect scaling, camera
- **Input** — macroquad input mapped to game actions
- **World Generation** — Multi-scale seeded generation, chunk management, border sync
- **Entity Spawning** — Affinity system, spawn families, event chains
- **Survival Systems** — Player needs, resources, crafting
- **AI Systems** — NPC behavior, world events, player interaction splicing
- **Networking** — Client-server communication, mutation replication, seed sharing

## ADRs

| ID | Title | Status |
|----|-------|--------|
| [001](adr-001-initial-tech-stack.md) | Initial Tech Stack | Superseded by ADR-002 |
| [002](adr-002-switch-to-macroquad.md) | Switch to Macroquad | Accepted |
| [003](adr-003-ecs-hecs.md) | hecs as ECS Framework | Accepted |
| [004](adr-004-procedural-generation.md) | Multi-Scale Deterministic Procedural Generation | Accepted |
