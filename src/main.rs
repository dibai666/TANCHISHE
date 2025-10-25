extern crate piston_window;
extern crate rand;
mod draw;
mod game;
mod snake;
use draw::to_coord_u32;
use game::Game;
use piston_window::types::Color;
use piston_window::*;
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const GAME_WIDTH: i32 = 30;
const GAME_HEIGHT: i32 = 30;

fn main() {
    //https://magiclen.org/rust-compile-optimize/
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(GAME_WIDTH), to_coord_u32(GAME_HEIGHT)])
            .exit_on_esc(true)
            .resizable(true)
            .build()
            .unwrap();
    let mut game = Game::new(GAME_WIDTH, GAME_HEIGHT);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        
        // 处理窗口大小变化
        if let Some(args) = event.resize_args() {
            game.update_window_size(args[0] as f64, args[1] as f64);
        }
        
        window.draw_2d(&event, |c, g| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
