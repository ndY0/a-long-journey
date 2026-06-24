mod input;
mod rendering;
mod scene;
mod systems;

use std::collections::HashMap;

use macroquad::prelude::*;

use alj_core::cave::{CaveMap, CaveTile, CAVE_SIZE};
use alj_core::chunk::ChunkPos;
use alj_core::decoration::Decoration;
use alj_core::world::GameWorld;
use alj_core::{CHUNK_SIZE, TILE_SIZE};

use input::is_walkable;
use rendering::cave_renderer::bake_cave;
use rendering::chunk_renderer::bake_chunk;
use rendering::decorations::draw_decoration;
use rendering::shaders::{HEAT_FRAGMENT, HEAT_VERTEX};
use scene::{SceneState, SceneTransition};
use systems::{
    CloudSystem, DayNightCycle, PlayerTrail, WaveSystem, WeatherSystem, WeatherType,
    DAY_CYCLE_DURATION,
};

const PLAYER_SPEED: f32 = 120.0;
const LOAD_RADIUS: i32 = 3;
const T: f32 = TILE_SIZE;

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
    let mut chunk_cache: HashMap<ChunkPos, RenderTarget> = HashMap::new();
    let mut clouds = CloudSystem::new();
    let mut trail = PlayerTrail::new();
    let mut waves = WaveSystem::new();
    let mut day_night = DayNightCycle::new();
    let mut weather = WeatherSystem::new();
    let mut game_time: f32 = 0.0;

    let heat_material = load_material(
        ShaderSource::Glsl {
            vertex: HEAT_VERTEX,
            fragment: HEAT_FRAGMENT,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("time", UniformType::Float1),
                UniformDesc::new("intensity", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .expect("failed to load heat shader");

    let mut scene_rt: Option<RenderTarget> = None;
    let mut scene_rt_size: (u32, u32) = (0, 0);

    let mut current_scene = SceneState::Overworld;
    let mut transition = SceneTransition::new();
    let mut cave_state: Option<(CaveMap, RenderTarget)> = None;
    let mut overworld_return_pos: Option<(f32, f32)> = None;

    loop {
        let dt = get_frame_time();
        game_time += dt;

        // Scene transition logic
        if let Some(new_scene) = transition.update(dt) {
            match new_scene {
                SceneState::Overworld => {
                    cave_state = None;
                    current_scene = SceneState::Overworld;
                    if let Some((rx, ry)) = overworld_return_pos.take() {
                        player_x = rx;
                        player_y = ry;
                    }
                }
                SceneState::Cave {
                    seed,
                    entrance_x,
                    entrance_y,
                } => {
                    let cave = CaveMap::generate(seed, entrance_x, entrance_y);
                    let rt = bake_cave(&cave);
                    player_x = cave.entrance_x as f32 * T;
                    player_y = cave.entrance_y as f32 * T;
                    cave_state = Some((cave, rt));
                    current_scene = SceneState::Cave {
                        seed,
                        entrance_x,
                        entrance_y,
                    };
                }
            }
        }

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

        match &current_scene {
            SceneState::Overworld => {
                // Movement
                let new_x = player_x + dx * PLAYER_SPEED * dt;
                let new_y = player_y + dy * PLAYER_SPEED * dt;
                let tile_x = ((new_x + T * 0.5) / T).floor() as i32;
                let tile_y = ((new_y + T * 0.5) / T).floor() as i32;
                if is_walkable(&world, tile_x, tile_y) {
                    player_x = new_x;
                    player_y = new_y;
                }
                let player_moving = dx != 0.0 || dy != 0.0;

                // Cave entrance interaction
                if is_key_pressed(KeyCode::E) && !transition.is_active() {
                    if let Some(entrance) = find_nearby_entrance(&world, player_x, player_y) {
                        overworld_return_pos = Some((player_x, player_y));
                        transition.start(SceneState::Cave {
                            seed: world.seed,
                            entrance_x: entrance.0,
                            entrance_y: entrance.1,
                        });
                    }
                }

                // Systems
                let player_chunk = ChunkPos::from_world(player_x, player_y, T);
                world.request_chunks_around(player_chunk, LOAD_RADIUS);
                world.poll_chunks();

                let cam_x = player_x - screen_width() * 0.5;
                let cam_y = player_y - screen_height() * 0.5;

                clouds.update(dt, cam_x, cam_y);
                trail.update(dt, player_x, player_y, player_moving);
                day_night.update(dt);
                let day_phase = day_night.time / DAY_CYCLE_DURATION;
                weather.update(dt, clouds.wind, day_phase);

                let cs = CHUNK_SIZE as f32 * T;
                let min_cx = (cam_x / cs).floor() as i32;
                let max_cx = ((cam_x + screen_width()) / cs).floor() as i32;
                let min_cy = (cam_y / cs).floor() as i32;
                let max_cy = ((cam_y + screen_height()) / cs).floor() as i32;

                waves.update(dt, &world, cam_x, cam_y, min_cx, max_cx, min_cy, max_cy);

                // Chunk baking
                for cy in min_cy..=max_cy {
                    for cx in min_cx..=max_cx {
                        let cpos = ChunkPos::new(cx, cy);
                        if world.chunks.contains_key(&cpos) && !chunk_cache.contains_key(&cpos) {
                            let chunk = &world.chunks[&cpos];
                            let chunks_ref = &world.chunks;
                            let rt = bake_chunk(chunk, |wtx, wty| {
                                let ccx = wtx.div_euclid(CHUNK_SIZE as i32);
                                let ccy = wty.div_euclid(CHUNK_SIZE as i32);
                                let lx = wtx.rem_euclid(CHUNK_SIZE as i32) as usize;
                                let ly = wty.rem_euclid(CHUNK_SIZE as i32) as usize;
                                chunks_ref
                                    .get(&ChunkPos::new(ccx, ccy))
                                    .map(|c| (c.tiles[ly][lx], c.tile_colors[ly][lx]))
                            });
                            chunk_cache.insert(cpos, rt);
                        }
                    }
                }

                // Heatwave RT
                let heat_active =
                    weather.current == WeatherType::Heatwave && weather.transition > 0.0;
                if heat_active {
                    let sw = screen_width() as u32;
                    let sh = screen_height() as u32;
                    if scene_rt.is_none() || scene_rt_size != (sw, sh) {
                        scene_rt = Some(render_target(sw, sh));
                        scene_rt
                            .as_ref()
                            .unwrap()
                            .texture
                            .set_filter(FilterMode::Linear);
                        scene_rt_size = (sw, sh);
                    }
                    let srt = scene_rt.as_ref().unwrap();
                    set_camera(&Camera2D {
                        render_target: Some(srt.clone()),
                        zoom: vec2(2.0 / sw as f32, -2.0 / sh as f32),
                        target: vec2(sw as f32 / 2.0, sh as f32 / 2.0),
                        ..Default::default()
                    });
                }

                // Draw scene
                clear_background(BLACK);

                for cy in min_cy..=max_cy {
                    for cx in min_cx..=max_cx {
                        let cpos = ChunkPos::new(cx, cy);
                        if let Some(rt) = chunk_cache.get(&cpos) {
                            let wx = cx as f32 * cs - cam_x;
                            let wy = cy as f32 * cs - cam_y;
                            draw_texture_ex(
                                &rt.texture,
                                wx,
                                wy,
                                WHITE,
                                DrawTextureParams {
                                    dest_size: Some(Vec2::new(cs, cs)),
                                    flip_y: true,
                                    ..Default::default()
                                },
                            );
                        }
                    }
                }

                waves.draw(cam_x, cam_y);

                // Grass interaction
                if !trail.points.is_empty() {
                    let interact_radius = T * 3.0;
                    for cy in min_cy..=max_cy {
                        for cx in min_cx..=max_cx {
                            let cpos = ChunkPos::new(cx, cy);
                            if let Some(chunk) = world.chunks.get(&cpos) {
                                let chunk_wx = cx as f32 * CHUNK_SIZE as f32 * T;
                                let chunk_wy = cy as f32 * CHUNK_SIZE as f32 * T;
                                for dec in &chunk.decorations {
                                    if !matches!(
                                        dec.kind,
                                        Decoration::TallGrass
                                            | Decoration::Bush
                                            | Decoration::Flowers
                                            | Decoration::Fern
                                    ) {
                                        continue;
                                    }
                                    let dwx = chunk_wx + dec.tile_x as f32 * T + dec.offset_x;
                                    let dwy = chunk_wy + dec.tile_y as f32 * T + dec.offset_y;
                                    let dist_to_player = ((dwx - player_x).powi(2)
                                        + (dwy - player_y).powi(2))
                                    .sqrt();
                                    if dist_to_player > interact_radius {
                                        continue;
                                    }
                                    let (disp_x, disp_y) =
                                        trail.displacement_at(dwx + T * 0.5, dwy + T * 0.5);
                                    if disp_x.abs() < 0.5 && disp_y.abs() < 0.5 {
                                        continue;
                                    }
                                    let sx = dwx - cam_x + disp_x;
                                    let sy = dwy - cam_y + disp_y;
                                    draw_decoration(dec, sx, sy);
                                }
                            }
                        }
                    }
                }

                // Player
                let px = player_x - cam_x;
                let py = player_y - cam_y;
                draw_rectangle(px, py, T, T, YELLOW);

                clouds.draw(cam_x, cam_y);
                day_night.draw();

                if heat_active {
                    set_default_camera();
                    clear_background(BLACK);
                    heat_material.set_uniform("time", game_time);
                    heat_material.set_uniform("intensity", weather.transition);
                    gl_use_material(&heat_material);
                    let srt = scene_rt.as_ref().unwrap();
                    draw_texture_ex(
                        &srt.texture,
                        0.0,
                        0.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(screen_width(), screen_height())),
                            flip_y: true,
                            ..Default::default()
                        },
                    );
                    gl_use_default_material();
                    let tint_alpha = (25.0 * weather.transition) as u8;
                    draw_rectangle(
                        0.0,
                        0.0,
                        screen_width(),
                        screen_height(),
                        Color::from_rgba(200, 150, 50, tint_alpha),
                    );
                }

                weather.draw(game_time);

                // HUD
                let chunk_pos = ChunkPos::from_world(player_x, player_y, T);
                let weather_label = match weather.current {
                    WeatherType::Normal => "Clear",
                    WeatherType::Mist => "Mist",
                    WeatherType::Rain => "Rain",
                    WeatherType::Heatwave => "Heatwave",
                };
                draw_text(
                    format!(
                        "FPS: {} | Chunk: ({}, {}) | Chunks: {} | {} | {} | [E] Enter cave",
                        get_fps(),
                        chunk_pos.x,
                        chunk_pos.y,
                        world.chunks.len(),
                        day_night.time_label(),
                        weather_label,
                    )
                    .as_str(),
                    8.0,
                    20.0,
                    20.0,
                    WHITE,
                );
            }

            SceneState::Cave { .. } => {
                if let Some((cave, cave_rt)) = &cave_state {
                    // Movement (walls block)
                    let new_x = player_x + dx * PLAYER_SPEED * dt;
                    let new_y = player_y + dy * PLAYER_SPEED * dt;
                    let tile_x = ((new_x + T * 0.5) / T).floor() as i32;
                    let tile_y = ((new_y + T * 0.5) / T).floor() as i32;
                    if tile_x >= 0
                        && tile_x < CAVE_SIZE as i32
                        && tile_y >= 0
                        && tile_y < CAVE_SIZE as i32
                        && cave.tiles[tile_y as usize][tile_x as usize] == CaveTile::Floor
                    {
                        player_x = new_x;
                        player_y = new_y;
                    }

                    // Exit interaction
                    if is_key_pressed(KeyCode::E) && !transition.is_active() {
                        let ptx = (player_x / T).floor() as i32;
                        let pty = (player_y / T).floor() as i32;
                        for exit in &cave.exits {
                            let edx = (exit.tile_x as i32 - ptx).abs();
                            let edy = (exit.tile_y as i32 - pty).abs();
                            if edx <= 1 && edy <= 1 {
                                overworld_return_pos = Some((
                                    exit.target_world_x as f32 * T,
                                    exit.target_world_y as f32 * T,
                                ));
                                transition.start(SceneState::Overworld);
                                break;
                            }
                        }
                        // Also check entrance to go back
                        if !transition.is_active() {
                            let edx = (cave.entrance_x as i32 - ptx).abs();
                            let edy = (cave.entrance_y as i32 - pty).abs();
                            if edx <= 1 && edy <= 1 {
                                transition.start(SceneState::Overworld);
                            }
                        }
                    }

                    // Camera
                    let cave_px = CAVE_SIZE as f32 * T;
                    let cam_x = (player_x - screen_width() * 0.5)
                        .max(0.0)
                        .min(cave_px - screen_width());
                    let cam_y = (player_y - screen_height() * 0.5)
                        .max(0.0)
                        .min(cave_px - screen_height());

                    // Draw
                    clear_background(Color::from_rgba(10, 8, 6, 255));

                    draw_texture_ex(
                        &cave_rt.texture,
                        -cam_x,
                        -cam_y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(Vec2::new(cave_px, cave_px)),
                            flip_y: true,
                            ..Default::default()
                        },
                    );

                    // Player
                    let px = player_x - cam_x;
                    let py = player_y - cam_y;
                    draw_rectangle(px, py, T, T, YELLOW);

                    // Dark ambient
                    draw_rectangle(
                        0.0,
                        0.0,
                        screen_width(),
                        screen_height(),
                        Color::from_rgba(0, 0, 0, 50),
                    );

                    // HUD
                    draw_text(
                        format!(
                            "FPS: {} | Cave | [E] Exit",
                            get_fps(),
                        )
                        .as_str(),
                        8.0,
                        20.0,
                        20.0,
                        WHITE,
                    );
                }
            }
        }

        // Scene transition overlay (always on top)
        transition.draw();

        next_frame().await
    }
}

fn find_nearby_entrance(world: &GameWorld, px: f32, py: f32) -> Option<(i32, i32)> {
    let chunk_pos = ChunkPos::from_world(px, py, T);
    for dy in -1..=1 {
        for dx in -1..=1 {
            let cpos = ChunkPos::new(chunk_pos.x + dx, chunk_pos.y + dy);
            if let Some(chunk) = world.chunks.get(&cpos) {
                for entrance in &chunk.entrances {
                    let ewx = entrance.world_x as f32 * T + T * 0.5;
                    let ewy = entrance.world_y as f32 * T + T * 0.5;
                    let dist = ((px - ewx).powi(2) + (py - ewy).powi(2)).sqrt();
                    if dist < T * 1.5 {
                        return Some((entrance.world_x, entrance.world_y));
                    }
                }
            }
        }
    }
    None
}
