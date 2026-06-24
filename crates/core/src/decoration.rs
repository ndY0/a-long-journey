use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::terrain::Terrain;
use crate::world_vectors::WorldSample;
use crate::CHUNK_SIZE;

pub const DECORATION_COUNT: usize = 25;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Decoration {
    PineTree,
    OakTree,
    BirchTree,
    Bush,
    FlowerBush,
    Rock,
    Boulder,
    MountainPeak,
    Pebbles,
    Flowers,
    Driftwood,
    SnowPile,
    Cactus,
    PalmTree,
    DeadTree,
    TallGrass,
    DryShrub,
    SandDune,
    Reeds,
    Mushroom,
    FlatRock,
    IceChunk,
    Fern,
    Moss,
    Seashell,
}

const ALL_DECORATIONS: [Decoration; DECORATION_COUNT] = [
    Decoration::PineTree,
    Decoration::OakTree,
    Decoration::BirchTree,
    Decoration::Bush,
    Decoration::FlowerBush,
    Decoration::Rock,
    Decoration::Boulder,
    Decoration::MountainPeak,
    Decoration::Pebbles,
    Decoration::Flowers,
    Decoration::Driftwood,
    Decoration::SnowPile,
    Decoration::Cactus,
    Decoration::PalmTree,
    Decoration::DeadTree,
    Decoration::TallGrass,
    Decoration::DryShrub,
    Decoration::SandDune,
    Decoration::Reeds,
    Decoration::Mushroom,
    Decoration::FlatRock,
    Decoration::IceChunk,
    Decoration::Fern,
    Decoration::Moss,
    Decoration::Seashell,
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecorationInstance {
    pub kind: Decoration,
    pub tile_x: u8,
    pub tile_y: u8,
    pub offset_x: f32,
    pub offset_y: f32,
    pub size: f32,
    pub variant: u8,
}

fn affinity(from: Decoration, to: Decoration) -> f32 {
    use Decoration::*;
    match (from, to) {
        (PineTree, PineTree) => 0.25,
        (PineTree, Bush) => 0.3,
        (PineTree, Rock) => 0.15,
        (PineTree, Pebbles) => 0.1,
        (PineTree, Moss) => 0.1,

        (OakTree, OakTree) => 0.2,
        (OakTree, FlowerBush) => 0.25,
        (OakTree, Bush) => 0.2,
        (OakTree, Flowers) => 0.2,
        (OakTree, TallGrass) => 0.15,
        (OakTree, Mushroom) => 0.1,

        (BirchTree, BirchTree) => 0.2,
        (BirchTree, Flowers) => 0.3,
        (BirchTree, FlowerBush) => 0.25,
        (BirchTree, OakTree) => 0.1,
        (BirchTree, TallGrass) => 0.15,

        (Rock, Rock) => 0.2,
        (Rock, Boulder) => 0.25,
        (Rock, Pebbles) => 0.2,
        (Rock, MountainPeak) => 0.15,
        (Rock, FlatRock) => 0.2,

        (Boulder, Rock) => 0.3,
        (Boulder, MountainPeak) => 0.25,
        (Boulder, Pebbles) => 0.15,
        (Boulder, FlatRock) => 0.2,

        (MountainPeak, Rock) => 0.2,
        (MountainPeak, Boulder) => 0.2,
        (MountainPeak, SnowPile) => 0.25,
        (MountainPeak, MountainPeak) => 0.15,
        (MountainPeak, FlatRock) => 0.15,

        (Bush, FlowerBush) => 0.2,
        (Bush, Flowers) => 0.15,
        (Bush, Bush) => 0.1,
        (Bush, TallGrass) => 0.15,

        (FlowerBush, Flowers) => 0.35,
        (FlowerBush, Bush) => 0.15,
        (FlowerBush, FlowerBush) => 0.15,

        (Flowers, FlowerBush) => 0.25,
        (Flowers, Flowers) => 0.1,
        (Flowers, TallGrass) => 0.15,

        (SnowPile, MountainPeak) => 0.15,
        (SnowPile, Rock) => 0.1,
        (SnowPile, SnowPile) => 0.1,
        (SnowPile, IceChunk) => 0.2,

        (Driftwood, Driftwood) => 0.1,
        (Driftwood, Pebbles) => 0.15,
        (Driftwood, Reeds) => 0.15,
        (Driftwood, Seashell) => 0.2,

        (Cactus, Cactus) => 0.15,
        (Cactus, DryShrub) => 0.3,
        (Cactus, Pebbles) => 0.15,
        (Cactus, SandDune) => 0.2,

        (PalmTree, PalmTree) => 0.2,
        (PalmTree, Seashell) => 0.2,
        (PalmTree, Flowers) => 0.15,
        (PalmTree, Fern) => 0.2,

        (DeadTree, DeadTree) => 0.15,
        (DeadTree, Mushroom) => 0.25,
        (DeadTree, Moss) => 0.2,
        (DeadTree, Reeds) => 0.15,

        (TallGrass, TallGrass) => 0.15,
        (TallGrass, Flowers) => 0.2,
        (TallGrass, Bush) => 0.15,
        (TallGrass, Fern) => 0.15,

        (DryShrub, DryShrub) => 0.1,
        (DryShrub, Cactus) => 0.2,
        (DryShrub, Pebbles) => 0.15,
        (DryShrub, Rock) => 0.1,

        (SandDune, SandDune) => 0.2,
        (SandDune, Cactus) => 0.15,
        (SandDune, DryShrub) => 0.15,

        (Reeds, Reeds) => 0.2,
        (Reeds, Driftwood) => 0.15,
        (Reeds, Moss) => 0.2,
        (Reeds, DeadTree) => 0.1,

        (Mushroom, Mushroom) => 0.15,
        (Mushroom, Moss) => 0.25,
        (Mushroom, Fern) => 0.2,
        (Mushroom, DeadTree) => 0.15,

        (FlatRock, FlatRock) => 0.15,
        (FlatRock, Rock) => 0.2,
        (FlatRock, Pebbles) => 0.15,

        (IceChunk, IceChunk) => 0.15,
        (IceChunk, SnowPile) => 0.25,
        (IceChunk, Rock) => 0.1,

        (Fern, Fern) => 0.15,
        (Fern, Moss) => 0.25,
        (Fern, Mushroom) => 0.2,
        (Fern, PalmTree) => 0.15,

        (Moss, Moss) => 0.15,
        (Moss, Mushroom) => 0.2,
        (Moss, Fern) => 0.2,
        (Moss, DeadTree) => 0.15,
        (Moss, Reeds) => 0.1,

        (Seashell, Seashell) => 0.1,
        (Seashell, Driftwood) => 0.2,
        (Seashell, Pebbles) => 0.15,

        _ => 0.0,
    }
}

struct AffinityTracker {
    placed: Vec<(u8, u8, Decoration)>,
    occupied: [[u8; CHUNK_SIZE]; CHUNK_SIZE],
}

impl AffinityTracker {
    fn new() -> Self {
        Self {
            placed: Vec::new(),
            occupied: [[0; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }

    fn boost_for(&self, tx: u8, ty: u8, target: Decoration) -> f32 {
        let mut boost = 0.0f32;
        for &(px, py, source) in &self.placed {
            let dx = tx as f32 - px as f32;
            let dy = ty as f32 - py as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < 6.0 {
                let aff = affinity(source, target);
                if aff > 0.0 {
                    boost += aff * (1.0 - dist / 6.0);
                }
            }
        }
        boost
    }

    fn crowding_penalty(&self, tx: u8, ty: u8) -> f32 {
        let count = self.occupied[ty as usize][tx as usize];
        match count {
            0 => 1.0,
            1 => 0.15,
            _ => 0.0,
        }
    }

    fn record(&mut self, tx: u8, ty: u8, kind: Decoration) {
        self.placed.push((tx, ty, kind));
        self.occupied[ty as usize][tx as usize] =
            self.occupied[ty as usize][tx as usize].saturating_add(1);
    }
}

fn base_spawn_chance(terrain: Terrain, kind: Decoration, ws: &WorldSample) -> f32 {
    use Decoration::*;
    use Terrain::*;

    let density_mult = 0.4 + ws.density as f32 * 0.6;

    let affinity_shift = ws.affinity_value as f32;
    let conifer_bonus = (affinity_shift * 0.3).max(0.0);
    let temperate_bonus = (-affinity_shift * 0.3).max(0.0);
    let moisture = ws.moisture as f32;

    let base = match (terrain, kind) {
        (Forest, PineTree) => 0.016 + conifer_bonus * 0.2,
        (Forest, OakTree) => 0.010 + temperate_bonus * 0.2,
        (Forest, BirchTree) => 0.006 + temperate_bonus * 0.1,
        (Forest, Bush) => 0.008,
        (Forest, Pebbles) => 0.002,
        (Forest, Mushroom) => 0.005,
        (Forest, TallGrass) => 0.004,
        (Forest, Moss) => 0.004,
        (Forest, Fern) => 0.004,

        (Grass, OakTree) => 0.004 + temperate_bonus * 0.1,
        (Grass, BirchTree) => 0.003 + temperate_bonus * 0.06,
        (Grass, PineTree) => 0.002 + conifer_bonus * 0.06,
        (Grass, Bush) => 0.006,
        (Grass, FlowerBush) => 0.004 + (moisture * 0.004).max(0.0),
        (Grass, Flowers) => 0.006 + (moisture * 0.006).max(0.0),
        (Grass, Pebbles) => 0.002,
        (Grass, TallGrass) => 0.008,
        (Grass, Moss) => 0.002,
        (Grass, Mushroom) => 0.0016,
        (Grass, DryShrub) => 0.001,

        (Stone, Rock) => 0.012,
        (Stone, Boulder) => 0.004,
        (Stone, MountainPeak) => 0.003,
        (Stone, Pebbles) => 0.006,
        (Stone, FlatRock) => 0.006,
        (Stone, Moss) => 0.001,

        (Snow, MountainPeak) => 0.004,
        (Snow, SnowPile) => 0.006,
        (Snow, Rock) => 0.004,
        (Snow, PineTree) => 0.0016,
        (Snow, Boulder) => 0.002,
        (Snow, IceChunk) => 0.004,
        (Snow, FlatRock) => 0.002,

        (Sand, Pebbles) => 0.004,
        (Sand, Driftwood) => 0.0016,
        (Sand, Rock) => 0.001,
        (Sand, Cactus) => 0.004,
        (Sand, DryShrub) => 0.003,
        (Sand, SandDune) => 0.004,
        (Sand, PalmTree) => 0.003,
        (Sand, Reeds) => 0.002,
        (Sand, Seashell) => 0.003,
        (Sand, DeadTree) => 0.001,

        _ => 0.0,
    };

    base * density_mult
}

struct GroupSpawn {
    center_x: u8,
    center_y: u8,
    radius: f32,
    primary: Decoration,
    count: usize,
}

fn plan_group_spawns(
    rng: &mut StdRng,
    tiles: &[[Terrain; CHUNK_SIZE]; CHUNK_SIZE],
    ws: &WorldSample,
    biome_mults: &[f32; DECORATION_COUNT],
) -> Vec<GroupSpawn> {
    let mut groups = Vec::new();

    let mut forest_count = 0u32;
    let mut grass_count = 0u32;
    let mut stone_count = 0u32;
    let mut snow_count = 0u32;
    let mut sand_count = 0u32;

    for row in tiles {
        for &tile in row {
            match tile {
                Terrain::Forest => forest_count += 1,
                Terrain::Grass => grass_count += 1,
                Terrain::Stone => stone_count += 1,
                Terrain::Snow => snow_count += 1,
                Terrain::Sand => sand_count += 1,
                _ => {}
            }
        }
    }

    let total = (CHUNK_SIZE * CHUNK_SIZE) as f32;
    let density_mult = 0.5 + ws.density as f32 * 0.5;
    let affinity_shift = ws.affinity_value as f32;

    // Forest groves
    let tree_mult = (biome_mults[Decoration::PineTree as usize]
        + biome_mults[Decoration::OakTree as usize]
        + biome_mults[Decoration::BirchTree as usize]
        + biome_mults[Decoration::PalmTree as usize]) / 4.0;
    if forest_count as f32 / total > 0.15 {
        let grove_chance = 0.2 * density_mult * tree_mult;
        if rng.random_range(0.0..1.0) < grove_chance {
            let palm_mult = biome_mults[Decoration::PalmTree as usize];
            let primary = if palm_mult > 1.5 {
                Decoration::PalmTree
            } else if affinity_shift > 0.2 {
                Decoration::PineTree
            } else if affinity_shift < -0.2 {
                if rng.random_range(0..2) == 0 {
                    Decoration::OakTree
                } else {
                    Decoration::BirchTree
                }
            } else {
                match rng.random_range(0..3) {
                    0 => Decoration::PineTree,
                    1 => Decoration::OakTree,
                    _ => Decoration::BirchTree,
                }
            };

            if let Some((cx, cy)) = find_terrain_center(rng, tiles, Terrain::Forest) {
                groups.push(GroupSpawn {
                    center_x: cx,
                    center_y: cy,
                    radius: rng.random_range(2.5..4.5),
                    primary,
                    count: rng.random_range(2..5),
                });
            }
        }
    }

    // Grass patches (flowers or tall grass)
    if grass_count as f32 / total > 0.2 {
        let grass_mult = (biome_mults[Decoration::TallGrass as usize]
            + biome_mults[Decoration::Flowers as usize]) / 2.0;
        let patch_chance = 0.15 * density_mult * grass_mult * (1.0 + ws.moisture as f32 * 0.5).max(0.3);
        if rng.random_range(0.0..1.0) < patch_chance {
            let primary = if biome_mults[Decoration::TallGrass as usize] > 1.0
                && rng.random_range(0..2) == 0
            {
                Decoration::TallGrass
            } else if rng.random_range(0..3) == 0 {
                Decoration::FlowerBush
            } else {
                Decoration::Flowers
            };
            if let Some((cx, cy)) = find_terrain_center(rng, tiles, Terrain::Grass) {
                groups.push(GroupSpawn {
                    center_x: cx,
                    center_y: cy,
                    radius: rng.random_range(2.0..4.0),
                    primary,
                    count: rng.random_range(2..5),
                });
            }
        }
    }

    // Rock formations
    if stone_count as f32 / total > 0.1 {
        let rock_mult = (biome_mults[Decoration::Rock as usize]
            + biome_mults[Decoration::Boulder as usize]
            + biome_mults[Decoration::FlatRock as usize]) / 3.0;
        let rock_chance = 0.15 * density_mult * rock_mult;
        if rng.random_range(0.0..1.0) < rock_chance {
            let primary = match rng.random_range(0..5) {
                0 => Decoration::Boulder,
                1 => Decoration::FlatRock,
                _ => Decoration::Rock,
            };
            if let Some((cx, cy)) = find_terrain_center(rng, tiles, Terrain::Stone) {
                groups.push(GroupSpawn {
                    center_x: cx,
                    center_y: cy,
                    radius: rng.random_range(2.0..3.5),
                    primary,
                    count: rng.random_range(2..4),
                });
            }
        }
    }

    // Mountain clusters
    if snow_count as f32 / total > 0.08 {
        let mt_mult = biome_mults[Decoration::MountainPeak as usize];
        let mt_chance = 0.12 * density_mult * mt_mult;
        if rng.random_range(0.0..1.0) < mt_chance
            && let Some((cx, cy)) = find_terrain_center(rng, tiles, Terrain::Snow)
        {
            groups.push(GroupSpawn {
                center_x: cx,
                center_y: cy,
                radius: rng.random_range(2.5..4.0),
                primary: Decoration::MountainPeak,
                count: rng.random_range(2..4),
            });
        }
    }

    // Cactus clusters (desert)
    if sand_count as f32 / total > 0.2 {
        let cactus_mult = biome_mults[Decoration::Cactus as usize];
        let cactus_chance = 0.12 * density_mult * cactus_mult;
        if rng.random_range(0.0..1.0) < cactus_chance
            && let Some((cx, cy)) = find_terrain_center(rng, tiles, Terrain::Sand)
        {
            groups.push(GroupSpawn {
                center_x: cx,
                center_y: cy,
                radius: rng.random_range(2.0..4.0),
                primary: Decoration::Cactus,
                count: rng.random_range(2..5),
            });
        }
    }

    groups
}

fn find_terrain_center(
    rng: &mut StdRng,
    tiles: &[[Terrain; CHUNK_SIZE]; CHUNK_SIZE],
    target: Terrain,
) -> Option<(u8, u8)> {
    let mut candidates = Vec::new();
    for (y, row) in tiles.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == target {
                candidates.push((x as u8, y as u8));
            }
        }
    }
    if candidates.is_empty() {
        return None;
    }
    let idx = rng.random_range(0..candidates.len());
    Some(candidates[idx])
}

fn spawn_group(
    rng: &mut StdRng,
    group: &GroupSpawn,
    tiles: &[[Terrain; CHUNK_SIZE]; CHUNK_SIZE],
    tracker: &mut AffinityTracker,
    out: &mut Vec<DecorationInstance>,
) {
    for _ in 0..group.count {
        let angle = rng.random_range(0.0..std::f32::consts::TAU);
        let dist = rng.random_range(0.0..group.radius);
        let tx = (group.center_x as f32 + angle.cos() * dist).round() as i32;
        let ty = (group.center_y as f32 + angle.sin() * dist).round() as i32;

        if tx < 0 || tx >= CHUNK_SIZE as i32 || ty < 0 || ty >= CHUNK_SIZE as i32 {
            continue;
        }

        let tx = tx as u8;
        let ty = ty as u8;
        let terrain = tiles[ty as usize][tx as usize];

        if matches!(terrain, Terrain::DeepWater | Terrain::Water) {
            continue;
        }

        if rng.random_range(0.0..1.0) > tracker.crowding_penalty(tx, ty) {
            continue;
        }

        let kind = if rng.random_range(0.0..1.0) < 0.7 {
            group.primary
        } else {
            pick_affinity_secondary(rng, group.primary)
        };

        let inst = DecorationInstance {
            kind,
            tile_x: tx,
            tile_y: ty,
            offset_x: rng.random_range(-4.0..4.0),
            offset_y: rng.random_range(-4.0..4.0),
            size: rng.random_range(0.8..1.3),
            variant: rng.random_range(0..4),
        };
        tracker.record(tx, ty, kind);
        out.push(inst);
    }
}

fn pick_affinity_secondary(rng: &mut StdRng, primary: Decoration) -> Decoration {
    let mut weights: Vec<(Decoration, f32)> = Vec::new();
    for &d in &ALL_DECORATIONS {
        let a = affinity(primary, d);
        if a > 0.0 {
            weights.push((d, a));
        }
    }
    if weights.is_empty() {
        return primary;
    }
    let total: f32 = weights.iter().map(|(_, w)| w).sum();
    let mut roll = rng.random_range(0.0..total);
    for (d, w) in &weights {
        roll -= w;
        if roll <= 0.0 {
            return *d;
        }
    }
    weights.last().unwrap().0
}

pub fn generate_decorations(
    tiles: &[[Terrain; CHUNK_SIZE]; CHUNK_SIZE],
    chunk_x: i32,
    chunk_y: i32,
    seed: u32,
    ws: &WorldSample,
    biome_mults: &[f32; DECORATION_COUNT],
) -> Vec<DecorationInstance> {
    let chunk_seed = seed
        .wrapping_mul(73856093)
        ^ (chunk_x as u32).wrapping_mul(19349663)
        ^ (chunk_y as u32).wrapping_mul(83492791);
    let mut rng = StdRng::seed_from_u64(chunk_seed as u64);
    let mut out = Vec::new();
    let mut tracker = AffinityTracker::new();

    let groups = plan_group_spawns(&mut rng, tiles, ws, biome_mults);
    for group in &groups {
        spawn_group(&mut rng, group, tiles, &mut tracker, &mut out);
    }

    for (ty, row) in tiles.iter().enumerate() {
        for (tx, &terrain) in row.iter().enumerate() {
            if matches!(terrain, Terrain::DeepWater | Terrain::Water) {
                continue;
            }

            for &kind in &ALL_DECORATIONS {
                let biome_mult = biome_mults[kind as usize];
                if biome_mult <= 0.0 {
                    continue;
                }
                let base = base_spawn_chance(terrain, kind, ws) * biome_mult;
                if base <= 0.0 {
                    continue;
                }

                let crowding = tracker.crowding_penalty(tx as u8, ty as u8);
                if crowding <= 0.0 {
                    continue;
                }
                let aff_boost = tracker.boost_for(tx as u8, ty as u8, kind);
                let chance = ((base + aff_boost) * crowding).min(0.3);

                if rng.random_range(0.0..1.0) < chance {
                    let inst = DecorationInstance {
                        kind,
                        tile_x: tx as u8,
                        tile_y: ty as u8,
                        offset_x: rng.random_range(-4.0..4.0),
                        offset_y: rng.random_range(-4.0..4.0),
                        size: rng.random_range(0.8..1.3),
                        variant: rng.random_range(0..4),
                    };
                    tracker.record(tx as u8, ty as u8, kind);
                    out.push(inst);
                }
            }
        }
    }

    out
}
