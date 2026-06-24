use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::decoration::{Decoration, DecorationInstance};

pub const CAVE_SIZE: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaveTile {
    Wall,
    Floor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaveEntrance {
    pub tile_x: u8,
    pub tile_y: u8,
    pub world_x: i32,
    pub world_y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaveExit {
    pub tile_x: u8,
    pub tile_y: u8,
    pub target_world_x: i32,
    pub target_world_y: i32,
}

#[derive(Debug, Clone)]
pub struct CaveMap {
    pub tiles: [[CaveTile; CAVE_SIZE]; CAVE_SIZE],
    pub decorations: Vec<DecorationInstance>,
    pub exits: Vec<CaveExit>,
    pub entrance_x: u8,
    pub entrance_y: u8,
}

pub fn should_spawn_entrance(seed: u32, chunk_x: i32, chunk_y: i32) -> bool {
    let h = seed
        .wrapping_mul(48271)
        ^ (chunk_x as u32).wrapping_mul(93481)
        ^ (chunk_y as u32).wrapping_mul(31547);
    (h % 50) == 0
}

pub fn pick_entrance_tile(
    seed: u32,
    chunk_x: i32,
    chunk_y: i32,
    tiles: &[[crate::terrain::Terrain; crate::CHUNK_SIZE]; crate::CHUNK_SIZE],
) -> Option<(u8, u8)> {
    let entry_seed = seed
        .wrapping_mul(71263)
        ^ (chunk_x as u32).wrapping_mul(48029)
        ^ (chunk_y as u32).wrapping_mul(67801);
    let mut rng = StdRng::seed_from_u64(entry_seed as u64);

    for _ in 0..30 {
        let tx = rng.random_range(2..crate::CHUNK_SIZE - 2);
        let ty = rng.random_range(2..crate::CHUNK_SIZE - 2);
        if tiles[ty][tx].walkable() {
            return Some((tx as u8, ty as u8));
        }
    }
    None
}

impl CaveMap {
    pub fn generate(world_seed: u32, entrance_world_x: i32, entrance_world_y: i32) -> Self {
        let cave_seed = world_seed
            .wrapping_mul(82757)
            ^ (entrance_world_x as u32).wrapping_mul(49297)
            ^ (entrance_world_y as u32).wrapping_mul(73939);
        let mut rng = StdRng::seed_from_u64(cave_seed as u64);

        // Phase 1: random fill
        let mut tiles = [[CaveTile::Wall; CAVE_SIZE]; CAVE_SIZE];
        for y in 1..CAVE_SIZE - 1 {
            for x in 1..CAVE_SIZE - 1 {
                tiles[y][x] = if rng.random_range(0..100) < 55 {
                    CaveTile::Floor
                } else {
                    CaveTile::Wall
                };
            }
        }

        // Phase 2: cellular automata smoothing (5 iterations)
        for _ in 0..5 {
            let mut next = [[CaveTile::Wall; CAVE_SIZE]; CAVE_SIZE];
            for y in 1..CAVE_SIZE - 1 {
                for x in 1..CAVE_SIZE - 1 {
                    let mut walls = 0u32;
                    for dy in -1i32..=1 {
                        for dx in -1i32..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let nx = (x as i32 + dx) as usize;
                            let ny = (y as i32 + dy) as usize;
                            if tiles[ny][nx] == CaveTile::Wall {
                                walls += 1;
                            }
                        }
                    }
                    next[y][x] = if walls >= 5 {
                        CaveTile::Wall
                    } else if walls <= 3 {
                        CaveTile::Floor
                    } else {
                        tiles[y][x]
                    };
                }
            }
            tiles = next;
        }

        // Phase 3: connectivity — flood fill to find regions, connect them
        let mut labels = [[0u16; CAVE_SIZE]; CAVE_SIZE];
        let mut region_id = 0u16;
        let mut region_sizes: Vec<(u16, usize, usize, usize)> = Vec::new(); // (id, size, sum_x, sum_y)

        for y in 0..CAVE_SIZE {
            for x in 0..CAVE_SIZE {
                if tiles[y][x] == CaveTile::Floor && labels[y][x] == 0 {
                    region_id += 1;
                    let mut stack = vec![(x, y)];
                    let mut size = 0usize;
                    let mut sum_x = 0usize;
                    let mut sum_y = 0usize;
                    while let Some((cx, cy)) = stack.pop() {
                        if labels[cy][cx] != 0 || tiles[cy][cx] != CaveTile::Floor {
                            continue;
                        }
                        labels[cy][cx] = region_id;
                        size += 1;
                        sum_x += cx;
                        sum_y += cy;
                        if cx > 0 { stack.push((cx - 1, cy)); }
                        if cx < CAVE_SIZE - 1 { stack.push((cx + 1, cy)); }
                        if cy > 0 { stack.push((cx, cy - 1)); }
                        if cy < CAVE_SIZE - 1 { stack.push((cx, cy + 1)); }
                    }
                    region_sizes.push((region_id, size, sum_x, sum_y));
                }
            }
        }

        if let Some(&(main_id, _, _, _)) = region_sizes.iter().max_by_key(|r| r.1) {
            for &(rid, size, sum_x, sum_y) in &region_sizes {
                if rid == main_id || size == 0 {
                    continue;
                }
                let small_cx = (sum_x / size) as i32;
                let small_cy = (sum_y / size) as i32;
                // Find closest main-region tile
                let mut best_dist = i32::MAX;
                let mut target = (CAVE_SIZE / 2, CAVE_SIZE / 2);
                for y in 0..CAVE_SIZE {
                    for x in 0..CAVE_SIZE {
                        if labels[y][x] == main_id {
                            let d = (x as i32 - small_cx).abs() + (y as i32 - small_cy).abs();
                            if d < best_dist {
                                best_dist = d;
                                target = (x, y);
                            }
                        }
                    }
                }
                // Carve L-shaped tunnel
                let (tx, ty) = target;
                let (sx, sy) = (small_cx as usize, small_cy as usize);
                let (min_x, max_x) = (sx.min(tx), sx.max(tx));
                let (min_y, max_y) = (sy.min(ty), sy.max(ty));
                for x in min_x..=max_x {
                    tiles[sy][x] = CaveTile::Floor;
                    if sy + 1 < CAVE_SIZE { tiles[sy + 1][x] = CaveTile::Floor; }
                }
                for y in min_y..=max_y {
                    tiles[y][tx] = CaveTile::Floor;
                    if tx + 1 < CAVE_SIZE { tiles[y][tx + 1] = CaveTile::Floor; }
                }
            }
        }

        // Phase 4: pillars in open areas
        for _ in 0..rng.random_range(3..8) {
            let px = rng.random_range(5..CAVE_SIZE - 5);
            let py = rng.random_range(5..CAVE_SIZE - 5);
            let mut open = true;
            for dy in -2i32..=2 {
                for dx in -2i32..=2 {
                    let nx = (px as i32 + dx) as usize;
                    let ny = (py as i32 + dy) as usize;
                    if tiles[ny][nx] == CaveTile::Wall {
                        open = false;
                    }
                }
            }
            if open {
                for dy in 0..2 {
                    for dx in 0..2 {
                        tiles[py + dy][px + dx] = CaveTile::Wall;
                    }
                }
            }
        }

        // Phase 5: entrance and exits
        let entrance_x;
        let entrance_y;
        // Find floor tile near center-bottom
        let mut found = false;
        let mut ex = CAVE_SIZE / 2;
        let mut ey = CAVE_SIZE - 5;
        for r in 0..20 {
            for dy in -(r as i32)..=(r as i32) {
                for dx in -(r as i32)..=(r as i32) {
                    let nx = (CAVE_SIZE as i32 / 2 + dx).clamp(1, CAVE_SIZE as i32 - 2) as usize;
                    let ny = (CAVE_SIZE as i32 - 5 + dy).clamp(1, CAVE_SIZE as i32 - 2) as usize;
                    if tiles[ny][nx] == CaveTile::Floor {
                        ex = nx;
                        ey = ny;
                        found = true;
                        break;
                    }
                }
                if found { break; }
            }
            if found { break; }
        }
        entrance_x = ex as u8;
        entrance_y = ey as u8;

        let mut exits = Vec::new();
        let exit_count = rng.random_range(1..4);
        let edge_positions = [
            (CAVE_SIZE / 4, 2),
            (3 * CAVE_SIZE / 4, 2),
            (2, CAVE_SIZE / 2),
            (CAVE_SIZE - 3, CAVE_SIZE / 2),
            (CAVE_SIZE / 2, CAVE_SIZE - 3),
        ];
        for &(epx, epy) in edge_positions.iter().take(exit_count) {
            // Find nearest floor tile to this edge position
            let mut best = None;
            let mut best_dist = i32::MAX;
            for y in 1..CAVE_SIZE - 1 {
                for x in 1..CAVE_SIZE - 1 {
                    if tiles[y][x] == CaveTile::Floor {
                        let d = (x as i32 - epx as i32).abs() + (y as i32 - epy as i32).abs();
                        if d < best_dist {
                            best_dist = d;
                            best = Some((x, y));
                        }
                    }
                }
            }
            if let Some((fx, fy)) = best {
                let offset_x = fx as i32 - CAVE_SIZE as i32 / 2;
                let offset_y = fy as i32 - CAVE_SIZE as i32 / 2;
                exits.push(CaveExit {
                    tile_x: fx as u8,
                    tile_y: fy as u8,
                    target_world_x: entrance_world_x + offset_x * 2,
                    target_world_y: entrance_world_y + offset_y * 2,
                });
            }
        }

        // Phase 6: decorations
        let mut decorations = Vec::new();
        for y in 1..CAVE_SIZE - 1 {
            for x in 1..CAVE_SIZE - 1 {
                if tiles[y][x] != CaveTile::Floor {
                    continue;
                }
                let near_wall = [
                    tiles[y - 1][x], tiles[y + 1][x],
                    tiles[y][x - 1], tiles[y][x + 1],
                ].iter().any(|&t| t == CaveTile::Wall);

                if near_wall && rng.random_range(0..100) < 3 {
                    decorations.push(DecorationInstance {
                        kind: Decoration::Boulder,
                        tile_x: x as u8,
                        tile_y: y as u8,
                        offset_x: rng.random_range(-3.0..3.0),
                        offset_y: rng.random_range(-3.0..3.0),
                        size: rng.random_range(0.7..1.2),
                        variant: rng.random_range(0..4),
                    });
                } else if rng.random_range(0..100) < 2 {
                    decorations.push(DecorationInstance {
                        kind: Decoration::Rock,
                        tile_x: x as u8,
                        tile_y: y as u8,
                        offset_x: rng.random_range(-4.0..4.0),
                        offset_y: rng.random_range(-4.0..4.0),
                        size: rng.random_range(0.6..1.0),
                        variant: rng.random_range(0..4),
                    });
                } else if rng.random_range(0..100) < 4 {
                    decorations.push(DecorationInstance {
                        kind: Decoration::Pebbles,
                        tile_x: x as u8,
                        tile_y: y as u8,
                        offset_x: rng.random_range(-4.0..4.0),
                        offset_y: rng.random_range(-4.0..4.0),
                        size: rng.random_range(0.5..0.9),
                        variant: rng.random_range(0..3),
                    });
                } else if near_wall && rng.random_range(0..100) < 2 {
                    decorations.push(DecorationInstance {
                        kind: Decoration::Moss,
                        tile_x: x as u8,
                        tile_y: y as u8,
                        offset_x: rng.random_range(-3.0..3.0),
                        offset_y: rng.random_range(-3.0..3.0),
                        size: rng.random_range(0.5..1.0),
                        variant: rng.random_range(0..3),
                    });
                }
            }
        }

        CaveMap {
            tiles,
            decorations,
            exits,
            entrance_x,
            entrance_y,
        }
    }
}
