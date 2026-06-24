use noise::{NoiseFn, Perlin};
use serde::{Deserialize, Serialize};

use crate::cave::{self, CaveEntrance};
use crate::decoration::{self, DecorationInstance};
use crate::terrain::Terrain;
use crate::world_vectors::WorldVectors;
use crate::CHUNK_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

impl ChunkPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_world(wx: f32, wy: f32, tile_size: f32) -> Self {
        let cs = (CHUNK_SIZE as f32) * tile_size;
        Self {
            x: (wx / cs).floor() as i32,
            y: (wy / cs).floor() as i32,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub pos: ChunkPos,
    pub tiles: [[Terrain; CHUNK_SIZE]; CHUNK_SIZE],
    pub tile_colors: [[[u8; 4]; CHUNK_SIZE]; CHUNK_SIZE],
    pub decorations: Vec<DecorationInstance>,
    pub entrances: Vec<CaveEntrance>,
}

impl Chunk {
    pub fn generate(pos: ChunkPos, seed: u32, vectors: &WorldVectors) -> Self {
        let perlin = Perlin::new(seed);
        let mut tiles = [[Terrain::Grass; CHUNK_SIZE]; CHUNK_SIZE];
        let mut tile_colors = [[[0u8; 4]; CHUNK_SIZE]; CHUNK_SIZE];

        let base_x = pos.x as f64 * CHUNK_SIZE as f64;
        let base_y = pos.y as f64 * CHUNK_SIZE as f64;

        for (y, row) in tiles.iter_mut().enumerate() {
            for (x, tile) in row.iter_mut().enumerate() {
                let wx = base_x + x as f64;
                let wy = base_y + y as f64;

                let bs = vectors.sample_tile(wx, wy);
                let biome = vectors.sample_biome(wx, wy);

                let effective_roughness = bs.roughness * biome.roughness_mult;

                let scale1 = 0.02;
                let scale2 = 0.05;
                let scale3 = 0.1;

                let raw_h = perlin.get([wx * scale1, wy * scale1]) * 0.6
                    + perlin.get([wx * scale2, wy * scale2]) * 0.3 * effective_roughness
                    + perlin.get([wx * scale3, wy * scale3]) * 0.1 * effective_roughness;

                let h = raw_h + bs.height_bias + biome.height_bias + bs.dither;

                *tile = biome.terrain_from_height(h);
                tile_colors[y][x] = biome.color(*tile);
            }
        }

        let ws = vectors.sample_chunk_center(pos.x, pos.y);
        let chunk_center_x = base_x + CHUNK_SIZE as f64 * 0.5;
        let chunk_center_y = base_y + CHUNK_SIZE as f64 * 0.5;
        let chunk_biome = vectors.sample_biome(chunk_center_x, chunk_center_y);
        let decorations = decoration::generate_decorations(
            &tiles,
            pos.x,
            pos.y,
            seed,
            &ws,
            &chunk_biome.decoration_mults,
        );

        let mut entrances = Vec::new();
        if cave::should_spawn_entrance(seed, pos.x, pos.y) {
            if let Some((tx, ty)) = cave::pick_entrance_tile(seed, pos.x, pos.y, &tiles) {
                entrances.push(CaveEntrance {
                    tile_x: tx,
                    tile_y: ty,
                    world_x: pos.x * CHUNK_SIZE as i32 + tx as i32,
                    world_y: pos.y * CHUNK_SIZE as i32 + ty as i32,
                });
            }
        }

        Self {
            pos,
            tiles,
            tile_colors,
            decorations,
            entrances,
        }
    }
}
