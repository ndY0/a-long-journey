use macroquad::prelude::*;

use alj_core::decoration::{Decoration, DecorationInstance};
use alj_core::TILE_SIZE;

const T: f32 = TILE_SIZE;

pub(crate) fn draw_decoration(dec: &DecorationInstance, x: f32, y: f32) {
    let s = dec.size;
    let v = dec.variant;
    let cx = x + T * 0.5;
    let cy = y + T * 0.5;

    match dec.kind {
        Decoration::PineTree => {
            let trunk_w = T * 0.15 * s;
            let trunk_h = T * 0.5 * s;
            let trunk_color = match v {
                0 => Color::from_rgba(90, 60, 30, 255),
                _ => Color::from_rgba(70, 50, 25, 255),
            };
            draw_rectangle(
                cx - trunk_w * 0.5,
                cy + T * 0.1,
                trunk_w,
                trunk_h,
                trunk_color,
            );

            let green = match v {
                0 => Color::from_rgba(15, 80, 15, 255),
                1 => Color::from_rgba(20, 90, 25, 255),
                2 => Color::from_rgba(10, 70, 20, 255),
                _ => Color::from_rgba(25, 85, 15, 255),
            };
            let canopy_w = T * 0.9 * s;
            let canopy_h = T * 1.0 * s;
            draw_triangle(
                Vec2::new(cx, cy - canopy_h * 0.5),
                Vec2::new(cx - canopy_w * 0.5, cy + T * 0.15),
                Vec2::new(cx + canopy_w * 0.5, cy + T * 0.15),
                green,
            );
            let lighter = Color::from_rgba(
                ((green.r * 255.0) as u8).saturating_add(15),
                ((green.g * 255.0) as u8).saturating_add(10),
                ((green.b * 255.0) as u8).saturating_add(5),
                255,
            );
            draw_triangle(
                Vec2::new(cx, cy - canopy_h * 0.35),
                Vec2::new(cx - canopy_w * 0.35, cy),
                Vec2::new(cx + canopy_w * 0.35, cy),
                lighter,
            );
        }
        Decoration::OakTree => {
            let trunk_color = Color::from_rgba(100, 70, 35, 255);
            let trunk_w = T * 0.18 * s;
            let trunk_h = T * 0.55 * s;
            draw_rectangle(cx - trunk_w * 0.5, cy, trunk_w, trunk_h, trunk_color);

            let canopy = match v {
                0 => Color::from_rgba(40, 130, 40, 255),
                1 => Color::from_rgba(50, 140, 35, 255),
                2 => Color::from_rgba(35, 120, 45, 255),
                _ => Color::from_rgba(45, 135, 30, 255),
            };
            let r = T * 0.45 * s;
            draw_circle(cx, cy - T * 0.1, r, canopy);
            let highlight = Color::from_rgba(
                ((canopy.r * 255.0) as u8).saturating_add(20),
                ((canopy.g * 255.0) as u8).saturating_add(15),
                (canopy.b * 255.0) as u8,
                255,
            );
            draw_circle(cx - r * 0.3, cy - T * 0.2, r * 0.45, highlight);
        }
        Decoration::BirchTree => {
            let trunk_color = Color::from_rgba(210, 200, 180, 255);
            let trunk_w = T * 0.12 * s;
            let trunk_h = T * 0.6 * s;
            draw_rectangle(
                cx - trunk_w * 0.5,
                cy - T * 0.05,
                trunk_w,
                trunk_h,
                trunk_color,
            );
            let dark_mark = Color::from_rgba(120, 110, 100, 255);
            draw_rectangle(cx - trunk_w * 0.5, cy + T * 0.15, trunk_w, 1.5, dark_mark);
            draw_rectangle(cx - trunk_w * 0.5, cy + T * 0.3, trunk_w, 1.5, dark_mark);

            let canopy = match v {
                0 => Color::from_rgba(80, 170, 50, 255),
                1 => Color::from_rgba(90, 180, 60, 255),
                2 => Color::from_rgba(100, 175, 55, 255),
                _ => Color::from_rgba(75, 165, 45, 255),
            };
            let r = T * 0.4 * s;
            draw_circle(cx, cy - T * 0.15, r, canopy);
        }
        Decoration::Bush => {
            let color = match v {
                0 => Color::from_rgba(45, 120, 35, 255),
                1 => Color::from_rgba(55, 130, 40, 255),
                2 => Color::from_rgba(50, 115, 38, 255),
                _ => Color::from_rgba(40, 110, 30, 255),
            };
            let r = T * 0.3 * s;
            draw_circle(cx, cy + T * 0.1, r, color);
            let darker = Color::from_rgba(
                ((color.r * 255.0) as u8).saturating_sub(12),
                ((color.g * 255.0) as u8).saturating_sub(15),
                ((color.b * 255.0) as u8).saturating_sub(8),
                255,
            );
            draw_circle(cx - r * 0.5, cy + T * 0.15, r * 0.55, darker);
            draw_circle(cx + r * 0.5, cy + T * 0.15, r * 0.55, darker);
            let leaf = Color::from_rgba(
                ((color.r * 255.0) as u8).saturating_add(8),
                ((color.g * 255.0) as u8).saturating_add(12),
                ((color.b * 255.0) as u8).saturating_add(5),
                255,
            );
            draw_circle(cx, cy + T * 0.02, r * 0.4, leaf);
        }
        Decoration::FlowerBush => {
            let bush_color = Color::from_rgba(50, 125, 40, 255);
            let r = T * 0.3 * s;
            draw_circle(cx, cy + T * 0.1, r, bush_color);
            let darker = Color::from_rgba(38, 100, 30, 255);
            draw_circle(cx - r * 0.4, cy + T * 0.15, r * 0.5, darker);
            draw_circle(cx + r * 0.4, cy + T * 0.15, r * 0.5, darker);

            let flower_colors = [
                Color::from_rgba(220, 60, 60, 255),
                Color::from_rgba(220, 180, 50, 255),
                Color::from_rgba(200, 80, 200, 255),
                Color::from_rgba(240, 130, 50, 255),
                Color::from_rgba(100, 100, 220, 255),
            ];
            let fc = flower_colors[v as usize % flower_colors.len()];
            let fr = T * 0.08;
            draw_circle(cx - r * 0.3, cy, fr, fc);
            draw_circle(cx + r * 0.25, cy + T * 0.05, fr, fc);
            draw_circle(cx, cy + T * 0.15, fr * 0.9, fc);
            draw_circle(cx + r * 0.15, cy - T * 0.05, fr * 0.85, fc);
        }
        Decoration::Rock => {
            let color = match v {
                0 => Color::from_rgba(150, 145, 135, 255),
                1 => Color::from_rgba(140, 135, 130, 255),
                2 => Color::from_rgba(160, 155, 145, 255),
                _ => Color::from_rgba(135, 130, 125, 255),
            };
            let w = T * 0.45 * s;
            let h = T * 0.3 * s;
            draw_rectangle(cx - w * 0.5, cy + T * 0.1, w, h, color);
            let highlight = Color::from_rgba(
                ((color.r * 255.0) as u8).saturating_add(20),
                ((color.g * 255.0) as u8).saturating_add(20),
                ((color.b * 255.0) as u8).saturating_add(15),
                255,
            );
            draw_rectangle(cx - w * 0.4, cy + T * 0.1, w * 0.5, h * 0.3, highlight);
        }
        Decoration::Boulder => {
            let base = Color::from_rgba(120, 115, 105, 255);
            let r = T * 0.4 * s;
            draw_circle(cx, cy + T * 0.05, r, base);
            let highlight = Color::from_rgba(145, 140, 130, 255);
            draw_circle(cx - r * 0.25, cy - T * 0.05, r * 0.4, highlight);
            let shadow = Color::from_rgba(90, 85, 80, 180);
            draw_rectangle(cx - r, cy + T * 0.05 + r * 0.7, r * 2.0, T * 0.08, shadow);
        }
        Decoration::MountainPeak => {
            let base = match v {
                0 => Color::from_rgba(110, 105, 95, 255),
                1 => Color::from_rgba(120, 115, 105, 255),
                _ => Color::from_rgba(100, 95, 90, 255),
            };
            let h = T * 1.8 * s;
            let w = T * 1.4 * s;
            let base_y = cy + T * 0.4;
            draw_triangle(
                Vec2::new(cx, base_y - h),
                Vec2::new(cx - w * 0.5, base_y),
                Vec2::new(cx + w * 0.5, base_y),
                base,
            );
            let snow_cap = Color::from_rgba(235, 235, 245, 255);
            let snow_h = h * 0.25;
            let snow_w = w * 0.25;
            draw_triangle(
                Vec2::new(cx, base_y - h),
                Vec2::new(cx - snow_w * 0.5, base_y - h + snow_h),
                Vec2::new(cx + snow_w * 0.5, base_y - h + snow_h),
                snow_cap,
            );
            let mid = Color::from_rgba(
                ((base.r * 255.0) as u8).saturating_add(10),
                ((base.g * 255.0) as u8).saturating_add(10),
                ((base.b * 255.0) as u8).saturating_add(8),
                255,
            );
            let mid_h = h * 0.4;
            let mid_w = w * 0.5;
            draw_triangle(
                Vec2::new(cx + w * 0.05, base_y - h + snow_h),
                Vec2::new(cx - mid_w * 0.5, base_y - h + snow_h + mid_h),
                Vec2::new(cx + mid_w * 0.5, base_y - h + snow_h + mid_h),
                mid,
            );
        }
        Decoration::Pebbles => {
            let color = Color::from_rgba(160, 155, 145, 220);
            let pr = T * 0.06 * s;
            draw_circle(cx - T * 0.15, cy + T * 0.05, pr, color);
            draw_circle(cx + T * 0.1, cy + T * 0.15, pr * 0.85, color);
            draw_circle(cx - T * 0.05, cy + T * 0.2, pr * 0.7, color);
            if v > 1 {
                draw_circle(cx + T * 0.18, cy - T * 0.05, pr * 0.6, color);
            }
        }
        Decoration::Flowers => {
            let colors = [
                Color::from_rgba(230, 70, 70, 255),
                Color::from_rgba(230, 200, 60, 255),
                Color::from_rgba(210, 90, 210, 255),
                Color::from_rgba(70, 130, 230, 255),
                Color::from_rgba(240, 150, 60, 255),
            ];
            let c = colors[v as usize % colors.len()];
            let fr = T * 0.07 * s;
            let stem = Color::from_rgba(50, 120, 40, 255);

            draw_line(
                cx - T * 0.12,
                cy + T * 0.05,
                cx - T * 0.12,
                cy + T * 0.25,
                1.0,
                stem,
            );
            draw_circle(cx - T * 0.12, cy + T * 0.02, fr, c);

            draw_line(
                cx + T * 0.1,
                cy + T * 0.1,
                cx + T * 0.1,
                cy + T * 0.3,
                1.0,
                stem,
            );
            draw_circle(cx + T * 0.1, cy + T * 0.07, fr * 0.85, c);

            draw_line(
                cx - T * 0.02,
                cy + T * 0.15,
                cx - T * 0.02,
                cy + T * 0.35,
                1.0,
                stem,
            );
            draw_circle(cx - T * 0.02, cy + T * 0.12, fr * 0.75, c);
        }
        Decoration::Driftwood => {
            let color = Color::from_rgba(140, 110, 70, 255);
            let lighter = Color::from_rgba(170, 140, 90, 255);
            let w = T * 0.55 * s;
            draw_line(
                cx - w * 0.5,
                cy + T * 0.1,
                cx + w * 0.5,
                cy + T * 0.15,
                T * 0.1,
                color,
            );
            draw_line(
                cx + w * 0.2,
                cy + T * 0.12,
                cx + w * 0.35,
                cy - T * 0.05,
                T * 0.06,
                lighter,
            );
        }
        Decoration::SnowPile => {
            let color = Color::from_rgba(225, 225, 235, 255);
            let r = T * 0.3 * s;
            draw_circle(cx, cy + T * 0.1, r, color);
            let shine = Color::from_rgba(240, 240, 250, 255);
            draw_circle(cx - r * 0.25, cy + T * 0.03, r * 0.4, shine);
        }
        Decoration::Cactus => {
            let green = match v {
                0 => Color::from_rgba(60, 140, 50, 255),
                1 => Color::from_rgba(50, 130, 45, 255),
                _ => Color::from_rgba(55, 135, 55, 255),
            };
            let trunk_w = T * 0.15 * s;
            let trunk_h = T * 0.8 * s;
            draw_rectangle(cx - trunk_w * 0.5, cy - T * 0.2, trunk_w, trunk_h, green);
            let arm_w = T * 0.12 * s;
            let arm_h = T * 0.25 * s;
            draw_rectangle(cx - trunk_w * 0.5 - arm_w, cy, arm_w, arm_h, green);
            draw_rectangle(cx + trunk_w * 0.5, cy - T * 0.1, arm_w, arm_h, green);
            draw_rectangle(cx - trunk_w * 0.5 - arm_w, cy - arm_h, arm_w, T * 0.08, green);
            draw_rectangle(cx + trunk_w * 0.5, cy - T * 0.1 - arm_h, arm_w, T * 0.08, green);
        }
        Decoration::PalmTree => {
            let trunk = Color::from_rgba(140, 100, 55, 255);
            let tw = T * 0.1 * s;
            draw_line(cx, cy + T * 0.4, cx - T * 0.1, cy - T * 0.3, tw, trunk);
            let canopy = match v {
                0 => Color::from_rgba(30, 150, 40, 255),
                1 => Color::from_rgba(40, 160, 35, 255),
                _ => Color::from_rgba(25, 140, 45, 255),
            };
            let top_x = cx - T * 0.1;
            let top_y = cy - T * 0.35;
            let lr = T * 0.35 * s;
            draw_circle(top_x, top_y, lr * 0.5, canopy);
            let frond = Color::from_rgba(
                ((canopy.r * 255.0) as u8).saturating_sub(10),
                ((canopy.g * 255.0) as u8).saturating_add(10),
                (canopy.b * 255.0) as u8,
                255,
            );
            draw_line(top_x, top_y, top_x - lr, top_y + lr * 0.4, 1.5, frond);
            draw_line(top_x, top_y, top_x + lr, top_y + lr * 0.3, 1.5, frond);
            draw_line(top_x, top_y, top_x - lr * 0.5, top_y - lr * 0.5, 1.5, frond);
            draw_line(top_x, top_y, top_x + lr * 0.7, top_y - lr * 0.4, 1.5, frond);
        }
        Decoration::DeadTree => {
            let trunk = Color::from_rgba(80, 60, 40, 255);
            let tw = T * 0.12 * s;
            let th = T * 0.7 * s;
            draw_rectangle(cx - tw * 0.5, cy - T * 0.1, tw, th, trunk);
            let branch = Color::from_rgba(70, 55, 35, 255);
            draw_line(cx, cy, cx - T * 0.25 * s, cy - T * 0.2, 1.5, branch);
            draw_line(cx, cy - T * 0.1, cx + T * 0.3 * s, cy - T * 0.25, 1.5, branch);
            draw_line(cx, cy + T * 0.1, cx - T * 0.2 * s, cy + T * 0.05, 1.0, branch);
        }
        Decoration::TallGrass => {
            let green = match v {
                0 => Color::from_rgba(50, 150, 45, 255),
                1 => Color::from_rgba(70, 160, 50, 255),
                2 => Color::from_rgba(60, 140, 55, 255),
                _ => Color::from_rgba(80, 155, 40, 255),
            };
            let h = T * 0.4 * s;
            draw_line(cx - T * 0.08, cy + T * 0.2, cx - T * 0.12, cy + T * 0.2 - h, 1.2, green);
            draw_line(cx, cy + T * 0.2, cx + T * 0.03, cy + T * 0.2 - h * 1.1, 1.2, green);
            draw_line(cx + T * 0.08, cy + T * 0.2, cx + T * 0.14, cy + T * 0.2 - h * 0.9, 1.2, green);
            if v > 1 {
                draw_line(cx - T * 0.15, cy + T * 0.2, cx - T * 0.18, cy + T * 0.2 - h * 0.7, 1.0, green);
                draw_line(cx + T * 0.15, cy + T * 0.2, cx + T * 0.2, cy + T * 0.2 - h * 0.8, 1.0, green);
            }
        }
        Decoration::DryShrub => {
            let color = match v {
                0 => Color::from_rgba(160, 140, 70, 255),
                1 => Color::from_rgba(150, 130, 60, 255),
                _ => Color::from_rgba(170, 145, 75, 255),
            };
            let r = T * 0.22 * s;
            draw_circle(cx, cy + T * 0.1, r, color);
            let twig = Color::from_rgba(120, 100, 50, 255);
            draw_line(cx, cy + T * 0.05, cx - T * 0.15, cy - T * 0.05, 0.8, twig);
            draw_line(cx, cy + T * 0.05, cx + T * 0.12, cy - T * 0.08, 0.8, twig);
        }
        Decoration::SandDune => {
            let color = Color::from_rgba(215, 200, 150, 200);
            let w = T * 0.7 * s;
            let h = T * 0.2 * s;
            draw_triangle(
                Vec2::new(cx, cy + T * 0.15 - h),
                Vec2::new(cx - w * 0.5, cy + T * 0.15),
                Vec2::new(cx + w * 0.5, cy + T * 0.15),
                color,
            );
            let highlight = Color::from_rgba(225, 215, 170, 180);
            draw_triangle(
                Vec2::new(cx - w * 0.1, cy + T * 0.15 - h * 0.8),
                Vec2::new(cx - w * 0.4, cy + T * 0.15),
                Vec2::new(cx, cy + T * 0.15),
                highlight,
            );
        }
        Decoration::Reeds => {
            let stem = Color::from_rgba(100, 130, 60, 255);
            let tip = Color::from_rgba(140, 120, 70, 255);
            let h = T * 0.45 * s;
            for i in 0..3 {
                let ox = (i as f32 - 1.0) * T * 0.08;
                let sway = (i as f32 - 1.0) * T * 0.02;
                draw_line(cx + ox, cy + T * 0.25, cx + ox + sway, cy + T * 0.25 - h, 1.0, stem);
                draw_circle(cx + ox + sway, cy + T * 0.25 - h, T * 0.03 * s, tip);
            }
        }
        Decoration::Mushroom => {
            let stem_color = Color::from_rgba(210, 200, 180, 255);
            let cap_colors = [
                Color::from_rgba(200, 50, 40, 255),
                Color::from_rgba(220, 160, 50, 255),
                Color::from_rgba(180, 130, 80, 255),
                Color::from_rgba(150, 80, 160, 255),
            ];
            let cap = cap_colors[v as usize % cap_colors.len()];
            let sw = T * 0.04 * s;
            let sh = T * 0.2 * s;
            draw_rectangle(cx - sw * 0.5, cy + T * 0.1, sw, sh, stem_color);
            let cr = T * 0.12 * s;
            draw_circle(cx, cy + T * 0.08, cr, cap);
            let dot = Color::from_rgba(240, 240, 230, 200);
            draw_circle(cx - cr * 0.3, cy + T * 0.06, cr * 0.2, dot);
            draw_circle(cx + cr * 0.2, cy + T * 0.1, cr * 0.15, dot);
        }
        Decoration::FlatRock => {
            let color = match v {
                0 => Color::from_rgba(130, 125, 120, 255),
                1 => Color::from_rgba(120, 118, 112, 255),
                _ => Color::from_rgba(140, 135, 125, 255),
            };
            let w = T * 0.6 * s;
            let h = T * 0.15 * s;
            draw_rectangle(cx - w * 0.5, cy + T * 0.15, w, h, color);
            let highlight = Color::from_rgba(
                ((color.r * 255.0) as u8).saturating_add(15),
                ((color.g * 255.0) as u8).saturating_add(15),
                ((color.b * 255.0) as u8).saturating_add(12),
                255,
            );
            draw_rectangle(cx - w * 0.4, cy + T * 0.15, w * 0.5, h * 0.4, highlight);
        }
        Decoration::IceChunk => {
            let ice = Color::from_rgba(180, 210, 240, 200);
            let w = T * 0.35 * s;
            let h = T * 0.4 * s;
            draw_triangle(
                Vec2::new(cx, cy - h * 0.3),
                Vec2::new(cx - w * 0.5, cy + T * 0.15),
                Vec2::new(cx + w * 0.5, cy + T * 0.15),
                ice,
            );
            let shine = Color::from_rgba(220, 235, 250, 180);
            draw_triangle(
                Vec2::new(cx - w * 0.1, cy - h * 0.15),
                Vec2::new(cx - w * 0.35, cy + T * 0.1),
                Vec2::new(cx - w * 0.05, cy + T * 0.05),
                shine,
            );
        }
        Decoration::Fern => {
            let green = match v {
                0 => Color::from_rgba(30, 135, 35, 255),
                1 => Color::from_rgba(25, 125, 40, 255),
                _ => Color::from_rgba(35, 145, 30, 255),
            };
            let r = T * 0.3 * s;
            for i in 0..5 {
                let angle = (i as f32 / 5.0) * std::f32::consts::TAU + 0.3;
                let dx = angle.cos() * r;
                let dy = angle.sin() * r * 0.6;
                draw_line(cx, cy + T * 0.1, cx + dx, cy + T * 0.1 + dy, 1.5, green);
                draw_circle(cx + dx, cy + T * 0.1 + dy, T * 0.04, green);
            }
        }
        Decoration::Moss => {
            let color = match v {
                0 => Color::from_rgba(70, 120, 50, 180),
                1 => Color::from_rgba(80, 130, 55, 180),
                _ => Color::from_rgba(65, 115, 45, 180),
            };
            let r = T * 0.15 * s;
            draw_circle(cx - T * 0.05, cy + T * 0.1, r, color);
            draw_circle(cx + T * 0.08, cy + T * 0.15, r * 0.8, color);
            draw_circle(cx, cy + T * 0.05, r * 0.6, color);
        }
        Decoration::Seashell => {
            let shell = match v {
                0 => Color::from_rgba(230, 200, 190, 255),
                1 => Color::from_rgba(220, 190, 180, 255),
                _ => Color::from_rgba(240, 215, 200, 255),
            };
            let r = T * 0.08 * s;
            draw_circle(cx, cy + T * 0.15, r, shell);
            let inner = Color::from_rgba(200, 170, 160, 255);
            draw_circle(cx + r * 0.3, cy + T * 0.15, r * 0.5, inner);
            let accent = Color::from_rgba(210, 140, 130, 255);
            draw_line(cx - r, cy + T * 0.15, cx + r * 0.3, cy + T * 0.12, 0.8, accent);
        }
    }
}
