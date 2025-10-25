use piston_window::*;
use std::path::Path;

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
    Playing,
}

pub struct Menu {
    pub state: MenuState,
    pub selected_mode: GameMode,
    pub font: Option<rusttype::Font<'static>>,
    pub window_width: f64,
    pub window_height: f64,
}

impl Menu {
    pub fn new(window_width: f64, window_height: f64) -> Menu {
        Menu {
            state: MenuState::Main,
            selected_mode: GameMode::Classic,
            font: None,
            window_width,
            window_height,
        }
    }

    pub fn load_font(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let font_path = Path::new("assets/FiraSans-Regular.ttf");
        let font_data = std::fs::read(font_path)?;
        self.font = Some(rusttype::Font::try_from_vec(font_data).ok_or("Failed to load font")?);
        Ok(())
    }

    pub fn update_window_size(&mut self, new_width: f64, new_height: f64) {
        self.window_width = new_width;
        self.window_height = new_height;
    }

    pub fn handle_click(&mut self, x: f64, y: f64) {
        match self.state {
            MenuState::Main => {
                // 主页面按钮区域
                let center_x = self.window_width / 2.0;
                let center_y = self.window_height / 2.0;
                
                // 游戏开始按钮 (y: center_y - 45 到 center_y + 5)
                if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                   y >= center_y - 45.0 && y <= center_y + 5.0 {
                    self.state = MenuState::ModeSelection;
                }
                // 退出按钮 (y: center_y + 25 到 center_y + 75)
                else if x >= center_x - 100.0 && x <= center_x + 100.0 && 
                        y >= center_y + 25.0 && y <= center_y + 75.0 {
                    std::process::exit(0);
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
                    self.state = MenuState::Playing;
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
            MenuState::Playing => {
                // 游戏进行中，不处理点击
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
            MenuState::Playing => {
                // 游戏进行中，不处理菜单按键
            }
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        match self.state {
            MenuState::Main => self.draw_main_menu(con, g),
            MenuState::ModeSelection => self.draw_mode_selection(con, g),
            MenuState::Playing => {
                // 游戏进行中，不绘制菜单
            }
        }
    }

    fn draw_main_menu(&self, con: &Context, g: &mut G2d) {
        let center_x = self.window_width / 2.0;
        let center_y = self.window_height / 2.0;

        // 绘制背景
        rectangle(
            [0.1, 0.1, 0.1, 1.0], // 深色背景
            [0.0, 0.0, self.window_width, self.window_height],
            con.transform,
            g,
        );

        // 绘制标题
        self.draw_text("SNAKE GAME", center_x, center_y - 120.0, 36.0, [1.0, 1.0, 1.0, 1.0], con, g);

        // 绘制游戏开始按钮
        self.draw_button("START GAME", center_x, center_y - 20.0, 200.0, 50.0, [0.2, 0.6, 0.2, 1.0], con, g);

        // 绘制退出按钮
        self.draw_button("EXIT GAME", center_x, center_y + 50.0, 200.0, 50.0, [0.6, 0.2, 0.2, 1.0], con, g);
    }

    fn draw_mode_selection(&self, con: &Context, g: &mut G2d) {
        let center_x = self.window_width / 2.0;
        let center_y = self.window_height / 2.0;

        // 绘制背景
        rectangle(
            [0.1, 0.1, 0.1, 1.0], // 深色背景
            [0.0, 0.0, self.window_width, self.window_height],
            con.transform,
            g,
        );

        // 绘制标题
        self.draw_text("SELECT MODE", center_x, center_y - 120.0, 32.0, [1.0, 1.0, 1.0, 1.0], con, g);

        // 绘制模式按钮
        self.draw_button("CLASSIC", center_x, center_y - 40.0, 200.0, 50.0, [0.2, 0.4, 0.8, 1.0], con, g);
        self.draw_button("SPEED", center_x, center_y + 20.0, 200.0, 50.0, [0.8, 0.4, 0.2, 1.0], con, g);
        self.draw_button("SURVIVAL", center_x, center_y + 80.0, 200.0, 50.0, [0.8, 0.2, 0.8, 1.0], con, g);
        self.draw_button("BACK", center_x, center_y + 140.0, 200.0, 50.0, [0.4, 0.4, 0.4, 1.0], con, g);
    }

    fn draw_button(&self, text: &str, x: f64, y: f64, width: f64, height: f64, color: [f32; 4], con: &Context, g: &mut G2d) {
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

        // 绘制按钮文字
        self.draw_text(text, x, y, 18.0, [1.0, 1.0, 1.0, 1.0], con, g);
    }

    fn draw_text(&self, text: &str, x: f64, y: f64, size: f64, color: [f32; 4], con: &Context, g: &mut G2d) {
        // 使用简单的英文文本显示
        self.draw_english_text(text, x, y, size, color, con, g);
    }

    fn draw_english_text(&self, text: &str, x: f64, y: f64, size: f64, color: [f32; 4], con: &Context, g: &mut G2d) {
        let char_width = size * 0.5;
        let char_height = size;
        let start_x = x - (text.len() as f64 * char_width) / 2.0;
        
        for (i, ch) in text.chars().enumerate() {
            let char_x = start_x + i as f64 * char_width;
            self.draw_english_char(ch, char_x, y, char_width, char_height, color, con, g);
        }
    }

    fn draw_english_char(&self, ch: char, x: f64, y: f64, width: f64, height: f64, color: [f32; 4], con: &Context, g: &mut G2d) {
        // 使用简单的点阵字体显示英文字符
        let pixel_size = width / 5.0; // 5x7像素网格
        
        match ch {
            'S' => {
                let pattern = [
                    [0,1,1,1,0],
                    [1,0,0,0,0],
                    [0,1,1,1,0],
                    [0,0,0,0,1],
                    [1,1,1,1,0],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'N' => {
                let pattern = [
                    [1,0,0,0,1],
                    [1,1,0,0,1],
                    [1,0,1,0,1],
                    [1,0,0,1,1],
                    [1,0,0,0,1],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'A' => {
                let pattern = [
                    [0,1,1,1,0],
                    [1,0,0,0,1],
                    [1,1,1,1,1],
                    [1,0,0,0,1],
                    [1,0,0,0,1],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'K' => {
                let pattern = [
                    [1,0,0,1,0],
                    [1,0,1,0,0],
                    [1,1,0,0,0],
                    [1,0,1,0,0],
                    [1,0,0,1,0],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'E' => {
                let pattern = [
                    [1,1,1,1,1],
                    [1,0,0,0,0],
                    [1,1,1,1,0],
                    [1,0,0,0,0],
                    [1,1,1,1,1],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'G' => {
                let pattern = [
                    [0,1,1,1,0],
                    [1,0,0,0,0],
                    [1,0,1,1,1],
                    [1,0,0,0,1],
                    [0,1,1,1,0],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'M' => {
                let pattern = [
                    [1,0,0,0,1],
                    [1,1,0,1,1],
                    [1,0,1,0,1],
                    [1,0,0,0,1],
                    [1,0,0,0,1],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'T' => {
                let pattern = [
                    [1,1,1,1,1],
                    [0,0,1,0,0],
                    [0,0,1,0,0],
                    [0,0,1,0,0],
                    [0,0,1,0,0],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'R' => {
                let pattern = [
                    [1,1,1,1,0],
                    [1,0,0,0,1],
                    [1,1,1,1,0],
                    [1,0,1,0,0],
                    [1,0,0,1,0],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'I' => {
                let pattern = [
                    [1,1,1,1,1],
                    [0,0,1,0,0],
                    [0,0,1,0,0],
                    [0,0,1,0,0],
                    [1,1,1,1,1],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'X' => {
                let pattern = [
                    [1,0,0,0,1],
                    [0,1,0,1,0],
                    [0,0,1,0,0],
                    [0,1,0,1,0],
                    [1,0,0,0,1],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'C' => {
                let pattern = [
                    [0,1,1,1,0],
                    [1,0,0,0,0],
                    [1,0,0,0,0],
                    [1,0,0,0,0],
                    [0,1,1,1,0],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'L' => {
                let pattern = [
                    [1,0,0,0,0],
                    [1,0,0,0,0],
                    [1,0,0,0,0],
                    [1,0,0,0,0],
                    [1,1,1,1,1],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'D' => {
                let pattern = [
                    [1,1,1,1,0],
                    [1,0,0,0,1],
                    [1,0,0,0,1],
                    [1,0,0,0,1],
                    [1,1,1,1,0],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'P' => {
                let pattern = [
                    [1,1,1,1,0],
                    [1,0,0,0,1],
                    [1,1,1,1,0],
                    [1,0,0,0,0],
                    [1,0,0,0,0],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'U' => {
                let pattern = [
                    [1,0,0,0,1],
                    [1,0,0,0,1],
                    [1,0,0,0,1],
                    [1,0,0,0,1],
                    [0,1,1,1,0],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'V' => {
                let pattern = [
                    [1,0,0,0,1],
                    [1,0,0,0,1],
                    [0,1,0,1,0],
                    [0,1,0,1,0],
                    [0,0,1,0,0],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
                ];
                self.draw_pattern(pattern, x, y, pixel_size, color, con, g);
            },
            'B' => {
                let pattern = [
                    [1,1,1,1,0],
                    [1,0,0,0,1],
                    [1,1,1,1,0],
                    [1,0,0,0,1],
                    [1,1,1,1,0],
                    [0,0,0,0,0],
                    [0,0,0,0,0],
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

    fn draw_pattern(&self, pattern: [[u8; 5]; 7], x: f64, y: f64, pixel_size: f64, color: [f32; 4], con: &Context, g: &mut G2d) {
        for (row, row_data) in pattern.iter().enumerate() {
            for (col, &pixel) in row_data.iter().enumerate() {
                if pixel == 1 {
                    let pixel_x = x + col as f64 * pixel_size;
                    let pixel_y = y - 3.0 * pixel_size + row as f64 * pixel_size;
                    rectangle(
                        color,
                        [pixel_x, pixel_y, pixel_size, pixel_size],
                        con.transform,
                        g,
                    );
                }
            }
        }
    }
}
