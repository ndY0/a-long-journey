# ADR-003: hecs as ECS Framework

**Date:** 2026-06-24
**Status:** Accepted

## Context

The game needs an Entity Component System to manage game entities (player, NPCs, tiles, items, hazards). Key constraints:

- **Multiplayer-ready:** Components must serialize for network replication between client and authoritative server
- **Shared logic:** The same ECS world and systems must compile into both the client and a headless server
- **Map chunk sharing:** Procedurally generated map chunks are bundles of entities that must serialize, transfer, and deserialize across game instances
- **AI integration:** AI systems need to query and mutate the ECS world alongside gameplay systems
- **No Bevy:** We use macroquad, so `bevy_ecs` is not an option — need a standalone ECS

Options evaluated:

| ECS | Serialization | Maintenance | Fit |
|-----|--------------|-------------|-----|
| **hecs** | Built-in column-serialize + serde | Active, stable API | Best — minimal, serializable, no framework lock-in |
| **legion** | Built-in serde | Low activity (Amethyst defunct) | Risky — maintenance uncertainty |
| **shipyard** | None built-in | Active | Poor — manual serialization is a large burden for multiplayer |

## Decision

Use **hecs** with `column-serialize` and `serde` features enabled.

Structure the project as a Cargo workspace:
- `alj-core` — hecs World, components, systems, world generation (shared between client and server)
- `alj-client` — macroquad rendering, input, network client
- `alj-server` — authoritative game server, network, AI systems

All serializable components derive `serde::Serialize` and `serde::Deserialize`. The `column-serialize` feature enables efficient bulk serialization of entity groups (map chunks).

## Consequences

**Easier:**
- Serialize map chunks as entity bundles — natural fit for network transfer and persistence
- Shared `alj-core` crate compiles identically for client and server
- AI systems are plain functions over the hecs World — no framework magic
- Minimal API surface reduces learning curve and maintenance burden
- glam types serialize via serde feature for positions, velocities, etc.

**Harder:**
- No built-in system scheduler — must implement a simple runner (list of functions)
- No change detection — must build dirty-tracking for network-relevant components
- No built-in event system — must implement a message bus for player actions and AI events
