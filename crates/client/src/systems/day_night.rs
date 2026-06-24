use macroquad::prelude::*;

pub(crate) const DAY_CYCLE_DURATION: f32 = 1200.0;

pub(crate) struct DayNightCycle {
    pub(crate) time: f32,
}

impl DayNightCycle {
    pub(crate) fn new() -> Self {
        Self { time: DAY_CYCLE_DURATION * 0.25 }
    }

    pub(crate) fn update(&mut self, dt: f32) {
        self.time = (self.time + dt) % DAY_CYCLE_DURATION;
    }

    pub(crate) fn daylight(&self) -> f32 {
        let phase = self.time / DAY_CYCLE_DURATION;
        (phase * std::f32::consts::TAU).sin() * 0.5 + 0.5
    }

    pub(crate) fn time_label(&self) -> &'static str {
        let phase = self.time / DAY_CYCLE_DURATION;
        match phase {
            p if p < 0.15 => "Night",
            p if p < 0.3 => "Dawn",
            p if p < 0.45 => "Morning",
            p if p < 0.55 => "Noon",
            p if p < 0.7 => "Afternoon",
            p if p < 0.8 => "Dusk",
            _ => "Night",
        }
    }

    pub(crate) fn draw(&self) {
        let daylight = self.daylight();
        let darkness = 1.0 - daylight;

        let night_alpha = (darkness * 180.0) as u8;
        if night_alpha > 0 {
            draw_rectangle(
                0.0, 0.0,
                screen_width(), screen_height(),
                Color::from_rgba(10, 10, 30, night_alpha),
            );
            let desat_alpha = (darkness * 60.0) as u8;
            draw_rectangle(
                0.0, 0.0,
                screen_width(), screen_height(),
                Color::from_rgba(128, 128, 128, desat_alpha),
            );
        }

        let phase = self.time / DAY_CYCLE_DURATION;
        let dawn = ((1.0 - ((phase - 0.25).abs() * 8.0).min(1.0)) * 30.0).max(0.0) as u8;
        let dusk = ((1.0 - ((phase - 0.75).abs() * 8.0).min(1.0)) * 30.0).max(0.0) as u8;
        let warm = dawn.max(dusk);
        if warm > 0 {
            draw_rectangle(
                0.0, 0.0,
                screen_width(), screen_height(),
                Color::from_rgba(180, 100, 50, warm),
            );
        }
    }
}
