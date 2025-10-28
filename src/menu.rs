use piston_window::*;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameSpeed {
    Slow,
    Medium,
    Fast,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameMode {
    Classic,
    Speed,
    Survival,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuState {
    Main,
    ModeSelection,
    SpeedSelection,
    ConfirmStart,
    Playing,
    GameMenu,
    GameOver,
}

pub struct Menu {
    pub state: MenuState,
    pub selected_mode: GameMode,
    pub selected_speed: GameSpeed,
    pub font: Option<rusttype::Font<'static>>,
    pub window_width: f64,
    pub window_height: f64,
    pub is_paused: bool,
    pub should_restart: bool,
    pub final_score: i32,
}

impl Menu {
    pub fn new(window_width: f64, window_height: f64) -> Menu {
        Menu {
            state: MenuState::Main,
            selected_mode: GameMode::Classic,
            selected_speed: GameSpeed::Medium,
            font: None,
            window_width,
            window_height,
            is_paused: false,
            should_restart: false,
            final_score: 0,
        }
    }

    pub fn load_font(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 依次尝试从项目根目录与 assets 目录加载字体
        let candidates = [
            Path::new("FiraSans-Regular.ttf"),
            Path::new("assets/FiraSans-Regular.ttf"),
        ];
        for path in candidates.iter() {
            if let Ok(font_data) = std::fs::read(path) {
                if let Some(font) = rusttype::Font::try_from_vec(font_data) {
                    self.font = Some(font);
                    return Ok(());
                }
            }
        }
        Err("Failed to load font from root or assets".into())
    }

    pub fn update_window_size(&mut self, new_width: f64, new_height: f64) {
        self.window_width = new_width;
        self.window_height = new_height;
    }
    
    pub fn set_final_score(&mut self, score: i32) {
        self.final_score = score;
    }

    pub fn handle_click(&mut self, x: f64, y: f64) {
        println!("Menu handle_click: ({}, {}) in state {:?}", x, y, self.state);
        match self.state {
            MenuState::Main => {
                // 主页面按钮区域
                let center_x = self.window_width / 2.0;
                let center_y = self.window_height / 2.0;
                
                println!("Window: {}x{}, center: ({}, {})", self.window_width, self.window_height, center_x, center_y);
                println!("START button area: x[{}, {}], y[{}, {}]", center_x - 150.0, center_x + 150.0, center_y - 80.0, center_y + 20.0);
                println!("EXIT button area: x[{}, {}], y[{}, {}]", center_x - 150.0, center_x + 150.0, center_y + 10.0, center_y + 110.0);
                
                // 游戏开始按钮 (扩大点击区域进行测试)
                if x >= center_x - 150.0 && x <= center_x + 150.0 && 
                   y >= center_y - 80.0 && y <= center_y + 20.0 {
                    println!("START GAME button clicked!");
                    self.state = MenuState::ModeSelection;
                }
                // 退出按钮 (扩大点击区域进行测试)
                else if x >= center_x - 150.0 && x <= center_x + 150.0 && 
                        y >= center_y + 10.0 && y <= center_y + 110.0 {
                    println!("EXIT GAME button clicked!");
                    std::process::exit(0);
                } else {
                    println!("Click not in any button area");
                }
            }
            MenuState::ModeSelection => {
                let center_x = self.window_width / 2.0;
                let center_y = self.window_height / 2.0;
                
                // 经典模式按钮
                if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                   y >= center_y - 65.0 && y <= center_y - 15.0 {
                    self.selected_mode = GameMode::Classic;
                    self.state = MenuState::Playing;
                }
                // 速度模式按钮
                else if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                        y >= center_y - 5.0 && y <= center_y + 45.0 {
                    self.selected_mode = GameMode::Speed;
                    self.state = MenuState::SpeedSelection;
                }
                // 生存模式按钮
                else if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                        y >= center_y + 55.0 && y <= center_y + 105.0 {
                    self.selected_mode = GameMode::Survival;
                    self.state = MenuState::Playing;
                }
                // 返回按钮
                else if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                        y >= center_y + 115.0 && y <= center_y + 165.0 {
                    self.state = MenuState::Main;
                }
            }
            MenuState::SpeedSelection => {
                let center_x = self.window_width / 2.0;
                let center_y = self.window_height / 2.0;
                // 慢速
                if x >= center_x - 180.0 && x <= center_x - 60.0 &&
                   y >= center_y - 20.0 && y <= center_y + 20.0 {
                    self.selected_speed = GameSpeed::Slow;
                    self.state = MenuState::ConfirmStart;
                }
                // 中速
                else if x >= center_x - 40.0 && x <= center_x + 40.0 &&
                        y >= center_y - 20.0 && y <= center_y + 20.0 {
                    self.selected_speed = GameSpeed::Medium;
                    self.state = MenuState::ConfirmStart;
                }
                // 快速
                else if x >= center_x + 60.0 && x <= center_x + 180.0 &&
                        y >= center_y - 20.0 && y <= center_y + 20.0 {
                    self.selected_speed = GameSpeed::Fast;
                    self.state = MenuState::ConfirmStart;
                }
                // 返回
                else if x >= center_x - 100.0 && x <= center_x + 100.0 &&
                        y >= center_y + 60.0 && y <= center_y + 100.0 {
                    self.state = MenuState::ModeSelection;
                }
            }
            MenuState::ConfirmStart => {
                let center_x = self.window_width / 2.0;
                let center_y = self.window_height / 2.0;
                // YES
                if x >= center_x - 90.0 && x <= center_x - 10.0 &&
                   y >= center_y + 20.0 && y <= center_y + 60.0 {
                    self.state = MenuState::Playing;
                }
                // NO
                else if x >= center_x + 10.0 && x <= center_x + 90.0 &&
                        y >= center_y + 20.0 && y <= center_y + 60.0 {
                    self.state = MenuState::SpeedSelection;
                }
            }
            MenuState::Playing => {
                // 游戏进行中，检查是否点击了菜单按钮
                let menu_button_x = self.window_width - 60.0;
                let menu_button_y = 30.0;
                if x >= menu_button_x - 20.0 && x <= menu_button_x + 20.0 && 
                   y >= menu_button_y - 15.0 && y <= menu_button_y + 15.0 {
                    self.state = MenuState::GameMenu;
                }
            }
            MenuState::GameMenu => {
                let center_x = self.window_width / 2.0;
                let center_y = self.window_height / 2.0;
                
                // 暂停/继续按钮 (y: center_y - 30 到 center_y + 10)
                if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                   y >= center_y - 30.0 && y <= center_y + 10.0 {
                    self.is_paused = !self.is_paused;
                    self.state = MenuState::Playing;
                }
                // 重新开始按钮 (y: center_y + 20 到 center_y + 60)
                else if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                        y >= center_y + 20.0 && y <= center_y + 60.0 {
                    // 重新开始游戏
                    self.is_paused = false;
                    self.should_restart = true;
                    self.state = MenuState::Playing;
                }
                // 返回主菜单按钮 (y: center_y + 70 到 center_y + 110)
                else if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                        y >= center_y + 70.0 && y <= center_y + 110.0 {
                    self.state = MenuState::Main;
                }
                // 关闭菜单按钮 (y: center_y + 120 到 center_y + 160)
                else if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                        y >= center_y + 120.0 && y <= center_y + 160.0 {
                    self.state = MenuState::Playing;
                }
            }
            MenuState::GameOver => {
                let center_x = self.window_width / 2.0;
                let center_y = self.window_height / 2.0;
                
                // 重新开始按钮 (y: center_y - 20 到 center_y + 20)
                if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                   y >= center_y - 20.0 && y <= center_y + 20.0 {
                    // 重新开始游戏
                    self.is_paused = false;
                    self.should_restart = true;
                    self.state = MenuState::Playing;
                }
                // 返回主菜单按钮 (y: center_y + 40 到 center_y + 80)
                else if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                        y >= center_y + 40.0 && y <= center_y + 80.0 {
                    self.state = MenuState::Main;
                }
            }
        }
    }

    pub fn handle_key(&mut self, key: Key) {
        match self.state {
            MenuState::Main => {
                if key == Key::Escape {
                    std::process::exit(0);
                }
            }
            MenuState::ModeSelection => {
                if key == Key::Escape {
                    self.state = MenuState::Main;
                }
            }
            MenuState::SpeedSelection => {
                if key == Key::Escape {
                    self.state = MenuState::ModeSelection;
                }
            }
            MenuState::ConfirmStart => {
                if key == Key::Escape {
                    self.state = MenuState::SpeedSelection;
                }
            }
            MenuState::Playing => {
                if key == Key::Escape {
                    self.state = MenuState::GameMenu;
                }
            }
            MenuState::GameMenu => {
                if key == Key::Escape {
                    self.state = MenuState::Playing;
                }
            }
            MenuState::GameOver => {
                if key == Key::Escape {
                    self.state = MenuState::Main;
                }
            }
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        match self.state {
            MenuState::Main => self.draw_main_menu(con, g, glyphs),
            MenuState::ModeSelection => self.draw_mode_selection(con, g, glyphs),
            MenuState::SpeedSelection => self.draw_speed_selection(con, g, glyphs),
            MenuState::ConfirmStart => self.draw_confirm_start(con, g, glyphs),
            MenuState::Playing => {
                // 游戏进行中，绘制菜单按钮
                self.draw_game_menu_button(con, g);
            }
            MenuState::GameMenu => {
                // 绘制游戏内菜单
                self.draw_game_menu(con, g, glyphs);
            }
            MenuState::GameOver => {
                // 绘制GameOver菜单
                self.draw_game_over_menu(con, g, glyphs);
            }
        }
    }

    fn draw_main_menu(&self, con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        let center_x = self.window_width / 2.0;
        let center_y = self.window_height / 2.0;

        // 绘制背景
        rectangle(
            [0.1, 0.1, 0.1, 1.0], // 深色背景
            [0.0, 0.0, self.window_width, self.window_height],
            con.transform,
            g,
        );

        // 绘制标题（使用字体）
        self.draw_text_glyph("SNAKE GAME", center_x + 2.0, center_y - 118.0, 48, [0.0, 0.0, 0.0, 0.8], con, g, glyphs);
        self.draw_text_glyph("SNAKE GAME", center_x, center_y - 120.0, 48, [1.0, 1.0, 1.0, 1.0], con, g, glyphs);

        // 绘制游戏开始按钮
        self.draw_button_glyph("START GAME", center_x, center_y - 20.0, 200.0, 50.0, [0.2, 0.6, 0.2, 1.0], con, g, glyphs);

        // 绘制退出按钮
        self.draw_button_glyph("EXIT GAME", center_x, center_y + 50.0, 200.0, 50.0, [0.6, 0.2, 0.2, 1.0], con, g, glyphs);
    }

    fn draw_mode_selection(&self, con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        let center_x = self.window_width / 2.0;
        let center_y = self.window_height / 2.0;

        // 绘制背景
        rectangle(
            [0.1, 0.1, 0.1, 1.0], // 深色背景
            [0.0, 0.0, self.window_width, self.window_height],
            con.transform,
            g,
        );

        self.draw_text_glyph("SELECT MODE", center_x + 2.0, center_y - 118.0, 40, [0.0, 0.0, 0.0, 0.8], con, g, glyphs);
        self.draw_text_glyph("SELECT MODE", center_x, center_y - 120.0, 40, [1.0, 1.0, 1.0, 1.0], con, g, glyphs);

        // 绘制模式按钮
        self.draw_button_glyph("CLASSIC", center_x, center_y - 40.0, 200.0, 50.0, [0.2, 0.4, 0.8, 1.0], con, g, glyphs);
        self.draw_button_glyph("SPEED", center_x, center_y + 20.0, 200.0, 50.0, [0.8, 0.4, 0.2, 1.0], con, g, glyphs);
        self.draw_button_glyph("SURVIVAL", center_x, center_y + 80.0, 200.0, 50.0, [0.8, 0.2, 0.8, 1.0], con, g, glyphs);
        self.draw_button_glyph("BACK", center_x, center_y + 140.0, 200.0, 50.0, [0.4, 0.4, 0.4, 1.0], con, g, glyphs);
    }

    fn draw_speed_selection(&self, con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        let center_x = self.window_width / 2.0;
        let center_y = self.window_height / 2.0;

        // 背景
        rectangle(
            [0.1, 0.1, 0.1, 1.0],
            [0.0, 0.0, self.window_width, self.window_height],
            con.transform,
            g,
        );

        self.draw_text_glyph("SELECT SPEED", center_x, center_y - 80.0, 36, [1.0, 1.0, 1.0, 1.0], con, g, glyphs);

        let slow_color = if self.selected_speed == GameSpeed::Slow { [0.2, 0.8, 0.2, 1.0] } else { [0.2, 0.6, 0.2, 1.0] };
        let medium_color = if self.selected_speed == GameSpeed::Medium { [0.2, 0.8, 0.2, 1.0] } else { [0.2, 0.6, 0.2, 1.0] };
        let fast_color = if self.selected_speed == GameSpeed::Fast { [0.2, 0.8, 0.2, 1.0] } else { [0.2, 0.6, 0.2, 1.0] };
        self.draw_button_glyph("SLOW", center_x - 120.0, center_y, 120.0, 40.0, slow_color, con, g, glyphs);
        self.draw_button_glyph("MEDIUM", center_x, center_y, 120.0, 40.0, medium_color, con, g, glyphs);
        self.draw_button_glyph("FAST", center_x + 120.0, center_y, 120.0, 40.0, fast_color, con, g, glyphs);

        self.draw_button_glyph("BACK", center_x, center_y + 100.0, 200.0, 40.0, [0.4, 0.4, 0.4, 1.0], con, g, glyphs);
    }

    fn draw_confirm_start(&self, con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        let center_x = self.window_width / 2.0;
        let center_y = self.window_height / 2.0;

        // 半透明遮罩
        rectangle(
            [0.0, 0.0, 0.0, 0.7],
            [0.0, 0.0, self.window_width, self.window_height],
            con.transform,
            g,
        );

        // 对话框
        rectangle(
            [0.15, 0.15, 0.15, 1.0],
            [center_x - 160.0, center_y - 80.0, 320.0, 160.0],
            con.transform,
            g,
        );

        self.draw_text_glyph("Start with:", center_x, center_y - 40.0, 24, [1.0, 1.0, 1.0, 1.0], con, g, glyphs);
        let speed_text = match self.selected_speed {
            GameSpeed::Slow => "SLOW",
            GameSpeed::Medium => "MEDIUM",
            GameSpeed::Fast => "FAST",
        };
        self.draw_text_glyph(speed_text, center_x, center_y - 10.0, 28, [1.0, 1.0, 0.0, 1.0], con, g, glyphs);

        self.draw_button_glyph("YES", center_x - 50.0, center_y + 40.0, 80.0, 40.0, [0.2, 0.8, 0.2, 1.0], con, g, glyphs);
        self.draw_button_glyph("NO", center_x + 50.0, center_y + 40.0, 80.0, 40.0, [0.8, 0.2, 0.2, 1.0], con, g, glyphs);
    }

    fn draw_game_menu_button(&self, con: &Context, g: &mut G2d) {
        let menu_button_x = self.window_width - 60.0;
        let menu_button_y = 30.0;
        
        // 绘制菜单按钮背景
        rectangle(
            [0.2, 0.2, 0.2, 0.8], // 半透明深色背景
            [menu_button_x - 20.0, menu_button_y - 15.0, 40.0, 30.0],
            con.transform,
            g,
        );
        
        // 绘制菜单按钮边框
        rectangle(
            [1.0, 1.0, 1.0, 1.0], // 白色边框
            [menu_button_x - 20.0, menu_button_y - 15.0, 2.0, 30.0],
            con.transform,
            g,
        );
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [menu_button_x + 18.0, menu_button_y - 15.0, 2.0, 30.0],
            con.transform,
            g,
        );
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [menu_button_x - 20.0, menu_button_y - 15.0, 40.0, 2.0],
            con.transform,
            g,
        );
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [menu_button_x - 20.0, menu_button_y + 13.0, 40.0, 2.0],
            con.transform,
            g,
        );
        
        // 绘制菜单图标（三条横线）
        let line_y1 = menu_button_y - 8.0;
        let line_y2 = menu_button_y;
        let line_y3 = menu_button_y + 8.0;
        let line_x1 = menu_button_x - 12.0;
        let line_x2 = menu_button_x + 12.0;
        
        // 第一条线
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [line_x1, line_y1 - 1.0, line_x2 - line_x1, 2.0],
            con.transform,
            g,
        );
        // 第二条线
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [line_x1, line_y2 - 1.0, line_x2 - line_x1, 2.0],
            con.transform,
            g,
        );
        // 第三条线
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [line_x1, line_y3 - 1.0, line_x2 - line_x1, 2.0],
            con.transform,
            g,
        );
    }
    
    pub fn draw_score(&self, score: i32, con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        let score_text = format!("SCORE: {}", score);
        self.draw_text_glyph(&score_text, 100.0, 30.0, 24, [1.0, 1.0, 1.0, 1.0], con, g, glyphs);
    }

    pub fn draw_text_top_right(&self, text: &str, size_px: f64, color: [f32; 4], con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        use piston_window::character::CharacterCache;
        let spx = size_px as u32;
        let mut total_w = 0.0;
        for ch in text.chars() {
            if let Ok(g) = glyphs.character(spx, ch) { total_w += g.advance_width(); }
        }
        let margin_right = 20.0;
        let x_right = self.window_width - margin_right;
        // 通过中心绘制，计算文本中心点位置
        let x_center = x_right - total_w / 2.0;
        let y = 30.0;
        self.draw_text_glyph(text, x_center, y, spx, color, con, g, glyphs);
    }
    
    pub fn draw_pause_indicator(&self, con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        if self.is_paused {
            let center_x = self.window_width / 2.0;
            let center_y = self.window_height / 2.0;
            
            // 绘制半透明背景
            rectangle(
                [0.0, 0.0, 0.0, 0.5], // 半透明黑色背景
                [0.0, 0.0, self.window_width, self.window_height],
                con.transform,
                g,
            );
            
            // 绘制暂停文本
            self.draw_text_glyph("PAUSED", center_x, center_y, 48, [1.0, 1.0, 0.0, 1.0], con, g, glyphs);
            self.draw_text_glyph("Press ESC to resume", center_x, center_y + 50.0, 22, [0.8, 0.8, 0.8, 1.0], con, g, glyphs);
        }
    }

    pub fn draw_controls_help(&self, con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        // 在游戏区域外（右侧边距）显示操作说明
        let panel_w = 230.0;
        let panel_h = 160.0;
        let x = self.window_width - panel_w - 20.0;
        let y = 60.0;

        // 背景面板
        rectangle(
            [0.0, 0.0, 0.0, 0.35],
            [x, y, panel_w, panel_h],
            con.transform,
            g,
        );

        // 标题与内容
        self.draw_text_glyph("CONTROLS", x + panel_w / 2.0, y + 22.0, 22, [1.0, 1.0, 1.0, 1.0], con, g, glyphs);

        let line1 = "Move: Arrow / WASD";
        let line2 = "Menu: ESC or button";
        let line3 = "Click: UI buttons";
        let mut ty = y + 52.0;
        let lh = 20.0;
        self.draw_text_glyph(line1, x + panel_w / 2.0, ty, 18, [0.9, 0.9, 0.9, 1.0], con, g, glyphs);
        ty += lh;
        self.draw_text_glyph(line2, x + panel_w / 2.0, ty, 18, [0.9, 0.9, 0.9, 1.0], con, g, glyphs);
        ty += lh;
        self.draw_text_glyph(line3, x + panel_w / 2.0, ty, 18, [0.9, 0.9, 0.9, 1.0], con, g, glyphs);
    }

    fn draw_game_menu(&self, con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        let center_x = self.window_width / 2.0;
        let center_y = self.window_height / 2.0;

        // 绘制半透明背景
        rectangle(
            [0.0, 0.0, 0.0, 0.7], // 半透明黑色背景
            [0.0, 0.0, self.window_width, self.window_height],
            con.transform,
            g,
        );

        // 绘制菜单背景
        rectangle(
            [0.1, 0.1, 0.1, 0.95], // 深色背景
            [center_x - 150.0, center_y - 120.0, 300.0, 240.0],
            con.transform,
            g,
        );

        // 绘制菜单边框（更粗的边框）
        let border_width = 3.0;
        rectangle(
            [0.8, 0.8, 0.8, 1.0], // 浅灰色边框
            [center_x - 150.0, center_y - 120.0, border_width, 240.0],
            con.transform,
            g,
        );
        rectangle(
            [0.8, 0.8, 0.8, 1.0],
            [center_x + 147.0, center_y - 120.0, border_width, 240.0],
            con.transform,
            g,
        );
        rectangle(
            [0.8, 0.8, 0.8, 1.0],
            [center_x - 150.0, center_y - 120.0, 300.0, border_width],
            con.transform,
            g,
        );
        rectangle(
            [0.8, 0.8, 0.8, 1.0],
            [center_x - 150.0, center_y + 117.0, 300.0, border_width],
            con.transform,
            g,
        );

        // 绘制菜单标题
        self.draw_text_glyph("GAME MENU", center_x, center_y - 80.0, 32, [1.0, 1.0, 1.0, 1.0], con, g, glyphs);

        // 绘制菜单按钮
        let pause_text = if self.is_paused { "RESUME" } else { "PAUSE" };
        let pause_color = if self.is_paused { [0.2, 0.8, 0.2, 1.0] } else { [0.8, 0.6, 0.2, 1.0] };
        self.draw_button_glyph(pause_text, center_x, center_y - 30.0, 200.0, 40.0, pause_color, con, g, glyphs);
        self.draw_button_glyph("RESTART", center_x, center_y + 20.0, 200.0, 40.0, [0.6, 0.4, 0.2, 1.0], con, g, glyphs);
        self.draw_button_glyph("MAIN MENU", center_x, center_y + 70.0, 200.0, 40.0, [0.6, 0.2, 0.2, 1.0], con, g, glyphs);
        self.draw_button_glyph("CLOSE", center_x, center_y + 120.0, 200.0, 40.0, [0.4, 0.4, 0.4, 1.0], con, g, glyphs);
    }

    fn draw_game_over_menu(&self, con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        let center_x = self.window_width / 2.0;
        let center_y = self.window_height / 2.0;

        // 绘制半透明背景
        rectangle(
            [0.0, 0.0, 0.0, 0.8], // 半透明黑色背景
            [0.0, 0.0, self.window_width, self.window_height],
            con.transform,
            g,
        );

        // 绘制菜单背景
        rectangle(
            [0.1, 0.1, 0.1, 0.95], // 深色背景
            [center_x - 150.0, center_y - 100.0, 300.0, 200.0],
            con.transform,
            g,
        );

        // 绘制菜单边框
        let border_width = 3.0;
        rectangle(
            [0.8, 0.0, 0.0, 1.0], // 红色边框
            [center_x - 150.0, center_y - 100.0, border_width, 200.0],
            con.transform,
            g,
        );
        rectangle(
            [0.8, 0.0, 0.0, 1.0],
            [center_x + 147.0, center_y - 100.0, border_width, 200.0],
            con.transform,
            g,
        );
        rectangle(
            [0.8, 0.0, 0.0, 1.0],
            [center_x - 150.0, center_y - 100.0, 300.0, border_width],
            con.transform,
            g,
        );
        rectangle(
            [0.8, 0.0, 0.0, 1.0],
            [center_x - 150.0, center_y + 97.0, 300.0, border_width],
            con.transform,
            g,
        );

        // 绘制GameOver标题
        self.draw_text_glyph("GAME OVER", center_x + 2.0, center_y - 58.0, 36, [0.0, 0.0, 0.0, 0.8], con, g, glyphs);
        self.draw_text_glyph("GAME OVER", center_x, center_y - 60.0, 36, [1.0, 0.0, 0.0, 1.0], con, g, glyphs);
        
        // 绘制最终分数
        let score_text = format!("FINAL SCORE: {}", self.final_score);
        self.draw_text_glyph(&score_text, center_x, center_y - 20.0, 24, [1.0, 1.0, 1.0, 1.0], con, g, glyphs);

        // 绘制菜单按钮
        self.draw_button_glyph("PLAY AGAIN", center_x, center_y + 20.0, 200.0, 40.0, [0.2, 0.8, 0.2, 1.0], con, g, glyphs);
        self.draw_button_glyph("MAIN MENU", center_x, center_y + 70.0, 200.0, 40.0, [0.6, 0.2, 0.2, 1.0], con, g, glyphs);
    }

    fn draw_button_glyph(&self, text: &str, x: f64, y: f64, width: f64, height: f64, color: [f32; 4], con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        // 绘制按钮背景
        rectangle(
            color,
            [x - width/2.0, y - height/2.0, width, height],
            con.transform,
            g,
        );

        // 绘制按钮边框（细边框）
        let border_width = 2.0;
        rectangle(
            [1.0, 1.0, 1.0, 1.0], // 白色边框
            [x - width/2.0, y - height/2.0, border_width, height],
            con.transform,
            g,
        );
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [x + width/2.0 - border_width, y - height/2.0, border_width, height],
            con.transform,
            g,
        );
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [x - width/2.0, y - height/2.0, width, border_width],
            con.transform,
            g,
        );
        rectangle(
            [1.0, 1.0, 1.0, 1.0],
            [x - width/2.0, y + height/2.0 - border_width, width, border_width],
            con.transform,
            g,
        );

        // 绘制按钮文字（使用字体）
        self.draw_text_glyph(text, x + 1.0, y + 1.0, 22, [0.0, 0.0, 0.0, 0.8], con, g, glyphs);
        self.draw_text_glyph(text, x, y, 22, [1.0, 1.0, 1.0, 1.0], con, g, glyphs);
    }

    fn draw_text_glyph(&self, text: &str, x: f64, y: f64, size_px: u32, color: [f32; 4], con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        use piston_window::character::CharacterCache;
        let total_w = glyphs.width(size_px, text).unwrap_or(0.0);
        let baseline_adjust = (size_px as f64) * 0.35;
        let transform = con.transform.trans(x - total_w / 2.0, y + baseline_adjust);
        let txt = piston_window::Text::new_color(color, size_px);
        let _ = txt.draw(text, glyphs, &con.draw_state, transform, g);
    }

    fn draw_text(&self, text: &str, x: f64, y: f64, size: f64, color: [f32; 4], con: &Context, g: &mut G2d) {
        // 使用简单的英文文本显示
        self.draw_english_text(text, x, y, size, color, con, g);
    }

    fn draw_english_text(&self, text: &str, x: f64, y: f64, size: f64, color: [f32; 4], con: &Context, g: &mut G2d) {
        let char_width = size * 0.6; // 增加字符宽度，让文字更清晰
        let char_height = size;
        let start_x = x - (text.len() as f64 * char_width) / 2.0;
        
        for (i, ch) in text.chars().enumerate() {
            let char_x = start_x + i as f64 * char_width;
            // 调整字符的垂直位置，让文字更好地居中
            let char_y = y - char_height * 0.1;
            self.draw_english_char(ch, char_x, char_y, char_width, char_height, color, con, g);
        }
    }

    fn draw_english_char(&self, ch: char, x: f64, y: f64, width: f64, height: f64, color: [f32; 4], con: &Context, g: &mut G2d) {
        // 使用简单的点阵字体显示英文字符，增加像素大小以提高清晰度
        let pixel_size = width / 4.0; // 改为4x6像素网格，像素更大更清晰
        
        match ch {
            'S' => {
                let pattern = [
                    [0,1,1,0],
                    [1,0,0,0],
                    [0,1,1,0],
                    [0,0,0,1],
                    [1,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'N' => {
                let pattern = [
                    [1,0,0,1],
                    [1,1,0,1],
                    [1,0,1,1],
                    [1,0,0,1],
                    [1,0,0,1],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'A' => {
                let pattern = [
                    [0,1,1,0],
                    [1,0,0,1],
                    [1,1,1,1],
                    [1,0,0,1],
                    [1,0,0,1],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'K' => {
                let pattern = [
                    [1,0,0,1],
                    [1,0,1,0],
                    [1,1,0,0],
                    [1,0,1,0],
                    [1,0,0,1],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'E' => {
                let pattern = [
                    [1,1,1,1],
                    [1,0,0,0],
                    [1,1,1,0],
                    [1,0,0,0],
                    [1,1,1,1],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'G' => {
                let pattern = [
                    [0,1,1,0],
                    [1,0,0,0],
                    [1,0,1,1],
                    [1,0,0,1],
                    [0,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'M' => {
                let pattern = [
                    [1,0,0,1],
                    [1,1,1,1],
                    [1,0,0,1],
                    [1,0,0,1],
                    [1,0,0,1],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'T' => {
                let pattern = [
                    [1,1,1,1],
                    [0,1,1,0],
                    [0,1,1,0],
                    [0,1,1,0],
                    [0,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'R' => {
                let pattern = [
                    [1,1,1,0],
                    [1,0,0,1],
                    [1,1,1,0],
                    [1,0,1,0],
                    [1,0,0,1],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'I' => {
                let pattern = [
                    [1,1,1,1],
                    [0,1,1,0],
                    [0,1,1,0],
                    [0,1,1,0],
                    [1,1,1,1],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'X' => {
                let pattern = [
                    [1,0,0,1],
                    [0,1,1,0],
                    [0,0,1,0],
                    [0,1,1,0],
                    [1,0,0,1],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'C' => {
                let pattern = [
                    [0,1,1,0],
                    [1,0,0,0],
                    [1,0,0,0],
                    [1,0,0,0],
                    [0,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'L' => {
                let pattern = [
                    [1,0,0,0],
                    [1,0,0,0],
                    [1,0,0,0],
                    [1,0,0,0],
                    [1,1,1,1],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'D' => {
                let pattern = [
                    [1,1,1,0],
                    [1,0,0,1],
                    [1,0,0,1],
                    [1,0,0,1],
                    [1,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'P' => {
                let pattern = [
                    [1,1,1,0],
                    [1,0,0,1],
                    [1,1,1,0],
                    [1,0,0,0],
                    [1,0,0,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'U' => {
                let pattern = [
                    [1,0,0,1],
                    [1,0,0,1],
                    [1,0,0,1],
                    [1,0,0,1],
                    [0,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'V' => {
                let pattern = [
                    [1,0,0,1],
                    [1,0,0,1],
                    [0,1,1,0],
                    [0,1,1,0],
                    [0,0,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'B' => {
                let pattern = [
                    [1,1,1,0],
                    [1,0,0,1],
                    [1,1,1,0],
                    [1,0,0,1],
                    [1,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            '0' => {
                let pattern = [
                    [0,1,1,0],
                    [1,0,0,1],
                    [1,0,0,1],
                    [1,0,0,1],
                    [0,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            '1' => {
                let pattern = [
                    [0,1,0,0],
                    [1,1,0,0],
                    [0,1,0,0],
                    [0,1,0,0],
                    [1,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            '2' => {
                let pattern = [
                    [0,1,1,0],
                    [0,0,0,1],
                    [0,1,1,0],
                    [1,0,0,0],
                    [1,1,1,1],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            '3' => {
                let pattern = [
                    [0,1,1,0],
                    [0,0,0,1],
                    [0,1,1,0],
                    [0,0,0,1],
                    [0,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            '4' => {
                let pattern = [
                    [1,0,1,0],
                    [1,0,1,0],
                    [1,1,1,1],
                    [0,0,1,0],
                    [0,0,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            '5' => {
                let pattern = [
                    [1,1,1,1],
                    [1,0,0,0],
                    [1,1,1,0],
                    [0,0,0,1],
                    [1,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            '6' => {
                let pattern = [
                    [0,1,1,0],
                    [1,0,0,0],
                    [1,1,1,0],
                    [1,0,0,1],
                    [0,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            '7' => {
                let pattern = [
                    [1,1,1,1],
                    [0,0,0,1],
                    [0,0,1,0],
                    [0,1,0,0],
                    [1,0,0,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            '8' => {
                let pattern = [
                    [0,1,1,0],
                    [1,0,0,1],
                    [0,1,1,0],
                    [1,0,0,1],
                    [0,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            '9' => {
                let pattern = [
                    [0,1,1,0],
                    [1,0,0,1],
                    [0,1,1,1],
                    [0,0,0,1],
                    [0,1,1,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            ':' => {
                let pattern = [
                    [0,0,0,0],
                    [0,1,1,0],
                    [0,0,0,0],
                    [0,1,1,0],
                    [0,0,0,0],
                    [0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            ' ' => {
                // 空格字符，不绘制任何内容
            },
            _ => {
                // 对于其他字符，绘制一个简单的方块
                rectangle(
                    color,
                    [x, y - height/2.0, width * 0.8, height],
                    con.transform,
                    g,
                );
            }
        }
    }

    fn draw_pattern(&self, pattern: [[u8; 4]; 6], x: f64, y: f64, pixel_size: f64, color: [f32; 4], con: &Context, g: &mut G2d) {
        for (row, row_data) in pattern.iter().enumerate() {
            for (col, &pixel) in row_data.iter().enumerate() {
                if pixel == 1 {
                    let pixel_x = x + col as f64 * pixel_size;
                    let pixel_y = y - 2.5 * pixel_size + row as f64 * pixel_size;
                    // 增加像素大小，让文字更清晰
                    let pixel_size_with_padding = pixel_size * 1.1;
                    rectangle(
                        color,
                        [pixel_x - 0.05 * pixel_size, pixel_y - 0.05 * pixel_size, pixel_size_with_padding, pixel_size_with_padding],
                        con.transform,
                        g,
                    );
                }
            }
        }
    }
}

// 增加供其他模块使用的简单文本显示
pub fn draw_simple_text(text: &str, x: f64, y: f64, size: f64, color: [f32; 4], con: &Context, g: &mut G2d) {
    // 用 menu 中自带的字体渲染
    // 这里只调用 draw_english_text，即原 draw_text 的底层
    let char_width = size * 0.6;
    let char_height = size;
    let start_x = x - (text.len() as f64 * char_width) / 2.0;
    for (i, ch) in text.chars().enumerate() {
        let char_x = start_x + i as f64 * char_width;
        let char_y = y - char_height * 0.1;
        Menu { state: crate::menu::MenuState::Main, selected_mode: crate::menu::GameMode::Classic, selected_speed: crate::menu::GameSpeed::Medium, font: None, window_width: 800.0, window_height: 600.0, is_paused: false, should_restart: false, final_score: 0 }
            .draw_english_char(ch, char_x, char_y, char_width, char_height, color, con, g);
    }
}
