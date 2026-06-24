# ADR-001: Initial Tech Stack

**Date:** 2026-06-24
**Status:** Superseded by ADR-002

## Context

We need to choose a technology stack for building a 2D exploration/survival game. The primary options are:

1. **Game engine** (Bevy, macroquad, ggez) — provides ECS, rendering, audio, and input out of the box
2. **Raw Vulkan** (ash + winit) — manual control over the entire rendering pipeline

## Decision

Use **Rust with raw Vulkan bindings** via the following stack:

| Crate | Role |
|-------|------|
| `ash` | Vulkan API bindings |
| `winit` | Cross-platform windowing and input |
| `raw-window-handle` | Vulkan surface creation bridge |
| `gpu-allocator` | GPU memory management |
| `glam` | Linear algebra (vectors, matrices) |
| `log` + `env_logger` | Logging |

The decision is motivated by:
- **Full control** over the rendering pipeline, enabling custom optimizations for 2D rendering
- **Learning** — building a renderer from scratch deepens understanding of GPU programming
- **Minimal dependency surface** — only pull in what's needed, no framework lock-in
- **Performance** — direct Vulkan access avoids engine abstraction overhead

## Consequences

**Easier:**
- Custom rendering optimizations (batching, atlas management, shader control)
- No framework version churn or API breakage from upstream engines
- Lean binary with only necessary dependencies

**Harder:**
- Significant boilerplate for Vulkan initialization (instance, device, swapchain, render passes)
- Must implement systems that engines provide for free (sprite batching, camera, input mapping, audio)
- More `unsafe` code to manage correctly
- Slower initial progress before anything renders on screen
