# Architecture — A Long Journey

## Current State

The project is a playable prototype with terrain generation, decoration spawning, and basic player movement. The three-scale procedural generation model is partially implemented (world vectors and local chunk generation are working; medium-scale block synchronization is not). Two of five planned spawn families are implemented (group-closed and recurring).

## Workspace Structure

```
a-long-journey/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── core/               # alj-core: shared game logic
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs          # Module exports, constants (CHUNK_SIZE, TILE_SIZE)
│   │       ├── terrain.rs      # Terrain enum (7 types), height mapping, colors
│   │       ├── chunk.rs        # ChunkPos, Chunk struct, terrain generation
│   │       ├── decoration.rs   # Decoration types, affinity system, group + recurring spawning
│   │       ├── world.rs        # GameWorld (chunk storage, on-demand generation)
│   │       └── world_vectors.rs # Large-scale noise fields (temperature, moisture, affinity, density)
│   ├── client/             # alj-client: macroquad game client
│   │   ├── Cargo.toml
│   │   └── src/main.rs     # Game loop, input, camera, tile + decoration rendering
│   └── server/             # alj-server: authoritative server (stub)
│       ├── Cargo.toml
│       └── src/main.rs
```

### alj-core (library)
Shared game logic compiled into both client and server. Contains:
- Terrain types and height-to-terrain mapping
- Chunk generation: 16x16 tiles from layered Perlin noise (3 octaves)
- World vectors: large-wavelength noise fields for regional variation
- Decoration system: 12 decoration types, affinity matrix, group and recurring spawning
- GameWorld: HashMap-based chunk storage with on-demand generation

Dependencies: `hecs`, `glam`, `noise`, `rand`, `serde`

### alj-client (binary)
The player-facing game. Renders terrain tiles and decorations, handles WASD movement with tile collision, camera tracking, viewport culling, and HUD.

Dependencies: `alj-core`, `macroquad`

### alj-server (binary)
Headless authoritative server. Currently a placeholder stub.

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
- **hecs ECS:** Minimal, archetype-based ECS with built-in serialization — chosen for multiplayer readiness (not yet used for entities; decorations are stored as Vec in chunks)
- **Systems are functions:** No scheduler framework — systems are plain functions that take `&mut World`
- **Deterministic generation:** Same seed + coordinates = identical output. Seeded RNG per chunk via `hash(world_seed, chunk_x, chunk_y)`

## Procedural Generation Model

Three-scale deterministic generation driven by a single **world seed**.

```
world_seed
  ├── Continental  (scale ~0.001)  → future: more WorldVectors fields
  ├── Regional     (scale ~0.003)  → WorldSample (temperature, moisture, affinity, density)
  │     sampled per chunk center   → modulates decoration spawning
  ├── Block/Medium (scale ~0.006)  → BlockSample (height_bias, roughness, dither)
  │     sampled per tile           → modulates terrain height computation
  └── Local/Chunk  (scale 0.02–0.1) → 3-octave Perlin → 16x16 terrain + decorations
```

### Large scale — World vectors (implemented)

`WorldVectors` contains four regional Perlin noise fields at large wavelength (scale 0.003):
- **Temperature** — reserved for biome blending
- **Moisture** — boosts flower/flower-bush spawn rates
- **Affinity field** — shifts which decoration families dominate (conifer vs temperate)
- **Density** — multiplies overall spawn rate per chunk (0.4x–1.0x)

### Medium scale — Blocks (implemented)

`WorldVectors` also contains three block-scale Perlin noise fields that modulate terrain generation per tile:
- **height_bias** (scales 0.006/0.015, amplitude ~0.12) — shifts elevation regionally, creating areas that are systematically higher (more stone/snow) or lower (more water/sand)
- **roughness** (scale 0.008, range 0.5–1.5) — multiplies high-frequency noise octaves, creating flat plains and rough mixed-terrain regions
- **threshold_dither** (scale 0.2, amplitude ~0.04) — fine noise added to height before threshold mapping, breaking hard contour lines into ragged natural edges

`Block` struct: 8×8 chunks (128×128 tiles). `BlockPos` maps chunk/world positions to block coordinates. Blocks cache a center `BlockSample` for future block-level features (interior entrances, mutations). Chunk generation samples noise directly per tile via `WorldVectors::sample_tile()`.

### Local scale — Chunks (implemented)

**Terrain:** 7 types mapped from layered Perlin noise height values:
| Terrain | Height Range | Walkable |
|---------|-------------|----------|
| DeepWater | < -0.3 | No |
| Water | -0.3 to -0.1 | No |
| Sand | -0.1 to 0.0 | Yes |
| Grass | 0.0 to 0.4 | Yes |
| Forest | 0.4 to 0.65 | Yes |
| Stone | 0.65 to 0.8 | Yes |
| Snow | > 0.8 | Yes |

**Decoration spawning** — two-phase system:
1. **Group spawns (closed-shape family):** Chunk-level rolls for tree groves, flower patches, rock formations, mountain clusters. Circular placement with 70/30 primary/secondary type split.
2. **Recurring spawns:** Per-tile probability checks modulated by world vectors and affinity accumulator from nearby placed decorations (6-tile falloff radius).

### Affinity system (implemented)

Sparse asymmetric matrix of ~30 relationships between 12 decoration types. `AffinityTracker` records placed decorations and computes distance-weighted probability boosts for subsequent spawns. Shared between group and recurring phases so clusters attract related fill.

### Biome system (implemented)

7 biomes driven by temperature × moisture noise (scale 0.003): Temperate, Desert, Jungle, Swamp, Tundra, Mountain, Coastal. Each biome defines terrain height thresholds, color palette, decoration multipliers, height_bias, and roughness_mult.

Biome blending uses inverse-distance weighting to biome climate centers in (temperature, moisture) space. Top 2 biomes are blended, producing smooth transitions. Sampled per tile during chunk generation. Colors stored in `Chunk::tile_colors` (computed at generation time from blended biome palette).

Decoration system wired into biome via per-decoration-type multipliers. Desert suppresses trees, Jungle amplifies them, Mountain boosts rocks/peaks, etc.

### Not yet implemented
- Group-open spawn family (path/line placement)
- Group-vector spawn family (direction field alignment)
- Unique spawn family (exclusion zones)
- Event-triggered spawn chains
- Mutations (player-caused world changes)
- Interior spaces

## Planned Components

_To be defined as features are implemented:_

- **Pixel-perfect rendering** — Render to low-res target, scale up with nearest-neighbor
- **Player sprite** — Replace yellow rectangle with animated character
- **World Generation: interior entrances** — Caves/buildings seeded from position
- **Entity Spawning: remaining families** — Group-open, group-vector, unique, events
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
| [005](adr-005-decoration-affinity-system.md) | Decoration System with Affinity-Driven Spawning | Accepted |
