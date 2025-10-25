extern crate piston_window;
extern crate rand;
mod draw;
mod game;
mod snake;
mod menu;
use game::Game;
use menu::{Menu, MenuState};
use piston_window::types::Color;
use piston_window::*;
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const GAME_WIDTH: i32 = 30;
const GAME_HEIGHT: i32 = 30;

fn main() {
    //https://magiclen.org/rust-compile-optimize/
    let mut window: PistonWindow =
        WindowSettings::new("贪吃蛇游戏", [800, 600])
            .exit_on_esc(true)
            .resizable(true)
            .build()
            .unwrap();
    
    let mut menu = Menu::new(800.0, 600.0);
    let mut game: Option<Game> = None;
    
    // 尝试加载字体
    if let Err(e) = menu.load_font() {
        eprintln!("无法加载字体: {}", e);
    }
    
    while let Some(event) = window.next() {
        // 处理鼠标点击事件
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            if let Some([x, y]) = event.mouse_cursor_args() {
                menu.handle_click(x, y);
            } else {
                // 使用默认位置进行测试
                menu.handle_click(400.0, 300.0);
            }
        }
        
        // 检查是否需要创建游戏实例
        if menu.state == MenuState::Playing && game.is_none() {
            game = Some(Game::new_with_mode(GAME_WIDTH, GAME_HEIGHT, menu.selected_mode));
        }
        
        // 处理键盘事件
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if menu.state == MenuState::Playing {
                if let Some(ref mut game) = game {
                    game.key_pressed(key);
                }
            } else {
                menu.handle_key(key);
            }
        }
        
        // 处理窗口大小变化
        if let Some(args) = event.resize_args() {
            let [width, height] = args.window_size;
            menu.update_window_size(width, height);
            if let Some(ref mut game) = game {
                game.update_window_size(width, height);
            }
        }
        
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            
            match menu.state {
                MenuState::Main | MenuState::ModeSelection => {
                    menu.draw(&c, g);
                }
                MenuState::Playing => {
                    if let Some(ref game) = game {
                        game.draw(&c, g);
                    }
                }
            }
        });
        
        event.update(|arg| {
            if menu.state == MenuState::Playing {
                if let Some(ref mut game) = game {
                    game.update(arg.dt);
                }
            }
        });
    }
}
