use macroquad::prelude::*;

use alj_core::chunk::ChunkPos;
use alj_core::terrain::Terrain;
use alj_core::world::GameWorld;
use alj_core::{CHUNK_SIZE, TILE_SIZE};

const T: f32 = TILE_SIZE;

pub(crate) struct Ripple {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) age: f32,
    pub(crate) max_age: f32,
    pub(crate) max_radius: f32,
}

pub(crate) struct WaveSystem {
    pub(crate) ripples: Vec<Ripple>,
    pub(crate) spawn_accum: f32,
}

impl WaveSystem {
    pub(crate) fn new() -> Self {
        Self {
            ripples: Vec::new(),
            spawn_accum: 0.0,
        }
    }

    pub(crate) fn update(
        &mut self,
        dt: f32,
        world: &GameWorld,
        cam_x: f32,
        cam_y: f32,
        min_cx: i32,
        max_cx: i32,
        min_cy: i32,
        max_cy: i32,
    ) {
        for r in &mut self.ripples {
            r.age += dt;
        }
        self.ripples.retain(|r| r.age < r.max_age);

        self.spawn_accum += dt * rand::gen_range(2.0, 5.0);
        while self.spawn_accum >= 1.0 {
            self.spawn_accum -= 1.0;
            if let Some((wx, wy)) = pick_random_water_tile(world, min_cx, max_cx, min_cy, max_cy)
            {
                let screen_x = wx - cam_x;
                let screen_y = wy - cam_y;
                if screen_x > -T && screen_x < screen_width() + T
                    && screen_y > -T && screen_y < screen_height() + T
                {
                    self.ripples.push(Ripple {
                        x: wx + rand::gen_range(0.0, T),
                        y: wy + rand::gen_range(0.0, T),
                        age: 0.0,
                        max_age: rand::gen_range(1.0, 3.0),
                        max_radius: rand::gen_range(T * 0.2, T * 0.6),
                    });
                }
            }
        }
    }

    pub(crate) fn draw(&self, cam_x: f32, cam_y: f32) {
        for r in &self.ripples {
            let progress = r.age / r.max_age;
            let radius = r.max_radius * progress;
            let alpha = ((1.0 - progress) * 120.0) as u8;
            let sx = r.x - cam_x;
            let sy = r.y - cam_y;
            draw_circle_lines(sx, sy, radius, 0.8, Color::from_rgba(200, 220, 250, alpha));
            if radius > T * 0.15 {
                draw_circle_lines(
                    sx,
                    sy,
                    radius * 0.6,
                    0.5,
                    Color::from_rgba(180, 210, 240, alpha / 2),
                );
            }
        }
    }
}

pub(crate) fn pick_random_water_tile(
    world: &GameWorld,
    min_cx: i32,
    max_cx: i32,
    min_cy: i32,
    max_cy: i32,
) -> Option<(f32, f32)> {
    let cx = rand::gen_range(min_cx, max_cx + 1);
    let cy = rand::gen_range(min_cy, max_cy + 1);
    let chunk = world.chunks.get(&ChunkPos::new(cx, cy))?;
    let tx = rand::gen_range(0, CHUNK_SIZE);
    let ty = rand::gen_range(0, CHUNK_SIZE);
    if matches!(chunk.tiles[ty][tx], Terrain::Water | Terrain::DeepWater) {
        let wx = (cx as f32 * CHUNK_SIZE as f32 + tx as f32) * T;
        let wy = (cy as f32 * CHUNK_SIZE as f32 + ty as f32) * T;
        Some((wx, wy))
    } else {
        None
    }
}
