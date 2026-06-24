# Features

This directory tracks game features as individual markdown files. Features are developed incrementally — each file describes one cohesive feature.

## Conventions

- **One file per feature**, named with short kebab-case: `player-movement.md`, `hunger-system.md`, `tile-rendering.md`
- Keep filenames explicit and short (2-4 words max)
- Features reference each other by filename when they have dependencies

## Template

Use this structure when creating a new feature file:

```markdown
# Feature Name

**Status:** Proposed | In Progress | Implemented | Deferred
**Priority:** High | Medium | Low
**Depends on:** [list of feature filenames, or "None"]

## Description

What this feature does from the player's perspective.

## Requirements

- Concrete, testable requirements
- One bullet per requirement

## Technical Notes

Implementation guidance, relevant modules, or architectural considerations.

## Acceptance Criteria

- [ ] Criterion 1
- [ ] Criterion 2
```

## Current Features

- [world-generation.md](world-generation.md) — Multi-scale deterministic world generation
- [entity-spawning.md](entity-spawning.md) — Affinity-based entity spawning with spawn families
