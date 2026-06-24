# ADR-004: Multi-Scale Deterministic Procedural Generation

**Date:** 2026-06-24
**Status:** Accepted

## Context

The game requires a procedurally generated 2D top-down world that:
- Feels continuous and coherent across vast distances
- Supports multiplayer via deterministic seeded generation (Minecraft-style)
- Has rich entity spawning with affinity relationships and spawn families
- Supports events that trigger action/spawn chains
- Includes interior spaces (caves, buildings) accessible from the overworld
- Allows the authoritative server to arbitrate without transferring full map data

## Decision

Adopt a **three-scale generation model** driven by a single deterministic **world seed**.

### Scale 1: Local — Chunk (16x16 tiles)

The atomic generation unit. Each chunk's RNG is derived from `hash(world_seed, chunk_x, chunk_y)`.

**Entity spawning:**
- Spawn tables driven by local context (biome, terrain, neighbors)
- Entities have **affinity scores** — spawning one shifts probability of related entities
- Five spawn families:
  - **Group (closed shape):** cluster forming a bounded shape (grove, stone ring)
  - **Group (open shape):** entities along a path with no enclosure (stone river, flower trail)
  - **Group (vector-following):** aligned to a direction field (wind-blown grass, erosion)
  - **Unique:** at most one per chunk or region (shrine, rare node)
  - **Recurring:** probability-based, multiple per chunk (bushes, rocks, common flora)

**Events:**
- Deterministic event rolls at generation time
- Events trigger chains: e.g., "meteor" → crater → debris → rare ore
- Events can span adjacent chunks

### Scale 2: Medium — Block (group of contiguous chunks)

**Border synchronization:**
- Adjacent chunks sample shared edge noise for seamless terrain transitions
- Biome transitions use blending zones at block boundaries

**Change vectors:**
- Each block carries gradient vectors modulating local generation (terrain density, entity frequency, palette)
- Smooth interpolation between blocks creates continuous landscape variation

**Interior entrances:**
- Special tiles mark transitions to interior spaces (caves, buildings)
- Interior layout is deterministically generated from `hash(world_seed, entrance_x, entrance_y)`
- No extra data to sync in multiplayer — same seed produces same interior

### Scale 3: Large — World vectors

Authoritative vector fields generated from the world seed via large-wavelength noise:
- **Biome field** — dominant biome at any world position
- **Affinity field** — shifts which entity families are favored regionally
- **Rendering modulation** — color palette shifts, prop shape variations
- **Difficulty/density gradient** — resource scarcity, hazard frequency

### Deterministic seeding

```
world_seed
  ├── world_noise(seed)           → large-scale fields
  ├── block_rng(seed, bx, by)     → change vectors, border params
  └── chunk_rng(seed, cx, cy)     → terrain, spawns, events
```

Any client or server with the same seed and coordinates produces identical output. The server only stores **mutations** (player-caused deviations from the generated baseline). Mutations persist globally — the world has permanent history.

## Consequences

**Easier:**
- Multiplayer sync: transmit seed + mutations, not full map data
- Deterministic replay and debugging
- Infinite world without pre-generation
- Interior spaces require no extra storage or sync

**Harder:**
- Generation algorithm changes break existing worlds (seed produces different output)
- Must ensure all platforms produce bit-identical results from the same seed
- Complex affinity/event systems need careful balancing
- Persistent mutations require server-side storage that grows over time
