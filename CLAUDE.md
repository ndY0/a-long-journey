# CLAUDE.md — Project Instructions

## Project

**A Long Journey** — A 2D exploration/survival game with multiplayer support, written in Rust with macroquad and hecs ECS.

## Tech Stack

- Rust edition 2024
- `macroquad` — lightweight 2D engine (rendering, input, windowing, game loop)
- `hecs` — minimal ECS with column-serialize + serde for network-ready serialization
- `glam` — linear algebra (re-exported by macroquad, serde-enabled in core)
- `noise` — procedural generation (Perlin, Simplex, etc.)
- `rand` — randomness
- `serde` — serialization for components and map chunks

## Workspace Structure

```
├── Cargo.toml                # Workspace root
├── SPECIFICATION.md          # Game specification (keep in sync with implementation)
├── BRAINSTORM.md             # Session decision log
├── features/                 # One markdown file per feature (kebab-case names)
├── architecture/
│   ├── ARCHITECTURE.md       # Living architecture document (always current)
│   └── adr-*.md              # Architecture Decision Records
└── crates/
    ├── core/                 # alj-core: shared game logic (hecs, systems, world gen)
    ├── client/               # alj-client: macroquad rendering + input
    └── server/               # alj-server: authoritative server + AI
```

### Crate responsibilities
- **alj-core:** All game logic, components, systems, world generation. Shared between client and server. No rendering code here.
- **alj-client:** macroquad rendering, input handling, network client. Depends on alj-core.
- **alj-server:** Headless server, network authority, AI systems. Depends on alj-core.

## Working Conventions

### Features
- Each feature is a markdown file in `features/` with a short kebab-case name
- Follow the template in `features/README.md`
- Update feature status as implementation progresses
- Reference SPECIFICATION.md sections where relevant

### Architecture
- Keep `architecture/ARCHITECTURE.md` up to date with the current system design
- When making a significant architectural decision, create an ADR: `architecture/adr-NNN-short-title.md`
- ADR format is documented in `architecture/README.md`
- Never let ARCHITECTURE.md drift from reality — update it when code changes

### Session Discipline
- At the end of each session, add an entry to `BRAINSTORM.md` with the date and key decisions
- Record only decisions and reasoning, not discarded ideas
- Use absolute dates (not "today" or "yesterday")

### Code
- No unnecessary abstractions — build what's needed now
- Game logic goes in `alj-core`, rendering in `alj-client`, server logic in `alj-server`
- All network-relevant components must derive `Serialize` and `Deserialize`
- macroquad uses an async main loop — all client code runs inside `async fn main()`
- Pixel art: render to a low-res render target, scale up with nearest-neighbor filtering
- Systems are plain functions taking `&mut hecs::World` — no scheduler framework
- Run `cargo check` frequently; `cargo clippy` before committing

## Build & Run

```bash
cargo run -p alj-client    # Run the game client
cargo run -p alj-server    # Run the server
cargo check                # Type-check all crates
cargo clippy               # Lint all crates
cargo test                 # Run tests across workspace
```
