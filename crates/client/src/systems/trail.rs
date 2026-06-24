use macroquad::prelude::*;

use alj_core::TILE_SIZE;

const T: f32 = TILE_SIZE;

pub(crate) struct TrailPoint {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) age: f32,
}

pub(crate) struct PlayerTrail {
    pub(crate) points: Vec<TrailPoint>,
    pub(crate) sample_timer: f32,
}

impl PlayerTrail {
    pub(crate) fn new() -> Self {
        Self {
            points: Vec::new(),
            sample_timer: 0.0,
        }
    }

    pub(crate) fn update(&mut self, dt: f32, px: f32, py: f32, moving: bool) {
        for p in &mut self.points {
            p.age += dt;
        }
        self.points.retain(|p| p.age < 2.0);

        if moving {
            self.sample_timer -= dt;
            if self.sample_timer <= 0.0 {
                self.points.push(TrailPoint { x: px, y: py, age: 0.0 });
                self.sample_timer = 0.08;
            }
        }
    }

    pub(crate) fn displacement_at(&self, wx: f32, wy: f32) -> (f32, f32) {
        let mut dx = 0.0f32;
        let mut dy = 0.0f32;
        for p in &self.points {
            let to_x = wx - p.x;
            let to_y = wy - p.y;
            let dist = (to_x * to_x + to_y * to_y).sqrt();
            if dist < T * 2.0 && dist > 0.1 {
                let fade = (1.0 - p.age / 2.0).max(0.0);
                let strength = (1.0 - dist / (T * 2.0)) * fade * T * 0.3;
                dx += (to_x / dist) * strength;
                dy += (to_y / dist) * strength;
            }
        }
        (dx, dy)
    }
}
