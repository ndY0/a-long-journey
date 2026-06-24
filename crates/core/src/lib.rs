pub mod chunk;
pub mod terrain;
pub mod world;

pub use hecs;
pub use glam;
pub use noise;
pub use rand;

pub const CHUNK_SIZE: usize = 16;
pub const TILE_SIZE: f32 = 16.0;
