# A Long Journey — Project Specification

## Overview

**Title:** A Long Journey
**Genre:** 2D Exploration / Survival
**Platform:** Desktop (Linux, Windows)
**Language:** Rust (edition 2024)
**Engine:** macroquad (lightweight 2D, raylib-inspired)
**Visual Style:** 8-bit pixel art, top-down view
**World:** Dynamically generated environments

## Vision

A 2D exploration game where the player traverses a vast, procedurally generated world while managing survival needs. The game supports multiplayer where players choose to collaborate or confront each other. AI systems splice the experience — shaping events, NPC behavior, and world dynamics. Game instances share their generated map blocks through an authoritative server.

## Gameplay Pillars

1. **Exploration** — A world worth wandering through, with secrets and varied biomes
2. **Survival** — Meaningful resource management that drives exploration decisions
3. **Multiplayer tension** — Players decide whether to cooperate or compete; AI amplifies the stakes
4. **Atmosphere** — Visual and audio design that reinforces the feeling of a long journey

## Core Mechanics

TBD — To be discussed in feature sessions.

## World

Procedurally generated from a deterministic **world seed** (Minecraft-style). Three generation scales:

**Local (chunk — 16x16 tiles):**
- Terrain from layered noise
- Entity spawning with affinity relationships (entities that "belong together" co-occur)
- Five spawn families: group-closed, group-open, group-vector, unique, recurring
- Events roll at generation time and trigger spawn/action chains

**Medium (block — group of chunks):**
- Border synchronization via shared edge noise
- Change vectors create smooth landscape transitions
- Interior entrances (caves, buildings) — deterministically seeded from world position

**Large (world vectors):**
- Biome field, affinity field, rendering modulation (color/shape shifts), difficulty gradient
- Large-wavelength noise ensures regional coherence

**Mutations:** Player changes persist globally on the server as deviations from the generated baseline.

## Player Systems

TBD — Movement, inventory, interaction model.

## Survival Systems

TBD — Hunger, thirst, stamina, shelter, crafting, hazards.

## Art Direction

TBD — Visual style, palette, tile/sprite approach, lighting.

## Audio

TBD — Music, ambient sounds, sound effects.

## Multiplayer

TBD — Networking protocol, lobby/matchmaking, client prediction, server reconciliation.

- Authoritative server arbitrates world state and map generation
- Game instances share procedurally generated map chunks
- Players interact in a shared world — cooperation and confrontation mechanics
- AI systems run server-side to splice gameplay experiences

## Technical Constraints

- **Engine:** macroquad — lightweight 2D rendering, immediate-mode draw calls, built-in input/window
- **ECS:** hecs — minimal archetype ECS with column-serialize for network-ready serialization
- **Math:** `glam` (re-exported by macroquad, serde-enabled)
- **Proc-gen:** `noise` crate for terrain/biome generation
- **Randomness:** `rand` crate
- **Serialization:** `serde` — all network-relevant components are serializable
- **Architecture:** Cargo workspace — `alj-core` (shared logic), `alj-client` (rendering), `alj-server` (authority + AI)
- **Target:** 60 FPS minimum on modest hardware
- **Rendering approach:** Render to low-res target, scale up with nearest-neighbor for crisp pixel art
