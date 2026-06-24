use std::collections::HashMap;

use crate::chunk::{Chunk, ChunkPos};

pub struct GameWorld {
    pub seed: u32,
    pub chunks: HashMap<ChunkPos, Chunk>,
}

impl GameWorld {
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            chunks: HashMap::new(),
        }
    }

    pub fn ensure_chunk(&mut self, pos: ChunkPos) -> &Chunk {
        self.chunks
            .entry(pos)
            .or_insert_with(|| Chunk::generate(pos, self.seed))
    }

    pub fn ensure_chunks_around(&mut self, center: ChunkPos, radius: i32) {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let pos = ChunkPos::new(center.x + dx, center.y + dy);
                self.ensure_chunk(pos);
            }
        }
    }
}
