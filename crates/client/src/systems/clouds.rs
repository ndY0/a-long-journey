use macroquad::prelude::*;

use alj_core::TILE_SIZE;

const T: f32 = TILE_SIZE;

pub(crate) struct Cloud {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) opacity: f32,
}

pub(crate) struct CloudSystem {
    pub(crate) clouds: Vec<Cloud>,
    pub(crate) wind: Vec2,
    pub(crate) target_wind: Vec2,
    pub(crate) wind_timer: f32,
    pub(crate) spawn_timer: f32,
}

impl CloudSystem {
    pub(crate) fn new() -> Self {
        Self {
            clouds: Vec::new(),
            wind: Vec2::new(20.0, 8.0),
            target_wind: Vec2::new(20.0, 8.0),
            wind_timer: 20.0,
            spawn_timer: 0.0,
        }
    }

    pub(crate) fn update(&mut self, dt: f32, cam_x: f32, cam_y: f32) {
        self.wind_timer -= dt;
        if self.wind_timer <= 0.0 {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(15.0, 40.0);
            self.target_wind = Vec2::new(angle.cos() * speed, angle.sin() * speed);
            self.wind_timer = rand::gen_range(15.0, 30.0);
        }
        self.wind = self.wind + (self.target_wind - self.wind) * (dt * 0.3);

        for cloud in &mut self.clouds {
            cloud.x += self.wind.x * dt;
            cloud.y += self.wind.y * dt;
        }

        let margin = 800.0;
        self.clouds.retain(|c| {
            c.x > cam_x - margin
                && c.x < cam_x + screen_width() + margin
                && c.y > cam_y - margin
                && c.y < cam_y + screen_height() + margin
        });

        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 && self.clouds.len() < 10 {
            let upwind_x = if self.wind.x > 0.0 {
                cam_x - rand::gen_range(100.0, 300.0)
            } else {
                cam_x + screen_width() + rand::gen_range(100.0, 300.0)
            };
            let spawn_y = cam_y + rand::gen_range(-100.0, screen_height() + 100.0);
            self.clouds.push(Cloud {
                x: upwind_x,
                y: spawn_y,
                width: rand::gen_range(T * 4.0, T * 10.0),
                height: rand::gen_range(T * 2.0, T * 5.0),
                opacity: rand::gen_range(0.08, 0.2),
            });
            self.spawn_timer = rand::gen_range(1.5, 4.0);
        }
    }

    pub(crate) fn draw(&self, cam_x: f32, cam_y: f32) {
        for cloud in &self.clouds {
            let sx = cloud.x - cam_x;
            let sy = cloud.y - cam_y;
            let shadow = Color::from_rgba(0, 0, 0, (cloud.opacity * 80.0) as u8);
            draw_ellipse(sx + 4.0, sy + 6.0, cloud.width * 0.5, cloud.height * 0.35, 0.0, shadow);

            let a = (cloud.opacity * 255.0) as u8;
            let c = Color::from_rgba(240, 240, 245, a);
            let c2 = Color::from_rgba(250, 250, 255, (a as f32 * 0.7) as u8);
            draw_ellipse(sx, sy, cloud.width * 0.5, cloud.height * 0.35, 0.0, c);
            draw_ellipse(
                sx - cloud.width * 0.2,
                sy - cloud.height * 0.08,
                cloud.width * 0.35,
                cloud.height * 0.3,
                0.0,
                c2,
            );
            draw_ellipse(
                sx + cloud.width * 0.2,
                sy + cloud.height * 0.05,
                cloud.width * 0.3,
                cloud.height * 0.25,
                0.0,
                c2,
            );
        }
    }
}
