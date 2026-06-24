use serde::{Deserialize, Serialize};

use crate::world_vectors::{BlockSample, WorldVectors};
use crate::CHUNK_SIZE;

pub const BLOCK_SIZE: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
}

impl BlockPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_chunk(cx: i32, cy: i32) -> Self {
        Self {
            x: cx.div_euclid(BLOCK_SIZE as i32),
            y: cy.div_euclid(BLOCK_SIZE as i32),
        }
    }

    pub fn from_world(wx: f32, wy: f32, tile_size: f32) -> Self {
        let tiles_per_block = (BLOCK_SIZE * CHUNK_SIZE) as f32 * tile_size;
        Self {
            x: (wx / tiles_per_block).floor() as i32,
            y: (wy / tiles_per_block).floor() as i32,
        }
    }
}

pub struct Block {
    pub pos: BlockPos,
    pub center_sample: BlockSample,
}

impl Block {
    pub fn generate(pos: BlockPos, vectors: &WorldVectors) -> Self {
        let tiles_per_block = (BLOCK_SIZE * CHUNK_SIZE) as f64;
        let cx = pos.x as f64 * tiles_per_block + tiles_per_block * 0.5;
        let cy = pos.y as f64 * tiles_per_block + tiles_per_block * 0.5;
        let center_sample = vectors.sample_tile(cx, cy);
        Self { pos, center_sample }
    }
}
