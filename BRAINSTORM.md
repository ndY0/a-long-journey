# Brainstorm — Session Log

Reasoning highlights from each working session. Captures **decisions made and why** — not discarded ideas or open questions.

---

## 2026-06-24 — Project Bootstrap

- Initialized Rust project with `cargo init`, edition 2024
- Chose raw Vulkan via `ash` over a game engine (Bevy, macroquad) for full rendering control and as a learning exercise
- Selected core dependencies: `ash` (Vulkan), `winit` (windowing), `gpu-allocator` (memory), `glam` (math), `log`/`env_logger` (logging)
- Established documentation structure: SPECIFICATION.md, features/, architecture/ with ADRs, BRAINSTORM.md, CLAUDE.md
- BRAINSTORM.md will track decisions only — not discarded alternatives
- Evaluated game engines: Bevy (heavy, ECS+plugins), macroquad (lightweight, immediate-mode), Comfy, ggez
- Switched from raw Vulkan (ash) to **macroquad** — the game is 2D/8-bit/top-down, raw Vulkan would mean months of engine work before gameplay
- Added `noise` and `rand` crates for procedural generation
- ADR-002 traces the engine switch decision, ADR-001 marked superseded
- Game will be multiplayer (collaborate or confront), with AI splicing the experience
- Authoritative server will arbitrate map generation and shared world state
- Evaluated ECS options: hecs (minimal, serializable), legion (unmaintained), shipyard (no serialization)
- Chose **hecs** with `column-serialize` + `serde` — best fit for serializing map chunks and network replication
- Restructured project into Cargo workspace: `alj-core` (shared), `alj-client` (macroquad), `alj-server` (headless)
- ADR-003 traces the ECS decision
- Designed multi-scale procedural generation: local (16x16 chunks), medium (blocks of chunks), large (world vector fields)
- Entity spawning uses affinity-weighted tables — entities that "belong together" co-occur naturally
- Five spawn families: group-closed, group-open, group-vector, unique, recurring
- Events roll deterministically at generation time and trigger spawn/action chains
- Deterministic seeded generation (Minecraft-style) — same seed + coordinates = same output everywhere
- Interior spaces (caves, buildings) seeded from entrance world position — no extra sync needed
- Player mutations persist globally on the server as deviations from generated baseline
- ADR-004 traces the procedural generation design
- **Next:** Discuss initial feature priorities and begin implementation

---

## 2026-06-24 — Terrain Prototype & Decoration System

- Implemented basic terrain generation: 7 terrain types (DeepWater, Water, Sand, Grass, Forest, Stone, Snow) mapped from layered Perlin noise at 3 octaves (scales 0.02, 0.05, 0.1)
- Implemented chunk system: 16x16 tile chunks generated on demand, stored in HashMap keyed by ChunkPos
- Client renders colored rectangles per tile with viewport culling, camera follows player, WASD movement with tile-based collision
- Added decoration/prop system with 12 decoration types: PineTree, OakTree, BirchTree, Bush, FlowerBush, Rock, Boulder, MountainPeak, Pebbles, Flowers, Driftwood, SnowPile
- Initial decoration pass spawned per-tile with fixed probabilities — too dense, too small, no spatial coherence
- Redesigned decoration system to follow the three-scale generation model from ADR-004:
  - **Large scale (world vectors):** `WorldVectors` struct with 4 Perlin noise fields (temperature, moisture, affinity, density) at wavelength ~0.003. Each chunk samples its center to get a `WorldSample` that modulates all spawning decisions
  - **Affinity field** maps to decoration families: positive values favor conifers (pine, bush, rock), negative values favor temperate (oak, birch, flowers). Moisture boosts flower/flower-bush spawns
  - **Density field** scales overall spawn probability from 0.4x to 1.0x per chunk
- Implemented **affinity system** as a sparse matrix via `affinity(from, to) -> f32` function — ~30 directional relationships between decoration types. `AffinityTracker` accumulates placed decorations and computes distance-weighted boost (linear falloff over 6-tile radius) for subsequent spawn decisions
- Implemented two spawn families from the five planned:
  - **Group-closed:** Circular clusters placed at chunk level. Four group types: tree groves (4–8 entities), flower patches (4–10), rock formations (3–6), mountain clusters (2–4). Groups are 70% primary type, 30% affinity-selected secondary. Roll probability gated by terrain composition and world vectors
  - **Recurring:** Per-tile probability checks with base rates of 1–8% (down from 15–70%), modulated by world density, affinity field, moisture, and accumulated affinity from nearby group spawns
- Three spawn families remain unimplemented: group-open, group-vector, unique
- Scaled up decoration rendering: all sizes relative to TILE_SIZE (16px). Trees span 1–1.5 tiles tall, mountains ~1.8 tiles, bushes ~0.6 tiles. Decorations overlap neighboring tiles for natural appearance
- Event-triggered spawns not yet implemented
- ADR-005 traces the decoration and affinity system decisions
- Chunk texture caching: each chunk's terrain + decorations baked into a `RenderTarget` (256×256 GPU texture) on first visibility, then drawn as a single textured quad per frame — reduced draw calls from ~4000+ to ~12 per frame

---

## 2026-06-24 — Medium-Scale Block Synchronization

- Implemented medium-scale terrain modulation to smooth terrain transitions and create regional variation
- Added three block-scale noise fields to `WorldVectors` (seed offsets +5000, +6000, +7000):
  - **height_bias** (2 octaves, scales 0.006/0.015, amplitude ~0.12): shifts terrain elevation regionally — some areas are systematically higher (more stone/snow) or lower (more water/sand)
  - **roughness** (1 octave, scale 0.008, range 0.5–1.5): multiplies high-frequency noise octaves — creates flat plains and rough mixed-terrain areas
  - **threshold_dither** (1 octave, scale 0.2, amplitude ~0.04): fine-grained noise added to height before threshold mapping — breaks hard terrain-type contour lines into natural ragged edges
- Roughness only modulates octaves 2 and 3 (scales 0.05, 0.1), preserving the base terrain shape from octave 1 (scale 0.02)
- `Terrain::from_height()` thresholds remain fixed — all modulation happens on the input height value
- Created `Block` concept: 8×8 chunks (128×128 tiles) per block. `BlockPos` maps chunk/world coordinates to block coordinates. `Block` struct stores center `BlockSample` for future features (interior entrances, mutations). Chunk generation samples noise directly per tile, not via cached block values
- Architecture designed for hierarchical infinite scaling: continental (~0.001) → regional (~0.003, existing) → block (~0.006, new) → local (0.02–0.1, existing). Adding a new scale = adding more Perlin fields at the appropriate frequency
- All computation is deterministic from seed + position — no pre-generation, works at infinite scale
- **Next:** Implement biome system for large-scale landscape evolution

---

## 2026-06-24 — Biome System

- Implemented 7 biomes: Temperate, Desert, Jungle, Swamp, Tundra, Mountain, Coastal
- Biome selection driven by existing temperature × moisture noise fields (scale 0.003, wavelength ~333 tiles ≈ 44s walk) — these were already in WorldVectors but only affected decorations, now they drive terrain generation
- Each biome defines a `BiomeProfile`: custom terrain height thresholds, color palette (7 RGBA values), decoration spawn multipliers (12 values), height_bias, and roughness_mult
- Biome blending uses inverse-distance weighting to each biome's climate center in (temperature, moisture) space — top 2 biomes blend all profile values (thresholds, colors, decoration mults, biases) producing smooth transitions with no hard edges
- Climate centers in (temp, moisture) space: Desert(0.6,-0.5), Jungle(0.6,0.6), Swamp(0.0,0.6), Tundra(-0.6,0.2), Mountain(-0.6,-0.4), Coastal(0.15,0.3), Temperate(0.0,0.0)
- Biome sampled per tile during chunk generation — sub-chunk biome transitions are possible
- Added `tile_colors` array to `Chunk` struct — colors computed at generation time from blended biome palette, replacing the static `terrain.color()` lookup
- Decoration system wired into biome: `generate_decorations()` accepts biome decoration multipliers, applied to both recurring and group spawn chances — Desert suppresses trees (0.0x), Jungle doubles them (2.0x), Mountain triples mountain peaks (3.0x), etc.
- Biome roughness_mult composes with block-scale roughness — e.g. Mountain(1.6x) × block roughness creates very jagged terrain in mountain regions
- Biome height_bias composes with block-scale height_bias — Mountain(+0.15) pushes terrain higher, Desert(-0.1) pushes lower
---

## 2026-06-24 — Decoration Expansion, Dynamic Effects, Visual Polish

### Decoration diversity
- Expanded from 12 to 25 decoration types: added Cactus, PalmTree, DeadTree, TallGrass, DryShrub, SandDune, Reeds, Mushroom, FlatRock, IceChunk, Fern, Moss, Seashell
- Biome-exclusive decorations via per-type multipliers: Cactus/SandDune in Desert only, PalmTree/Fern in Jungle, DeadTree/Reeds in Swamp, IceChunk in Tundra, Seashell at Coast
- Added affinity relationships for all new types (Cactus→DryShrub, Mushroom→Moss→Fern, etc.)
- All rendering as geometric primitives in draw_decoration()

### Density reduction
- All recurring spawn base rates divided by 5x (e.g. Forest PineTree: 0.08→0.016)
- Group spawn chances reduced ~3x, counts reduced (groves: 4-8→2-5, etc.)
- **Anti-overlap system**: AffinityTracker now tracks occupied tiles per chunk — 2nd decoration on same tile gets 15% chance, 3rd+ gets 0%. Applied to both group and recurring spawns

### Non-blocking chunk loading
- Background thread for chunk generation via `std::sync::mpsc` channels
- Main thread sends ChunkPos requests, polls for completed chunks each frame via `try_recv`
- Spawn area (3×3 around origin) generated synchronously to avoid spawning into void
- Existing `ensure_chunks_around` kept for server-side synchronous use

### Dynamic visual effects (all client-side)
- **Cloud system**: semi-transparent ellipses drifting with wind. Wind smoothly changes direction every 15-30s. Clouds spawn at upwind edge, despawn when far off-screen. Shadow ellipses below each cloud
- **Terrain tile texture**: 2 deterministic detail dots (darker + lighter) per tile, baked into chunk RenderTarget. Stone gets additional crack lines. Water/DeepWater skipped (handled by wave system)
- **Player trail & grass interaction**: ring buffer of last ~2s of player positions. TallGrass, Bush, Flowers, Fern within 3 tiles of trail redrawn with displacement (pushed away from player path). Baked "at rest" version shows through when player leaves
- **Water wave ripples**: particle-based system. Individual ripples spawn randomly on visible water tiles (2-5/sec). Each ripple: expanding ring that grows and fades over 1-3s with secondary inner ring. Replaced original uniform sine pattern

### Day/night cycle
- 20-minute full cycle (DAY_CYCLE_DURATION = 1200s), starts at dawn
- Daylight follows sine curve: noon=full brightness, midnight=near-greyscale
- Night overlay: dark blue (10,10,30) at up to 180 alpha + gray desaturation at up to 60 alpha
- Dawn/dusk: warm orange-pink tint (180,100,50) around phase 0.25 and 0.75
- HUD displays time-of-day label: Night/Dawn/Morning/Noon/Afternoon/Dusk

### Weather system
- **Types**: Normal (clear), Mist, Rain, Heatwave
- **Scheduling**: random non-overlapping events, 2-10 min duration, 5-15 min cooldown between. Never overlap
- **Time-of-day constraints**: Mist only at dawn/dusk/night, Heatwave only around noon, Rain anytime
- **10-second fade in/out** via transition field (0→1 on start, 1→0 on end), all alphas multiplied by transition
- **Mist**: smooth position-based gradient overlays (horizontal + vertical strips with sin/cos drift), denser at screen edges. No circles
- **Rain**: 150 diagonal drops (1.5px wide, 12px long, alpha 140), wind-angled. Splash effect on impact (expanding circle + spray lines). Lightning every 5-15s (bright white flash, exponential decay)
- **Heatwave**: GLSL shader-based distortion. Scene rendered to off-screen RenderTarget, then drawn to screen through fragment shader that displaces UV coords using two interfering sine waves. Creates real pixel-level wobble. Warm tint overlay on top

### Tile corner smoothing (work in progress)
- **Approach**: marching-squares style at grid intersections. When 3 of 4 tiles share a terrain type and 1 differs (3-vs-1 pattern), draw a quarter-circle fan of majority color INTO the minority tile's corner
- **Implementation**: triangle fan (8 segments) with explicit (dx,dy) arm endpoints — no sin/cos angle math, avoids render target Y-flip confusion
- **Cross-chunk support**: bake_chunk accepts a neighbor_lookup closure for tiles outside chunk boundaries, loop runs 0..=CHUNK_SIZE to handle chunk-border intersections
- **Status**: mostly working — corners are correctly identified, colors and directions are right. Still some edge cases to fix:
  - Some corners at chunk boundaries may still be missed if neighbor chunks aren't loaded yet
  - 2-vs-2 straight borders (where staircase meets a straight line) are not smoothed
  - Bilinear texture filtering (FilterMode::Linear) handles some straight-edge softening but not all
- **Next session**: continue debugging remaining corner cases, potentially add straight-edge blending

### Biome variation rate
- Temperature/moisture noise scales reduced from 0.003/0.008 to 0.0015/0.004 — biome wavelength ~88s walk per transition

### Technical notes
- Chunk textures use FilterMode::Linear for smoother tile edges
- Heatwave shader uses GLSL 100 (ES compatible): vertex pass-through + fragment UV displacement
- Scene RenderTarget for heatwave recreated on screen resize
- All weather/day-night/cloud state lives in client main.rs structs, updated each frame on main thread (trivially cheap)

### Known issues to address next session
- Tile corner smoothing needs more edge case work
- Props density may still need tuning per biome
- Decoration rendering at chunk borders (decorations extending beyond chunk texture get clipped)
- Documentation (ARCHITECTURE.md, features/) not updated for this session's changes

- **Next:** Fix remaining corner smoothing issues, tune decoration density, update docs, begin player/survival systems
