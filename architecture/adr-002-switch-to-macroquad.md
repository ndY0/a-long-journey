# ADR-002: Switch from Raw Vulkan to Macroquad

**Date:** 2026-06-24
**Status:** Accepted
**Supersedes:** ADR-001 (Initial Tech Stack)

## Context

ADR-001 chose raw Vulkan via `ash` for maximum rendering control. After evaluating the game's requirements — 2D, 8-bit pixel art style, top-down view, procedurally generated environments — the cost/benefit shifted:

- Raw Vulkan requires building a full 2D rendering pipeline (sprite batching, tilemap renderer, camera, input) before any gameplay work can begin
- The game does not need 3D, custom shaders, or GPU compute — it needs fast 2D sprite/tile drawing
- Macroquad provides exactly this with minimal API surface and near-zero boilerplate

Engines considered:
- **Bevy** — powerful ECS + tilemap plugins, but heavier than needed and frequent breaking API changes
- **Macroquad** — minimal, raylib-inspired, immediate-mode 2D rendering, lightweight
- **Comfy** — similar to macroquad but smaller community
- **ggez** — mature but slower development pace

## Decision

Replace `ash`, `winit`, `raw-window-handle`, `gpu-allocator`, `log`, and `env_logger` with **macroquad**.

Retain `glam` (macroquad re-exports it). Add `noise` for procedural generation and `rand` for randomness.

## Consequences

**Easier:**
- Immediate 2D rendering with `draw_texture`, `draw_rectangle`, camera support
- Pixel-perfect 8-bit rendering via render targets with nearest-neighbor scaling
- Built-in input handling, window management, and game loop
- Can start gameplay development immediately
- WASM target support for browser builds

**Harder:**
- Less control over the rendering pipeline (no custom Vulkan shaders)
- Immediate-mode rendering may need care for large tile counts (batching)
- No built-in ECS — must choose and integrate separately
