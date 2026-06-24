use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum WeatherType {
    Normal,
    Mist,
    Rain,
    Heatwave,
}

pub(crate) struct RainDrop {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub(crate) struct WeatherSystem {
    pub(crate) current: WeatherType,
    pub(crate) event_timer: f32,
    pub(crate) cooldown: f32,
    pub(crate) transition: f32,
    pub(crate) fading_out: bool,
    pub(crate) lightning_timer: f32,
    pub(crate) lightning_flash: f32,
    pub(crate) rain_drops: Vec<RainDrop>,
}

pub(crate) const WEATHER_FADE_DURATION: f32 = 10.0;

impl WeatherSystem {
    pub(crate) fn new() -> Self {
        Self {
            current: WeatherType::Normal,
            event_timer: 0.0,
            cooldown: rand::gen_range(60.0, 180.0),
            transition: 0.0,
            fading_out: false,
            lightning_timer: rand::gen_range(5.0, 15.0),
            lightning_flash: 0.0,
            rain_drops: Vec::new(),
        }
    }

    pub(crate) fn update(&mut self, dt: f32, wind: Vec2, day_phase: f32) {
        match self.current {
            WeatherType::Normal => {
                self.transition = 0.0;
                self.cooldown -= dt;
                if self.cooldown <= 0.0 {
                    let is_noon = day_phase > 0.35 && day_phase < 0.65;
                    let is_dim = day_phase < 0.2 || day_phase > 0.7;

                    let mut candidates = vec![WeatherType::Rain];
                    if is_dim {
                        candidates.push(WeatherType::Mist);
                        candidates.push(WeatherType::Mist);
                    }
                    if is_noon {
                        candidates.push(WeatherType::Heatwave);
                        candidates.push(WeatherType::Heatwave);
                    }
                    self.current = candidates[rand::gen_range(0, candidates.len())];
                    self.event_timer = rand::gen_range(120.0, 600.0);
                    self.transition = 0.0;
                    self.fading_out = false;
                    if self.current == WeatherType::Rain {
                        self.rain_drops = (0..150)
                            .map(|_| RainDrop {
                                x: rand::gen_range(0.0, screen_width()),
                                y: rand::gen_range(-screen_height(), screen_height()),
                            })
                            .collect();
                    }
                }
            }
            _ => {
                self.event_timer -= dt;
                if self.event_timer <= WEATHER_FADE_DURATION && !self.fading_out {
                    self.fading_out = true;
                }
                if self.fading_out {
                    self.transition = (self.transition - dt / WEATHER_FADE_DURATION).max(0.0);
                    if self.transition <= 0.0 {
                        self.current = WeatherType::Normal;
                        self.cooldown = rand::gen_range(300.0, 900.0);
                        self.rain_drops.clear();
                        self.fading_out = false;
                    }
                } else {
                    self.transition = (self.transition + dt / WEATHER_FADE_DURATION).min(1.0);
                }
            }
        }

        if self.current == WeatherType::Rain {
            let wind_angle = wind.y.atan2(wind.x);
            let rain_dx = wind_angle.cos() * 80.0;
            for drop in &mut self.rain_drops {
                drop.x += rain_dx * dt;
                drop.y += 500.0 * dt;
                if drop.y > screen_height() {
                    drop.y -= screen_height() + rand::gen_range(0.0, 100.0);
                    drop.x = rand::gen_range(-50.0, screen_width() + 50.0);
                }
            }
            self.lightning_timer -= dt;
            if self.lightning_timer <= 0.0 && self.transition > 0.5 {
                self.lightning_flash = 1.0;
                self.lightning_timer = rand::gen_range(5.0, 15.0);
            }
        }
        self.lightning_flash = (self.lightning_flash - dt * 5.0).max(0.0);
    }

    pub(crate) fn draw(&self, game_time: f32) {
        let t = self.transition;
        if t <= 0.0 {
            return;
        }

        match self.current {
            WeatherType::Normal => {}
            WeatherType::Mist => {
                let sw = screen_width();
                let sh = screen_height();
                let base_alpha = (45.0 * t) as u8;
                draw_rectangle(0.0, 0.0, sw, sh, Color::from_rgba(200, 200, 210, base_alpha));

                let strip_h = 8.0;
                let mut y = 0.0;
                while y < sh {
                    let ny = y / sh;
                    let edge_y = 1.0 - (2.0 * ny - 1.0).abs();
                    let intensity = (1.0 - edge_y) * 0.6;
                    let wave = (ny * 7.0 + game_time * 0.08).sin() * 0.15;
                    let alpha = ((intensity + wave).clamp(0.0, 1.0) * 70.0 * t) as u8;
                    if alpha > 0 {
                        draw_rectangle(0.0, y, sw, strip_h, Color::from_rgba(210, 210, 220, alpha));
                    }
                    y += strip_h;
                }

                let mut x = 0.0;
                while x < sw {
                    let nx = x / sw;
                    let edge_x = 1.0 - (2.0 * nx - 1.0).abs();
                    let intensity = (1.0 - edge_x) * 0.5;
                    let wave = (nx * 5.0 + game_time * 0.06).cos() * 0.1;
                    let alpha = ((intensity + wave).clamp(0.0, 1.0) * 60.0 * t) as u8;
                    if alpha > 0 {
                        draw_rectangle(x, 0.0, strip_h, sh, Color::from_rgba(210, 210, 220, alpha));
                    }
                    x += strip_h;
                }
            }
            WeatherType::Rain => {
                let rain_alpha = (50.0 * t) as u8;
                draw_rectangle(
                    0.0, 0.0,
                    screen_width(), screen_height(),
                    Color::from_rgba(50, 50, 70, rain_alpha),
                );
                let drop_alpha = (140.0 * t) as u8;
                let rain_color = Color::from_rgba(180, 195, 230, drop_alpha);
                let splash_color = Color::from_rgba(170, 190, 220, (80.0 * t) as u8);
                for drop in &self.rain_drops {
                    draw_line(drop.x, drop.y, drop.x + 2.5, drop.y + 12.0, 1.5, rain_color);
                    let splash_y = drop.y + 12.0;
                    if splash_y > 0.0 && splash_y < screen_height() {
                        let splash_phase = (drop.y % 40.0) / 40.0;
                        if splash_phase > 0.85 {
                            let sr = (1.0 - splash_phase) * 20.0 * 3.0;
                            draw_circle(drop.x + 2.0, splash_y, sr, splash_color);
                            draw_line(drop.x - 1.0, splash_y, drop.x - 1.0 - sr, splash_y - sr * 0.8, 0.6, splash_color);
                            draw_line(drop.x + 4.0, splash_y, drop.x + 4.0 + sr, splash_y - sr * 0.6, 0.6, splash_color);
                        }
                    }
                }
                if self.lightning_flash > 0.0 {
                    let flash_alpha = (self.lightning_flash * 200.0 * t) as u8;
                    draw_rectangle(
                        0.0, 0.0,
                        screen_width(), screen_height(),
                        Color::from_rgba(255, 255, 240, flash_alpha),
                    );
                }
            }
            WeatherType::Heatwave => {}
        }
    }
}
