# World Generation

**Status:** In Progress
**Priority:** High
**Depends on:** None

## Description

Procedurally generated 2D top-down world using a deterministic world seed. The generation operates at three scales — local (chunk), medium (block), and large (world vectors) — to produce a coherent, varied, and infinite landscape.

## Requirements

### Deterministic seeding
- A single world seed drives all generation
- `chunk_rng(seed, x, y)` produces identical output for the same inputs across all platforms
- Clients and server generate chunks independently from the seed

### Local scale — Chunks (16x16 tiles)
- Terrain tiles generated from layered noise
- Entity spawning via affinity-weighted spawn tables
- Five spawn families: group-closed, group-open, group-vector, unique, recurring
- Deterministic event rolls that trigger action/spawn chains
- Events can span adjacent chunks

### Medium scale — Blocks
- Adjacent chunks sync borders via shared edge noise
- Change vectors modulate local generation parameters across block boundaries
- Smooth interpolation creates continuous landscape
- Interior entrances (caves, buildings) generated deterministically from entrance position

### Large scale — World vectors
- Biome field (large-wavelength noise)
- Affinity field (entity family preferences by region)
- Rendering modulation (color, shape variations)
- Difficulty/density gradient

### Mutations
- Player-caused world changes (destroyed terrain, placed items) persist globally
- Server stores mutations as deviations from the generated baseline
- Clients apply mutations on top of deterministic generation

## Technical Notes

- Generation lives in `alj-core` — shared between client and server
- Use `noise` crate for Perlin/Simplex noise at multiple octaves and scales
- Seeded RNG via `rand` with deterministic seed derivation per chunk
- Chunk data serializable via hecs column-serialize for network transfer when needed
- Interior spaces are separate hecs Worlds, seeded from entrance coordinates

## Implementation Status

### Implemented (2026-06-24)

- **Deterministic seeding:** Chunk RNG derived from `hash(world_seed, chunk_x, chunk_y)` via wrapping multiplication with large primes. `StdRng::seed_from_u64` ensures reproducibility.
- **Local scale terrain:** 7 terrain types from layered Perlin noise at 3 octaves (scales 0.02, 0.05, 0.1 with weights 0.6, 0.3, 0.1). Height thresholds map to DeepWater/Water/Sand/Grass/Forest/Stone/Snow.
- **Large scale world vectors:** `WorldVectors` with 4 Perlin fields (temperature, moisture, affinity, density) at wavelength 0.003. Sampled per chunk center. Affinity field shifts conifer vs temperate family preference. Density field scales spawn rate 0.4x–1.0x. Moisture boosts flower spawns.
- **On-demand chunk loading:** `GameWorld` generates and caches chunks as needed with radius-based loading around player position.

- **Medium scale blocks:** `WorldVectors` extended with 3 block-scale Perlin fields (height_bias at scales 0.006/0.015, roughness at 0.008, dither at 0.2). Sampled per tile during terrain generation. Roughness modulates high-frequency octaves (0.5x–1.5x). Height_bias shifts elevation (amplitude ~0.12). Dither breaks threshold contour lines (amplitude ~0.04). `Block` struct (8×8 chunks) with `BlockPos` for block-level addressing. Blocks cached in `GameWorld` for future features.
- **Hierarchical noise architecture:** Continental (~0.001, future) → Regional (~0.003) → Block (~0.006) → Local (0.02–0.1). Adding scales = adding Perlin fields at new frequencies.

- **Biome system:** 7 biomes (Temperate, Desert, Jungle, Swamp, Tundra, Mountain, Coastal) driven by temperature × moisture noise at scale 0.003. Each biome defines terrain thresholds, color palette, decoration multipliers, height_bias, roughness_mult. Inverse-distance blending between top-2 biomes creates smooth transitions. Sampled per tile. Colors stored in `Chunk::tile_colors`. Decoration system modulated by biome multipliers.

### Not yet implemented

- Interior entrances (caves, buildings) deterministically seeded from position
- Difficulty gradient
- Mutations (player-caused world changes)
- Cross-chunk event spanning

## Acceptance Criteria

- [x] World seed produces identical chunks on client and server
- [ ] Adjacent chunks have seamless terrain borders
- [x] Entity affinity system produces believable co-occurrence patterns
- [ ] All five spawn families implemented and configurable
- [ ] Events trigger and chain correctly
- [ ] Interior spaces accessible and deterministically generated
- [ ] Mutations persist through server restarts
