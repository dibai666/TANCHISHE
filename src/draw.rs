use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};
const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn to_coord_dynamic(game_coord: i32, block_size: f64) -> f64 {
    (game_coord as f64) * block_size
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_block_dynamic(color: Color, x: i32, y: i32, block_size: f64, con: &Context, g: &mut G2d) {
    let gui_x = to_coord_dynamic(x, block_size);
    let gui_y = to_coord_dynamic(y, block_size);
    rectangle(
        color,
        [gui_x, gui_y, block_size, block_size],
        con.transform,
        g,
    );
}

pub fn draw_block_dynamic_with_offset(color: Color, x: i32, y: i32, block_size: f64, offset_x: f64, offset_y: f64, con: &Context, g: &mut G2d) {
    let gui_x = to_coord_dynamic(x, block_size) + offset_x;
    let gui_y = to_coord_dynamic(y, block_size) + offset_y;
    rectangle(
        color,
        [gui_x, gui_y, block_size, block_size],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);
    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}

pub fn draw_rectangle_dynamic(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    block_size: f64,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord_dynamic(x, block_size);
    let y = to_coord_dynamic(y, block_size);
    rectangle(
        color,
        [
            x,
            y,
            block_size * (width as f64),
            block_size * (height as f64),
        ],
        con.transform,
        g,
    );
}

pub fn draw_rectangle_dynamic_with_offset(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    block_size: f64,
    offset_x: f64,
    offset_y: f64,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord_dynamic(x, block_size) + offset_x;
    let y = to_coord_dynamic(y, block_size) + offset_y;
    rectangle(
        color,
        [
            x,
            y,
            block_size * (width as f64),
            block_size * (height as f64),
        ],
        con.transform,
        g,
    );
}
