# Entity Spawning

**Status:** Proposed
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

## Acceptance Criteria

- [ ] Affinity system produces visible clustering of related entities
- [ ] All five spawn families produce distinct, recognizable patterns
- [ ] Unique spawns correctly enforce exclusion zones
- [ ] Event chains execute and produce expected entity/tile patterns
- [ ] Spawn results are deterministic given the same seed and coordinates
