# Entity Spawning

**Status:** In Progress
**Priority:** High
**Depends on:** world-generation.md

## Description

A system for spawning entities into generated chunks based on context, affinity relationships, and spawn family rules. Entities are not placed randomly — they follow patterns that make the world feel authored despite being procedural.

## Requirements

### Affinity system
- Each entity type has affinity scores with other entity types
- Spawning an entity increases the probability of high-affinity entities spawning nearby
- Affinity is bidirectional but can be asymmetric (wolves attract bones, but bones don't attract wolves as strongly)

### Spawn families

**Group — closed shape:**
- A cluster of entities forming a bounded area (circle, convex polygon, irregular blob)
- Parameters: entity type, count range, shape type, radius range
- Example: a grove of trees, a ring of standing stones

**Group — open shape:**
- Entities placed along a path or line with no enclosure
- Parameters: entity type, count range, path algorithm (random walk, bezier), length range
- Example: a trail of flowers, a line of rocks along a ridge

**Group — vector-following:**
- Entities aligned along a direction field (from world vectors or local gradients)
- Parameters: entity type, density, alignment strength
- Example: grass bent by prevailing wind, erosion scarring

**Unique:**
- At most one instance per chunk or per region
- Exclusion zone prevents duplicates in adjacent chunks
- Example: a shrine, a boss-spawn point, a rare resource

**Recurring:**
- Standard probability-based spawning, multiple instances allowed
- Parameters: entity type, probability per tile, density cap
- Example: common bushes, pebbles, background flora

### Event-triggered spawns
- Events roll at chunk generation time (deterministic)
- An event defines a spawn chain: sequence of spawns and tile modifications
- Events have rarity tiers and biome requirements
- Example: "ancient battle" → scattered weapon entities + scorched ground tiles + skeleton entities

## Technical Notes

- Spawn tables are data-driven (define in config/data files, not hardcoded)
- Affinity matrix stored as a sparse structure (most pairs have zero affinity)
- All spawned entities are hecs components — serializable for network sync
- Spawn family algorithms live in `alj-core`

## Implementation Status

### Implemented (2026-06-24)

- **12 decoration types:** PineTree, OakTree, BirchTree, Bush, FlowerBush, Rock, Boulder, MountainPeak, Pebbles, Flowers, Driftwood, SnowPile
- **Affinity matrix:** Sparse function `affinity(from, to) -> f32` with ~30 asymmetric relationships. Examples: PineTree→Bush (0.3), OakTree→Flowers (0.2), Rock→Boulder (0.25), MountainPeak→SnowPile (0.25).
- **AffinityTracker:** Records placed decorations during chunk generation. Computes distance-weighted boost for candidate spawns (linear falloff over 6-tile radius). Shared between group and recurring phases.
- **Group-closed family:** Four group types — tree groves (4–8 entities, radius 2.5–4.5), flower patches (4–10, radius 2.0–4.0), rock formations (3–6, radius 2.0–3.5), mountain clusters (2–4, radius 2.5–4.0). Circular placement with 70% primary / 30% affinity-selected secondary. Roll probability gated by terrain composition and world vector density.
- **Recurring family:** Per-tile probability checks for all 12 types against terrain. Base rates 1–8%, modulated by world density, affinity field, moisture, and accumulated affinity from nearby spawns. Capped at 50%.
- **World vector modulation:** Conifer bonus (positive affinity field) vs temperate bonus (negative affinity field). Moisture boosts flowers. Density field scales all spawn rates.

### Not yet implemented

- **Group-open family:** Path/line-based placement (flower trails, rock ridges)
- **Group-vector family:** Direction-field-aligned placement (wind-blown grass, erosion)
- **Unique family:** At-most-one-per-chunk with exclusion zones (shrines, rare resources)
- **Event-triggered spawns:** Deterministic event rolls with spawn chains
- **Data-driven spawn tables:** Currently hardcoded in Rust; spec calls for config files
- **hecs integration:** Decorations are stored as `Vec<DecorationInstance>` in chunks, not as hecs entities

## Acceptance Criteria

- [x] Affinity system produces visible clustering of related entities
- [ ] All five spawn families produce distinct, recognizable patterns
- [ ] Unique spawns correctly enforce exclusion zones
- [ ] Event chains execute and produce expected entity/tile patterns
- [x] Spawn results are deterministic given the same seed and coordinates
