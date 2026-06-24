# ADR-005: Decoration System with Affinity-Driven Spawning

**Date:** 2026-06-24
**Status:** Accepted

## Context

The procedural generation model (ADR-004) specifies three-scale generation with affinity-weighted entity spawning and five spawn families. The first implementation attempt used per-tile random spawning with fixed probabilities — this produced props that were too dense, too small, and had no spatial coherence. Decorations appeared uniformly distributed with no regional character or clustering of related entities.

The system needed to produce visually authored-feeling placement despite being fully procedural and deterministic.

## Decision

### World vectors (large scale)

Introduced `WorldVectors` — four Perlin noise fields derived from the world seed (offsets +1000, +2000, +3000, +4000) sampled at large wavelength (scale 0.003, medium detail 0.008):

- **Temperature** — reserved for future biome blending, currently unused by terrain
- **Moisture** — boosts flower and flower-bush spawn probabilities in wet regions
- **Affinity field** — shifts which decoration families dominate regionally: positive values favor conifer family (pine, bush, rock), negative values favor temperate family (oak, birch, flowers)
- **Density** — multiplies overall spawn rate per chunk (range 0.4x–1.0x)

Each chunk samples `WorldVectors` at its center tile to produce a `WorldSample` passed into decoration generation.

### Affinity system (local scale)

Implemented as a sparse function `affinity(from: Decoration, to: Decoration) -> f32` defining ~30 asymmetric directional relationships. Examples: PineTree→Bush (0.3), OakTree→Flowers (0.2), Rock→Boulder (0.25), Boulder→Rock (0.3).

An `AffinityTracker` records all placed decorations during chunk generation. For each candidate spawn, it sums distance-weighted affinity contributions from all previously placed decorations within a 6-tile radius (linear falloff). This boost is added to the base spawn probability before the roll.

The tracker is shared between group spawning (phase 1) and recurring spawning (phase 2), so group clusters naturally attract related recurring spawns nearby.

### Two-phase spawning per chunk

**Phase 1 — Group spawns (closed-shape family):**
Chunk-level rolls based on terrain composition and world vectors. Four group types:
- Tree groves (forest terrain, 4–8 entities, radius 2.5–4.5 tiles)
- Flower patches (grass terrain, 4–10 entities, radius 2.0–4.0 tiles)
- Rock formations (stone terrain, 3–6 entities, radius 2.0–3.5 tiles)
- Mountain clusters (snow terrain, 2–4 entities, radius 2.5–4.0 tiles)

Each group picks a center tile of matching terrain, places entities in a circular pattern (70% primary type, 30% affinity-weighted secondary types), and records placements in the affinity tracker.

**Phase 2 — Recurring spawns:**
Per-tile probability checks for each decoration type against the current terrain. Base rates are low (1–8%) and modulated by world density, affinity field, moisture, and accumulated affinity boost from phase 1 and earlier phase 2 placements. Capped at 50% to prevent over-saturation.

### Decoration rendering scale

All decoration dimensions are expressed relative to `TILE_SIZE` (16px) rather than absolute pixel counts. Trees span 1.0–1.5 tiles in height, mountains ~1.8 tiles, bushes ~0.6 tiles. Decorations extend beyond their anchor tile for natural overlap.

## Consequences

**Easier:**
- Regional character emerges naturally — moving across the world reveals areas dominated by different vegetation or geological features
- Clusters of related entities look intentional (a pine grove with undergrowth bushes and scattered pebbles)
- Density varies meaningfully — some areas are lush, others sparse
- System is deterministic — same seed + position = same decorations everywhere

**Harder:**
- Affinity tracker scans all placed decorations per candidate (O(N) per tile) — acceptable for 16x16 chunks but may need spatial indexing if chunks grow
- Group spawn rolls are terrain-composition dependent — changes to terrain thresholds cascade into decoration placement
- Three spawn families not yet implemented (group-open, group-vector, unique) — current architecture supports adding them as additional phases
- Affinity matrix is hardcoded — spec calls for data-driven spawn tables, which would require loading from config files
