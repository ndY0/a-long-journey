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
