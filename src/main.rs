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
use piston_window::{Glyphs, TextureSettings};
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
    // 注意：当前依赖版本不支持直接关闭 IME；如需避免编辑器“打字”，
    // 请确保焦点在游戏窗口（单击游戏窗口或 Alt+Tab 切过去）。
    
    let mut menu = Menu::new(800.0, 600.0);
    // 加载字体 
    let font_bytes: &'static [u8] = include_bytes!("../assets/FiraSans-Regular.ttf");
    let mut glyphs = Glyphs::from_bytes(
        font_bytes,
        window.create_texture_context(),
        TextureSettings::new(),
    ).unwrap();
    let mut game: Option<Game> = None;
    let mut cursor_pos = [0.0, 0.0];
    
    // 尝试加载字体
    if let Err(e) = menu.load_font() {
        eprintln!("无法加载字体: {}", e);
    }
    
    while let Some(event) = window.next() {
        // 更新光标位置
        if let Some(pos) = event.mouse_cursor_args() {
            cursor_pos = pos;
        }
        
        // 处理鼠标点击事件
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            println!("Mouse button pressed!");
            println!("Mouse click detected at: ({}, {})", cursor_pos[0], cursor_pos[1]);
            menu.handle_click(cursor_pos[0], cursor_pos[1]);
        }
        
        // 检查是否需要创建游戏实例
        if menu.state == MenuState::Playing && game.is_none() {
            game = Some(Game::new_with_mode(GAME_WIDTH, GAME_HEIGHT, menu.selected_mode, menu.selected_speed));
        }
        
        // 检查是否需要重新开始游戏
        if menu.should_restart {
            game = Some(Game::new_with_mode(GAME_WIDTH, GAME_HEIGHT, menu.selected_mode, menu.selected_speed));
            menu.should_restart = false;
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

        // 兼容某些平台/输入法将字母键作为文本事件而非键盘事件投递的情况
        if let Some(text) = event.text_args() {
            if menu.state == MenuState::Playing {
                if let Some(ref mut game) = game {
                    for ch in text.chars() {
                        match ch {
                            'w' | 'W' => game.key_pressed(Key::W),
                            'a' | 'A' => game.key_pressed(Key::A),
                            's' | 'S' => game.key_pressed(Key::S),
                            'd' | 'D' => game.key_pressed(Key::D),
                            _ => {}
                        }
                    }
                }
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
        
        window.draw_2d(&event, |c, g, device| {
            clear(BACK_COLOR, g);
            
            match menu.state {
                MenuState::Main | MenuState::ModeSelection | MenuState::SpeedSelection | MenuState::ConfirmStart => {
                    menu.draw(&c, g, &mut glyphs);
                }
                MenuState::Playing => {
                    if let Some(ref game) = game {
                        game.draw(&c, g);
                        // 绘制分数
                        menu.draw_score(game.get_score(), &c, g, &mut glyphs);
                        // 限时模式：绘制倒计时
                        if let Some(rt) = game.get_remaining_time() {
                            let min = (rt as i32) / 60;
                            let s = (rt as i32) % 60;
                            let time_text = format!("TIME {:02}:{:02}", min, s);
                            menu.draw_text_top_right(&time_text, 24.0, [1.0, 1.0, 0.0, 1.0], &c, g, &mut glyphs);
                        }
                        // 绘制暂停指示器
                        menu.draw_pause_indicator(&c, g, &mut glyphs);
                        // 绘制游戏消息
                        game.draw_messages(&c, g, &mut glyphs);
                    }
                    menu.draw(&c, g, &mut glyphs); // 绘制游戏内菜单按钮
                }
                MenuState::GameMenu => {
                    if let Some(ref game) = game {
                        game.draw(&c, g);
                    }
                    menu.draw(&c, g, &mut glyphs); // 绘制游戏内菜单
                }
                MenuState::GameOver => {
                    if let Some(ref game) = game {
                        game.draw(&c, g);
                    }
                    menu.draw(&c, g, &mut glyphs); // 绘制GameOver菜单
                }
            }
            // 刷新字体缓冲，避免只绘制首字符的问题
            glyphs.factory.encoder.flush(device);
        });
        
        event.update(|arg| {
            if menu.state == MenuState::Playing && !menu.is_paused {
                if let Some(ref mut game) = game {
                    game.update(arg.dt);
                    // 检查游戏是否结束
                    if game.is_game_over() {
                        menu.set_final_score(game.get_score());
                        menu.state = MenuState::GameOver;
                    }
                }
            }
        });
    }
}
