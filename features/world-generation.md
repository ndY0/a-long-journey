# World Generation

**Status:** Proposed
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

## Acceptance Criteria

- [ ] World seed produces identical chunks on client and server
- [ ] Adjacent chunks have seamless terrain borders
- [ ] Entity affinity system produces believable co-occurrence patterns
- [ ] All five spawn families implemented and configurable
- [ ] Events trigger and chain correctly
- [ ] Interior spaces accessible and deterministically generated
- [ ] Mutations persist through server restarts
