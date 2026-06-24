use alj_core::chunk::ChunkPos;
use alj_core::world::GameWorld;
use alj_core::CHUNK_SIZE;

pub(crate) fn is_walkable(world: &GameWorld, tile_x: i32, tile_y: i32) -> bool {
    let chunk_x = tile_x.div_euclid(CHUNK_SIZE as i32);
    let chunk_y = tile_y.div_euclid(CHUNK_SIZE as i32);
    let local_x = tile_x.rem_euclid(CHUNK_SIZE as i32) as usize;
    let local_y = tile_y.rem_euclid(CHUNK_SIZE as i32) as usize;

    match world.chunks.get(&ChunkPos::new(chunk_x, chunk_y)) {
        Some(chunk) => chunk.tiles[local_y][local_x].walkable(),
        None => false,
    }
}
