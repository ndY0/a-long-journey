use macroquad::prelude::*;

use alj_core::cave::{CaveMap, CaveTile, CAVE_SIZE};
use alj_core::TILE_SIZE;

use super::decorations::draw_decoration;

const T: f32 = TILE_SIZE;

pub(crate) fn bake_cave(cave: &CaveMap) -> RenderTarget {
    let size = (CAVE_SIZE as f32 * T) as u32;
    let rt = render_target(size, size);
    rt.texture.set_filter(FilterMode::Linear);

    set_camera(&Camera2D {
        render_target: Some(rt.clone()),
        zoom: vec2(2.0 / size as f32, -2.0 / size as f32),
        target: vec2(size as f32 / 2.0, size as f32 / 2.0),
        ..Default::default()
    });

    clear_background(Color::from_rgba(15, 12, 10, 255));

    let floor_base: [u8; 3] = [75, 75, 80];
    let wall_base: [u8; 3] = [35, 30, 28];

    for ty in 0..CAVE_SIZE {
        for tx in 0..CAVE_SIZE {
            let px = tx as f32 * T;
            let py = ty as f32 * T;

            match cave.tiles[ty][tx] {
                CaveTile::Floor => {
                    let tseed = (tx as u32).wrapping_mul(37) ^ (ty as u32).wrapping_mul(53);
                    let shade = (tseed % 12) as u8;
                    let r = floor_base[0].wrapping_add(shade);
                    let g = floor_base[1].wrapping_add(shade);
                    let b = floor_base[2].wrapping_add(shade);
                    draw_rectangle(px, py, T, T, Color::from_rgba(r, g, b, 255));

                    let detail = Color::from_rgba(
                        r.saturating_sub(10),
                        g.saturating_sub(8),
                        b.saturating_sub(8),
                        100,
                    );
                    let dx1 = ((tseed % 11) as f32 / 11.0) * T;
                    let dy1 = ((tseed.wrapping_mul(7) % 13) as f32 / 13.0) * T;
                    draw_circle(px + dx1, py + dy1, 0.8, detail);
                }
                CaveTile::Wall => {
                    let tseed = (tx as u32).wrapping_mul(41) ^ (ty as u32).wrapping_mul(59);
                    let shade = (tseed % 10) as u8;
                    let r = wall_base[0].wrapping_add(shade);
                    let g = wall_base[1].wrapping_add(shade);
                    let b = wall_base[2].wrapping_add(shade);
                    draw_rectangle(px, py, T, T, Color::from_rgba(r, g, b, 255));
                }
            }
        }
    }

    // Corner smoothing between wall and floor
    let corner_r = T * 0.45;
    let segments = 6;
    for gy in 1..CAVE_SIZE {
        for gx in 1..CAVE_SIZE {
            let ta = cave.tiles[gy - 1][gx - 1];
            let tb = cave.tiles[gy - 1][gx];
            let tc = cave.tiles[gy][gx - 1];
            let td = cave.tiles[gy][gx];

            let ix = gx as f32 * T;
            let iy = gy as f32 * T;

            let wall_col = Color::from_rgba(wall_base[0], wall_base[1], wall_base[2], 255);

            let corner = if tb == tc && tc == td && ta != tb && ta == CaveTile::Floor {
                Some((wall_col, (-corner_r, 0.0), (0.0, -corner_r)))
            } else if ta == tc && tc == td && tb != ta && tb == CaveTile::Floor {
                Some((wall_col, (corner_r, 0.0), (0.0, -corner_r)))
            } else if ta == tb && tb == td && tc != ta && tc == CaveTile::Floor {
                Some((wall_col, (-corner_r, 0.0), (0.0, corner_r)))
            } else if ta == tb && tb == tc && td != ta && td == CaveTile::Floor {
                Some((wall_col, (corner_r, 0.0), (0.0, corner_r)))
            } else {
                None
            };

            if let Some((c, arm1, arm2)) = corner {
                let center = Vec2::new(ix, iy);
                for i in 0..segments {
                    let t1 = i as f32 / segments as f32;
                    let t2 = (i + 1) as f32 / segments as f32;
                    let p1 = lerp_arc_cave(ix, iy, arm1, arm2, t1, corner_r);
                    let p2 = lerp_arc_cave(ix, iy, arm1, arm2, t2, corner_r);
                    draw_triangle(center, p1, p2, c);
                }
            }
        }
    }

    // Decorations
    for dec in &cave.decorations {
        let dx = dec.tile_x as f32 * T + dec.offset_x;
        let dy = dec.tile_y as f32 * T + dec.offset_y;
        draw_decoration(dec, dx, dy);
    }

    // Exit markers
    for exit in &cave.exits {
        let ex = exit.tile_x as f32 * T + T * 0.5;
        let ey = exit.tile_y as f32 * T + T * 0.5;
        draw_circle(ex, ey, T * 0.4, Color::from_rgba(120, 115, 90, 180));
        draw_circle(ex, ey, T * 0.25, Color::from_rgba(160, 150, 120, 160));
        draw_triangle(
            Vec2::new(ex, ey - T * 0.3),
            Vec2::new(ex - T * 0.15, ey),
            Vec2::new(ex + T * 0.15, ey),
            Color::from_rgba(200, 190, 150, 140),
        );
    }

    // Entrance marker
    let enx = cave.entrance_x as f32 * T + T * 0.5;
    let eny = cave.entrance_y as f32 * T + T * 0.5;
    draw_circle(enx, eny, T * 0.35, Color::from_rgba(100, 110, 120, 150));

    set_default_camera();
    rt
}

fn lerp_arc_cave(cx: f32, cy: f32, arm1: (f32, f32), arm2: (f32, f32), t: f32, r: f32) -> Vec2 {
    let dx = arm1.0 * (1.0 - t) + arm2.0 * t;
    let dy = arm1.1 * (1.0 - t) + arm2.1 * t;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    Vec2::new(cx + dx / len * r, cy + dy / len * r)
}
