use macroquad::prelude::*;

use alj_core::chunk::Chunk;
use alj_core::terrain::Terrain;
use alj_core::{CHUNK_SIZE, TILE_SIZE};

use super::decorations::draw_decoration;

const T: f32 = TILE_SIZE;
pub(crate) const CHUNK_PX: u32 = CHUNK_SIZE as u32 * TILE_SIZE as u32;

pub(crate) fn lerp_arc(cx: f32, cy: f32, arm1: (f32, f32), arm2: (f32, f32), t: f32, r: f32) -> Vec2 {
    let dx = arm1.0 * (1.0 - t) + arm2.0 * t;
    let dy = arm1.1 * (1.0 - t) + arm2.1 * t;
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    Vec2::new(cx + dx / len * r, cy + dy / len * r)
}

pub(crate) fn bake_chunk(
    chunk: &Chunk,
    neighbor_lookup: impl Fn(i32, i32) -> Option<(Terrain, [u8; 4])>,
) -> RenderTarget {
    let rt = render_target(CHUNK_PX, CHUNK_PX);
    rt.texture.set_filter(FilterMode::Linear);

    set_camera(&Camera2D {
        render_target: Some(rt.clone()),
        zoom: vec2(2.0 / CHUNK_PX as f32, -2.0 / CHUNK_PX as f32),
        target: vec2(CHUNK_PX as f32 / 2.0, CHUNK_PX as f32 / 2.0),
        ..Default::default()
    });

    clear_background(BLANK);

    // Terrain tiles with texture detail
    for ty in 0..CHUNK_SIZE {
        for tx in 0..CHUNK_SIZE {
            let px = tx as f32 * T;
            let py = ty as f32 * T;
            let [r, g, b, a] = chunk.tile_colors[ty][tx];
            draw_rectangle(px, py, T, T, Color::from_rgba(r, g, b, a));

            let terrain = chunk.tiles[ty][tx];
            if matches!(terrain, Terrain::Water | Terrain::DeepWater) {
                continue;
            }
            let tseed = (chunk.pos.x.wrapping_mul(73) ^ chunk.pos.y.wrapping_mul(179)
                ^ (tx as i32 * 37) ^ (ty as i32 * 53)) as u32;
            let detail_dark = Color::from_rgba(
                r.saturating_sub(15),
                g.saturating_sub(12),
                b.saturating_sub(10),
                80,
            );
            let detail_light = Color::from_rgba(
                r.saturating_add(12),
                g.saturating_add(10),
                b.saturating_add(8),
                60,
            );
            let dx1 = ((tseed % 11) as f32 / 11.0) * T;
            let dy1 = ((tseed.wrapping_mul(7) % 13) as f32 / 13.0) * T;
            let dx2 = ((tseed.wrapping_mul(13) % 11) as f32 / 11.0) * T;
            let dy2 = ((tseed.wrapping_mul(17) % 13) as f32 / 13.0) * T;
            draw_circle(px + dx1, py + dy1, 1.0, detail_dark);
            draw_circle(px + dx2, py + dy2, 0.8, detail_light);
            if matches!(terrain, Terrain::Stone) {
                let cx1 = px + ((tseed.wrapping_mul(3) % 10) as f32 / 10.0) * T;
                let cy1 = py + ((tseed.wrapping_mul(11) % 10) as f32 / 10.0) * T;
                draw_line(cx1, cy1, cx1 + T * 0.25, cy1 + T * 0.1, 0.5, detail_dark);
            }
        }
    }

    // Corner smoothing at grid intersections (including chunk borders).
    let corner_r = T * 0.5;
    let segments = 8;
    let base_wx = chunk.pos.x * CHUNK_SIZE as i32;
    let base_wy = chunk.pos.y * CHUNK_SIZE as i32;

    let tile_at = |lx: i32, ly: i32| -> Option<(Terrain, [u8; 4])> {
        if lx >= 0 && lx < CHUNK_SIZE as i32 && ly >= 0 && ly < CHUNK_SIZE as i32 {
            Some((
                chunk.tiles[ly as usize][lx as usize],
                chunk.tile_colors[ly as usize][lx as usize],
            ))
        } else {
            neighbor_lookup(base_wx + lx, base_wy + ly)
        }
    };

    for gy in 0..=CHUNK_SIZE as i32 {
        for gx in 0..=CHUNK_SIZE as i32 {
            let Some((ta, _)) = tile_at(gx - 1, gy - 1) else { continue };
            let Some((tb, _)) = tile_at(gx, gy - 1) else { continue };
            let Some((tc, _)) = tile_at(gx - 1, gy) else { continue };
            let Some((td, _)) = tile_at(gx, gy) else { continue };

            let ix = gx as f32 * T;
            let iy = gy as f32 * T;

            // 3-vs-1: draw minority color into diagonal majority tile's corner.
            // Use explicit (dx,dy) endpoints — no angle math.
            // arm1/arm2 define the two edges of the quarter-circle arc,
            // both at distance corner_r from intersection, spanning the
            // quadrant of the majority tile diagonal to the minority.
            // Draw MAJORITY color INTO MINORITY tile's corner, rounding
            // the minority's sharp edge with the surrounding terrain.
            // Color from diagonal majority tile, fan points into minority.
            let corner_case: Option<([u8; 4], (f32, f32), (f32, f32))> =
                if tb == tc && tc == td && ta != tb {
                    // A minority → majority (D's) color into A: left and up
                    tile_at(gx, gy).map(|(_, c)| (c, (-corner_r, 0.0), (0.0, -corner_r)))
                } else if ta == tc && tc == td && tb != ta {
                    // B minority → majority (C's) color into B: right and up
                    tile_at(gx - 1, gy).map(|(_, c)| (c, (corner_r, 0.0), (0.0, -corner_r)))
                } else if ta == tb && tb == td && tc != ta {
                    // C minority → majority (B's) color into C: left and down
                    tile_at(gx, gy - 1).map(|(_, c)| (c, (-corner_r, 0.0), (0.0, corner_r)))
                } else if ta == tb && tb == tc && td != ta {
                    // D minority → majority (A's) color into D: right and down
                    tile_at(gx - 1, gy - 1).map(|(_, c)| (c, (corner_r, 0.0), (0.0, corner_r)))
                } else {
                    None
                };

            if let Some((color, arm1, arm2)) = corner_case {
                let [r, g, b, _] = color;
                let c = Color::from_rgba(r, g, b, 255);
                let center = Vec2::new(ix, iy);
                for i in 0..segments {
                    let t1 = i as f32 / segments as f32;
                    let t2 = (i + 1) as f32 / segments as f32;
                    // Interpolate along arc using normalized lerp
                    let p1 = lerp_arc(ix, iy, arm1, arm2, t1, corner_r);
                    let p2 = lerp_arc(ix, iy, arm1, arm2, t2, corner_r);
                    draw_triangle(center, p1, p2, c);
                }
                continue;
            }

        }
    }

    // Decorations
    for dec in &chunk.decorations {
        let dx = dec.tile_x as f32 * T + dec.offset_x;
        let dy = dec.tile_y as f32 * T + dec.offset_y;
        draw_decoration(dec, dx, dy);
    }

    // Cave entrances
    for entrance in &chunk.entrances {
        let ex = entrance.tile_x as f32 * T;
        let ey = entrance.tile_y as f32 * T;
        let cx = ex + T * 0.5;
        let cy = ey + T * 0.5;
        let dark = Color::from_rgba(20, 15, 10, 255);
        let deeper = Color::from_rgba(8, 5, 3, 255);
        draw_ellipse(cx, cy, T * 0.9, T * 0.55, 0.0, dark);
        draw_ellipse(cx, cy + T * 0.05, T * 0.55, T * 0.35, 0.0, deeper);
        let edge = Color::from_rgba(30, 22, 15, 255);
        draw_triangle(
            Vec2::new(cx - T * 0.7, cy - T * 0.1),
            Vec2::new(cx - T * 0.5, cy - T * 0.4),
            Vec2::new(cx - T * 0.3, cy - T * 0.15),
            edge,
        );
        draw_triangle(
            Vec2::new(cx + T * 0.4, cy - T * 0.15),
            Vec2::new(cx + T * 0.6, cy - T * 0.35),
            Vec2::new(cx + T * 0.75, cy - T * 0.05),
            edge,
        );
        draw_triangle(
            Vec2::new(cx - T * 0.2, cy + T * 0.3),
            Vec2::new(cx + T * 0.1, cy + T * 0.5),
            Vec2::new(cx + T * 0.3, cy + T * 0.25),
            edge,
        );
    }

    set_default_camera();
    rt
}
