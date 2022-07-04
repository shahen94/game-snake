use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

use super::common::BLOCK_SIZE;

pub fn to_coords(coord: i32) -> f64 {
    (coord as f64) * BLOCK_SIZE
}

pub fn to_coords_u32(coords: i32) -> u32 {
    to_coords(coords) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, ctx: &Context, g: &mut G2d) {
    let gui_x = to_coords(x);
    let gui_y = to_coords(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        ctx.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    ctx: &Context,
    g: &mut G2d,
) {
    let gui_x = to_coords(x);
    let gui_y = to_coords(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE * (width as f64), BLOCK_SIZE * (height as f64)],
        ctx.transform,
        g,
    );
}
