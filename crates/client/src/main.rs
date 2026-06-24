use macroquad::prelude::*;

use alj_core::chunk::ChunkPos;
use alj_core::world::GameWorld;
use alj_core::{CHUNK_SIZE, TILE_SIZE};

const PLAYER_SPEED: f32 = 120.0;
const LOAD_RADIUS: i32 = 3;

fn window_conf() -> Conf {
    Conf {
        window_title: "A Long Journey".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = GameWorld::new(42);
    let mut player_x: f32 = 0.0;
    let mut player_y: f32 = 0.0;

    loop {
        let dt = get_frame_time();

        // Input
        let mut dx: f32 = 0.0;
        let mut dy: f32 = 0.0;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dy -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dy += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dx -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dx += 1.0;
        }

        if dx != 0.0 || dy != 0.0 {
            let len = (dx * dx + dy * dy).sqrt();
            dx /= len;
            dy /= len;
        }

        let new_x = player_x + dx * PLAYER_SPEED * dt;
        let new_y = player_y + dy * PLAYER_SPEED * dt;

        // Collision: check target tile is walkable
        let tile_x = ((new_x + TILE_SIZE * 0.5) / TILE_SIZE).floor() as i32;
        let tile_y = ((new_y + TILE_SIZE * 0.5) / TILE_SIZE).floor() as i32;
        if is_walkable(&world, tile_x, tile_y) {
            player_x = new_x;
            player_y = new_y;
        }

        // Load chunks around player
        let player_chunk = ChunkPos::from_world(player_x, player_y, TILE_SIZE);
        world.ensure_chunks_around(player_chunk, LOAD_RADIUS);

        // Camera centered on player
        let cam_x = player_x - screen_width() * 0.5;
        let cam_y = player_y - screen_height() * 0.5;

        clear_background(BLACK);

        // Render visible chunks
        let cs = CHUNK_SIZE as f32 * TILE_SIZE;
        let min_cx = ((cam_x) / cs).floor() as i32;
        let max_cx = ((cam_x + screen_width()) / cs).floor() as i32;
        let min_cy = ((cam_y) / cs).floor() as i32;
        let max_cy = ((cam_y + screen_height()) / cs).floor() as i32;

        for cy in min_cy..=max_cy {
            for cx in min_cx..=max_cx {
                let cpos = ChunkPos::new(cx, cy);
                if let Some(chunk) = world.chunks.get(&cpos) {
                    for ty in 0..CHUNK_SIZE {
                        for tx in 0..CHUNK_SIZE {
                            let wx = (cx as f32 * CHUNK_SIZE as f32 + tx as f32) * TILE_SIZE
                                - cam_x;
                            let wy = (cy as f32 * CHUNK_SIZE as f32 + ty as f32) * TILE_SIZE
                                - cam_y;

                            // Skip tiles outside viewport
                            if wx + TILE_SIZE < 0.0
                                || wx > screen_width()
                                || wy + TILE_SIZE < 0.0
                                || wy > screen_height()
                            {
                                continue;
                            }

                            let [r, g, b, a] = chunk.tiles[ty][tx].color();
                            draw_rectangle(
                                wx,
                                wy,
                                TILE_SIZE,
                                TILE_SIZE,
                                Color::from_rgba(r, g, b, a),
                            );
                        }
                    }
                }
            }
        }

        // Draw player
        let px = player_x - cam_x;
        let py = player_y - cam_y;
        draw_rectangle(px, py, TILE_SIZE, TILE_SIZE, YELLOW);

        // HUD
        let chunk = ChunkPos::from_world(player_x, player_y, TILE_SIZE);
        draw_text(
            format!(
                "FPS: {} | Chunk: ({}, {}) | Chunks loaded: {}",
                get_fps(),
                chunk.x,
                chunk.y,
                world.chunks.len()
            )
            .as_str(),
            8.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}

fn is_walkable(world: &GameWorld, tile_x: i32, tile_y: i32) -> bool {
    let chunk_x = tile_x.div_euclid(CHUNK_SIZE as i32);
    let chunk_y = tile_y.div_euclid(CHUNK_SIZE as i32);
    let local_x = tile_x.rem_euclid(CHUNK_SIZE as i32) as usize;
    let local_y = tile_y.rem_euclid(CHUNK_SIZE as i32) as usize;

    match world.chunks.get(&ChunkPos::new(chunk_x, chunk_y)) {
        Some(chunk) => chunk.tiles[local_y][local_x].walkable(),
        None => false,
    }
}
