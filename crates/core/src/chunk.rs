use noise::{NoiseFn, Perlin};
use serde::{Deserialize, Serialize};

use crate::terrain::Terrain;
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
}

impl Chunk {
    pub fn generate(pos: ChunkPos, seed: u32) -> Self {
        let perlin = Perlin::new(seed);
        let mut tiles = [[Terrain::Grass; CHUNK_SIZE]; CHUNK_SIZE];

        let base_x = pos.x as f64 * CHUNK_SIZE as f64;
        let base_y = pos.y as f64 * CHUNK_SIZE as f64;

        for (y, row) in tiles.iter_mut().enumerate() {
            for (x, tile) in row.iter_mut().enumerate() {
                let wx = base_x + x as f64;
                let wy = base_y + y as f64;

                let scale1 = 0.02;
                let scale2 = 0.05;
                let scale3 = 0.1;

                let h = perlin.get([wx * scale1, wy * scale1]) * 0.6
                    + perlin.get([wx * scale2, wy * scale2]) * 0.3
                    + perlin.get([wx * scale3, wy * scale3]) * 0.1;

                *tile = Terrain::from_height(h);
            }
        }

        Self { pos, tiles }
    }
}
