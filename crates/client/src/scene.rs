use macroquad::prelude::*;

const FADE_DURATION: f32 = 0.5;

#[derive(Clone)]
pub enum SceneState {
    Overworld,
    Cave {
        seed: u32,
        entrance_x: i32,
        entrance_y: i32,
    },
}

enum TransitionPhase {
    None,
    FadingOut {
        target: SceneState,
        timer: f32,
    },
    FadingIn {
        timer: f32,
    },
}

pub struct SceneTransition {
    phase: TransitionPhase,
}

impl SceneTransition {
    pub fn new() -> Self {
        Self {
            phase: TransitionPhase::None,
        }
    }

    pub fn start(&mut self, target: SceneState) {
        self.phase = TransitionPhase::FadingOut {
            target,
            timer: FADE_DURATION,
        };
    }

    pub fn update(&mut self, dt: f32) -> Option<SceneState> {
        match &mut self.phase {
            TransitionPhase::None => None,
            TransitionPhase::FadingOut { target, timer } => {
                *timer -= dt;
                if *timer <= 0.0 {
                    let result = target.clone();
                    self.phase = TransitionPhase::FadingIn {
                        timer: FADE_DURATION,
                    };
                    Some(result)
                } else {
                    None
                }
            }
            TransitionPhase::FadingIn { timer } => {
                *timer -= dt;
                if *timer <= 0.0 {
                    self.phase = TransitionPhase::None;
                }
                None
            }
        }
    }

    pub fn is_active(&self) -> bool {
        !matches!(self.phase, TransitionPhase::None)
    }

    pub fn draw(&self) {
        let alpha = match &self.phase {
            TransitionPhase::None => return,
            TransitionPhase::FadingOut { timer, .. } => {
                ((1.0 - *timer / FADE_DURATION) * 255.0) as u8
            }
            TransitionPhase::FadingIn { timer } => {
                (*timer / FADE_DURATION * 255.0) as u8
            }
        };
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 0, 0, alpha),
        );
    }
}
