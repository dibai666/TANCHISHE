use crate::draw::{draw_block_dynamic_with_offset, draw_rectangle_dynamic_with_offset};
use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};
use crate::snake::{Direction, Snake};
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];
const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
    window_width: f64,
    window_height: f64,
    block_size: f64,
    offset_x: f64,
    offset_y: f64,
}
impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        let initial_window_width = (width * 25) as f64; // 25 is BLOCK_SIZE
        let initial_window_height = (height * 25) as f64;
        Game {
            snake: Snake::new(2, 2),
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
            window_width: initial_window_width,
            window_height: initial_window_height,
            block_size: 25.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
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
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }
        self.update_snake(dir);
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw_dynamic_with_offset(con, g, self.block_size, self.offset_x, self.offset_y);
        if self.food_exists {
            draw_block_dynamic_with_offset(FOOD_COLOR, self.food_x, self.food_y, self.block_size, self.offset_x, self.offset_y, con, g);
        }
        draw_rectangle_dynamic_with_offset(BORDER_COLOR, 0, 0, self.width, 1, self.block_size, self.offset_x, self.offset_y, con, g);
        draw_rectangle_dynamic_with_offset(BORDER_COLOR, 0, self.height - 1, self.width, 1, self.block_size, self.offset_x, self.offset_y, con, g);
        draw_rectangle_dynamic_with_offset(BORDER_COLOR, 0, 0, 1, self.height, self.block_size, self.offset_x, self.offset_y, con, g);
        draw_rectangle_dynamic_with_offset(BORDER_COLOR, self.width - 1, 0, 1, self.height, self.block_size, self.offset_x, self.offset_y, con, g);
        if self.game_over {
            draw_rectangle_dynamic_with_offset(GAMEOVER_COLOR, 0, 0, self.width, self.height, self.block_size, self.offset_x, self.offset_y, con, g);
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        if !self.food_exists {
            self.add_food();
        }
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
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
        self.game_over = false;
    }
}
