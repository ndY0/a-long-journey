use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::block::{Block, BlockPos};
use crate::chunk::{Chunk, ChunkPos};
use crate::world_vectors::WorldVectors;

pub struct GameWorld {
    pub seed: u32,
    pub chunks: HashMap<ChunkPos, Chunk>,
    pub blocks: HashMap<BlockPos, Block>,
    pub vectors: WorldVectors,
    request_tx: Sender<ChunkPos>,
    result_rx: Receiver<Chunk>,
    pending: HashSet<ChunkPos>,
}

impl GameWorld {
    pub fn new(seed: u32) -> Self {
        let (request_tx, request_rx) = mpsc::channel::<ChunkPos>();
        let (result_tx, result_rx) = mpsc::channel::<Chunk>();

        thread::spawn(move || {
            let vectors = WorldVectors::new(seed);
            for pos in request_rx {
                let chunk = Chunk::generate(pos, seed, &vectors);
                if result_tx.send(chunk).is_err() {
                    break;
                }
            }
        });

        let vectors = WorldVectors::new(seed);
        let mut chunks = HashMap::new();

        // Generate spawn area synchronously
        for dy in -1..=1 {
            for dx in -1..=1 {
                let pos = ChunkPos::new(dx, dy);
                chunks.insert(pos, Chunk::generate(pos, seed, &vectors));
            }
        }

        Self {
            seed,
            chunks,
            blocks: HashMap::new(),
            vectors,
            request_tx,
            result_rx,
            pending: HashSet::new(),
        }
    }

    pub fn request_chunks_around(&mut self, center: ChunkPos, radius: i32) {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let pos = ChunkPos::new(center.x + dx, center.y + dy);
                if !self.chunks.contains_key(&pos) && self.pending.insert(pos) {
                    let _ = self.request_tx.send(pos);
                }
            }
        }
    }

    pub fn poll_chunks(&mut self) {
        while let Ok(chunk) = self.result_rx.try_recv() {
            self.pending.remove(&chunk.pos);
            self.chunks.insert(chunk.pos, chunk);
        }
    }

    pub fn ensure_block(&mut self, pos: BlockPos) -> &Block {
        let vectors = &self.vectors;
        self.blocks
            .entry(pos)
            .or_insert_with(|| Block::generate(pos, vectors))
    }

    pub fn ensure_chunk(&mut self, pos: ChunkPos) -> &Chunk {
        let seed = self.seed;
        let vectors = &self.vectors;
        self.chunks
            .entry(pos)
            .or_insert_with(|| Chunk::generate(pos, seed, vectors))
    }

    pub fn ensure_chunks_around(&mut self, center: ChunkPos, radius: i32) {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let pos = ChunkPos::new(center.x + dx, center.y + dy);
                let seed = self.seed;
                let vectors = &self.vectors;
                self.chunks
                    .entry(pos)
                    .or_insert_with(|| Chunk::generate(pos, seed, vectors));
            }
        }
    }
}
