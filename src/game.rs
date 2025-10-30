use crate::draw::{draw_block_dynamic_with_offset, draw_rectangle_dynamic_with_offset};
use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};
use crate::snake::{Direction, Snake};
use crate::menu::{GameMode, GameSpeed};
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BIG_FOOD_COLOR: Color = [1.0, 0.5, 0.0, 1.0]; // 黄色的大食物
const BORDER_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];
const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;
const TIME_LIMIT_SECONDS: i32 = 30; // 初始30秒
const TIME_ADD_PER_FOOD: i32 = 10;  // 每吃一个加10秒
const BIG_FOOD_SPAWN_INTERVAL: f64 = 8.0; // 大食物出现倒计时
const BIG_FOOD_LIFETIME: f64 = 5.0; //大食物消失倒计时
const MESSAGE_DISPLAY_TIME: f64 = 5.0; // 信息显示时间

struct GameMessage {
    text: String,
    lifetime: f64,
}

impl GameMessage {
    fn new(text: String) -> Self {
        Self {
            text,
            lifetime: MESSAGE_DISPLAY_TIME,
        }
    }
}

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    big_food_exists: bool,
    big_food_x: i32,
    big_food_y: i32,
    big_food_timer: f64, // 大食物生成计时器
    big_food_lifetime: f64, // 当前大食物的存活时间
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
    window_width: f64,
    window_height: f64,
    block_size: f64,
    offset_x: f64,
    offset_y: f64,
    game_mode: GameMode,
    score: i32,
    speed_multiplier: f64,
    speed_setting: GameSpeed,
    remaining_time: Option<f64>, // Survival模式剩余时间，秒
    messages: Vec<GameMessage>, // 在右侧显示的消息
}
impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Self::new_with_mode(width, height, GameMode::Classic, GameSpeed::Medium)
    }
    
    pub fn new_with_mode(width: i32, height: i32, mode: GameMode, speed: GameSpeed) -> Game {
        let initial_window_width = (width * 25) as f64; // 25是区块大小
        let initial_window_height = (height * 25) as f64;
        let speed_multiplier = match mode {
            GameMode::Classic => 1.0,
            GameMode::Speed => 1.5,
            GameMode::Survival => 0.8,
        };
        let mut g = Game {
            snake: Snake::new(2, 2),
            food_exists: true,
            food_x: 6,
            food_y: 4,
            big_food_exists: false,
            big_food_x: 0,
            big_food_y: 0,
            big_food_timer: 0.0,
            big_food_lifetime: 0.0,
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
            window_width: initial_window_width,
            window_height: initial_window_height,
            block_size: 25.0,
            offset_x: 0.0,
            offset_y: 0.0,
            game_mode: mode,
            score: 0,
            speed_multiplier,
            speed_setting: speed,
            remaining_time: None,
            messages: Vec::new(),
        };
        if mode == GameMode::Survival {
            g.remaining_time = Some(TIME_LIMIT_SECONDS as f64);
            g.speed_setting = GameSpeed::Fast; // 限时模式速度始终最快
            g.speed_multiplier = 1.5; // 彻底快
        }
        g
    }
    
    pub fn update_window_size(&mut self, new_width: f64, new_height: f64) {
        self.window_width = new_width;
        self.window_height = new_height;
        // 计算新的块大小，让游戏区域完全填满窗口
        // 分别计算宽度和高度的缩放比例
        let scale_x = new_width / self.width as f64;
        let scale_y = new_height / self.height as f64;
        // 使用较小的缩放比例来保持宽高比，但让游戏区域尽可能填满窗口
        self.block_size = scale_x.min(scale_y);
        
        // 计算偏移量来居中显示游戏区域
        let game_width = self.width as f64 * self.block_size;
        let game_height = self.height as f64 * self.block_size;
        self.offset_x = (new_width - game_width) / 2.0;
        self.offset_y = (new_height - game_height) / 2.0;
    }
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::W => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::S => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::A => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            Key::D => Some(Direction::Right),
            _ => None,
        };
        if let Some(d) = dir {
            if d == self.snake.head_direction().opposite() {
                return;
            }
            self.update_snake(Some(d));
        } else {
            // 非方向键：忽略
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw_dynamic_with_offset(con, g, self.block_size, self.offset_x, self.offset_y);
        if self.food_exists {
            draw_block_dynamic_with_offset(FOOD_COLOR, self.food_x, self.food_y, self.block_size, self.offset_x, self.offset_y, con, g);
        }
        // 绘制大食物（更大的 - 2x2方块）
        if self.big_food_exists {
            for dx in 0..2 {
                for dy in 0..2 {
                    draw_block_dynamic_with_offset(BIG_FOOD_COLOR, self.big_food_x + dx, self.big_food_y + dy, self.block_size, self.offset_x, self.offset_y, con, g);
                }
            }
        }
        draw_rectangle_dynamic_with_offset(BORDER_COLOR, 0, 0, self.width, 1, self.block_size, self.offset_x, self.offset_y, con, g);
        draw_rectangle_dynamic_with_offset(BORDER_COLOR, 0, self.height - 1, self.width, 1, self.block_size, self.offset_x, self.offset_y, con, g);
        draw_rectangle_dynamic_with_offset(BORDER_COLOR, 0, 0, 1, self.height, self.block_size, self.offset_x, self.offset_y, con, g);
        draw_rectangle_dynamic_with_offset(BORDER_COLOR, self.width - 1, 0, 1, self.height, self.block_size, self.offset_x, self.offset_y, con, g);
        if self.game_over {
            draw_rectangle_dynamic_with_offset(GAMEOVER_COLOR, 0, 0, self.width, self.height, self.block_size, self.offset_x, self.offset_y, con, g);
        }
        // 限时模式右上角时间
        if self.game_mode == GameMode::Survival {
            if let Some(sec) = self.remaining_time {
                let min = (sec as i32) / 60;
                let s = (sec as i32) % 60;
                let time_str = format!("TIME {:02}:{:02}", min, s);
                // 右上角
                let txt_x = self.window_width - 160.0;
                let txt_y = 40.0;
                crate::menu::draw_simple_text(&time_str, txt_x, txt_y, 28.0, [1.0, 1.0, 0.0, 1.0], con, g);
            }
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.game_mode == GameMode::Survival {
            if let Some(rt) = self.remaining_time.as_mut() {
                *rt -= delta_time;
                if *rt < 0.0 {
                    *rt = 0.0;
                }
                if *rt <= 0.0 && !self.game_over {
                    self.game_over = true;
                }
            }
        }
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        if !self.food_exists {
            self.add_food();
        }
        
        // 处理大食物计时
        self.big_food_timer += delta_time;
        
        // 每8秒生成大食物
        if !self.big_food_exists && self.big_food_timer >= BIG_FOOD_SPAWN_INTERVAL {
            self.add_big_food();
            self.big_food_timer = 0.0;
            self.big_food_lifetime = 0.0;
        }
        
        // 更新大食物存活时间
        if self.big_food_exists {
            self.big_food_lifetime += delta_time;
            // 5秒后移除大食物
            if self.big_food_lifetime >= BIG_FOOD_LIFETIME {
                self.big_food_exists = false;
            }
        }
        
        // 更新消息存活时间
        for msg in &mut self.messages {
            msg.lifetime -= delta_time;
        }
        self.messages.retain(|msg| msg.lifetime > 0.0);
        
        // 根据游戏模式与选择的速度调整移动速度
        let speed_setting_multiplier = match self.speed_setting {
            GameSpeed::Slow => 0.7,    // 慢速
            GameSpeed::Medium => 1.0,  // 中速
            GameSpeed::Fast => 1.5,    // 快速
        };
        let moving_period = MOVING_PERIOD / (self.speed_multiplier * speed_setting_multiplier);
        if self.waiting_time > moving_period {
            self.update_snake(None);
        }
    }
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
            self.score += 1;
            self.add_message(format!("+1 Point"));
            
            // 在速度模式下，随着分数增加，速度也会增加
            if self.game_mode == GameMode::Speed {
                self.speed_multiplier = 1.5 + (self.score as f64 * 0.1);
            }
            if self.game_mode == GameMode::Survival {
                if let Some(rt) = self.remaining_time.as_mut() {
                    *rt += TIME_ADD_PER_FOOD as f64;
                    self.add_message(format!("+{} Seconds", TIME_ADD_PER_FOOD));
                }
            }
        }
        
        // 检查蛇是否吃到大食物（头部接触到2x2大食物的任意部分）
        if self.big_food_exists {
            let in_big_food_x = head_x >= self.big_food_x && head_x < self.big_food_x + 2;
            let in_big_food_y = head_y >= self.big_food_y && head_y < self.big_food_y + 2;
            if in_big_food_x && in_big_food_y {
                self.big_food_exists = false;
                // 大食物加3分
                for _ in 0..3 {
                    self.snake.restore_tail();
                }
                self.score += 3;
                self.add_message(format!("Big Food +3 Points!"));
                
                // 在速度模式下，随着分数增加，速度也会增加
                if self.game_mode == GameMode::Speed {
                    self.speed_multiplier = 1.5 + (self.score as f64 * 0.1);
                }
                if self.game_mode == GameMode::Survival {
                    if let Some(rt) = self.remaining_time.as_mut() {
                        let time_bonus = TIME_ADD_PER_FOOD * 3;
                        *rt += time_bonus as f64;
                        self.add_message(format!("+{} Seconds", time_bonus));
                    }
                }
            }
        }
    }
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }
    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.width - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.width - 1);
        }
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }
    
    fn add_big_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1, self.width - 2); // -2 确保2x2大小能放得下
        let mut new_y = rng.gen_range(1, self.height - 2);
        
        // 检查2x2区域是否与蛇重叠
        let mut valid_position = false;
        let mut attempts = 0;
        while !valid_position && attempts < 100 {
            valid_position = true;
            // 检查2x2区域的所有4个方块
            for dx in 0..2 {
                for dy in 0..2 {
                    if self.snake.overlap_tail(new_x + dx, new_y + dy) {
                        valid_position = false;
                        break;
                    }
                }
                if !valid_position {
                    break;
                }
            }
            if !valid_position {
                new_x = rng.gen_range(1, self.width - 2);
                new_y = rng.gen_range(1, self.height - 2);
                attempts += 1;
            }
        }
        
        self.big_food_x = new_x;
        self.big_food_y = new_y;
        self.big_food_exists = true;
    }
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.big_food_exists = false;
        self.big_food_x = 0;
        self.big_food_y = 0;
        self.big_food_timer = 0.0;
        self.big_food_lifetime = 0.0;
        self.game_over = false;
        self.score = 0;
        // 重置速度倍数
        self.speed_multiplier = match self.game_mode {
            GameMode::Classic => 1.0,
            GameMode::Speed => 1.5,
            GameMode::Survival => 0.8,
        };
        self.remaining_time = if self.game_mode == GameMode::Survival {
            Some(TIME_LIMIT_SECONDS as f64)
        } else {
            None
        };
    }
    
    pub fn get_score(&self) -> i32 {
        self.score
    }
    
    pub fn get_game_mode(&self) -> GameMode {
        self.game_mode
    }
    
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
    
    pub fn set_game_over(&mut self) {
        self.game_over = true;
    }
    pub fn get_remaining_time(&self) -> Option<f64> {
        self.remaining_time
    }
    
    fn add_message(&mut self, text: String) {
        self.messages.push(GameMessage::new(text));
    }
    
    pub fn draw_messages(&self, con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        // 在游戏区域右侧绘制消息
        let start_x = self.offset_x + (self.width as f64 * self.block_size) + 20.0;
        let start_y = 100.0; // 从分数下方开始
        let line_height = 30.0;
        
        for (i, msg) in self.messages.iter().enumerate() {
            // 根据剩余存活时间计算透明度（最后0.5秒淡出）
            let fade_start = 0.5;
            let alpha = if msg.lifetime > fade_start {
                1.0
            } else {
                (msg.lifetime / fade_start).max(0.0)
            };
            
            let y = start_y + (i as f64 * line_height);
            let color = if msg.text.contains("Big Food") {
                // 大食物消息使用橙色
                [1.0f32, 0.6f32, 0.0f32, alpha as f32]
            } else {
                // 普通食物消息使用绿色
                [0.2f32, 1.0f32, 0.2f32, alpha as f32]
            };
            
            // 绘制背景矩形以提高可见性
            let text_size = 20.0;
            let bg_width = 200.0;
            let bg_height = 28.0;
            rectangle(
                [0.0f32, 0.0f32, 0.0f32, (alpha * 0.5) as f32],
                [start_x, y - bg_height / 2.0, bg_width, bg_height],
                con.transform,
                g,
            );
            
            // 绘制消息文本
            self.draw_text_with_alpha(&msg.text, start_x + bg_width / 2.0, y, text_size, color, con, g, glyphs);
        }
    }
    
    fn draw_text_with_alpha(&self, text: &str, x: f64, y: f64, size_px: f64, color: [f32; 4], con: &Context, g: &mut G2d, glyphs: &mut piston_window::Glyphs) {
        use piston_window::character::CharacterCache;
        
        let spx = size_px as u32;
        let total_w = glyphs.width(spx, text).unwrap_or(0.0);
        let baseline_adjust = size_px * 0.35;
        let transform = con.transform.trans(x - total_w / 2.0, y + baseline_adjust);
        let txt = piston_window::Text::new_color(color, spx);
        let _ = txt.draw(text, glyphs, &con.draw_state, transform, g);
    }
}
