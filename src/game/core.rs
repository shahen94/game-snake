use piston_window::*;

use super::{
    common::{BORDER_COLOR, FOOD_COLOR, MOVING_PERIOD, GAMEOVER_COLOR, RESTART_TIME},
    draw::{draw_block, draw_rectangle},
    snake::Direction,
};
use crate::game::snake::Snake;
use rand::{thread_rng, Rng};

pub struct Game {
    snake: Snake,
    width: i32,
    height: i32,
    is_game_over: bool,

    food: FoodPosition,
    waiting_time: f64,
}

struct FoodPosition {
    exists: bool,
    x: i32,
    y: i32,
}

impl FoodPosition {
    fn new(x: i32, y: i32) -> FoodPosition {
        FoodPosition {
            x,
            y,
            exists: false,
        }
    }
}

trait Newable {
    fn new(width: i32, height: i32) -> Self;
}

impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Game {
            snake: Snake::new(2, 2),
            width,
            height,
            is_game_over: false,
            waiting_time: 0.0,
            food: FoodPosition::new(0, 0),
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.is_game_over {
            return;
        }

        let dir = match key {
            Key::Up => Direction::Up,
            Key::Down => Direction::Down,
            Key::Left => Direction::Left,
            Key::Right => Direction::Right,
            _ => self.snake.head_direction(),
        };

        if dir == self.snake.head_direction() || dir == self.snake.head_direction().opposite() {
            return;
        }
        self.update_snake(Some(dir));
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        // Draw food if it exists
        if self.food.exists {
            draw_block(FOOD_COLOR, self.food.x, self.food.y, ctx, g);
        }

        self.snake.draw(ctx, g);
        // Top line
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, ctx, g);
        // Bottom line
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, ctx, g);
        // Left line
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, ctx, g);
        // Right line
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, ctx, g);

        if self.is_game_over {
          draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, ctx, g);
        }
    }

    fn check_snake_alive(&self, dir: Option<Direction>) -> bool {
      let (x, y) = self.snake.next_head(dir);

      if self.snake.overlap_tail(x, y) {
        return false;
      }
      x > 0 && y > 0 && x < self.width - 1 && y < self.height - 1
    }

    pub fn update(&mut self, dt: f64) {
        self.waiting_time += dt;

        if self.is_game_over {
          if self.waiting_time > RESTART_TIME {
            self.restart();
          }
          return;
        }

        if !self.food.exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    fn restart(&mut self) {
      self.snake = Snake::new(2, 2);
      self.waiting_time = 0.0;
      self.food.exists = false;
      self.food.x = 0;
      self.food.y = 0;
      self.is_game_over = false;
    }

    fn add_food(&mut self) {
        let mut rnd = thread_rng();
        let mut food_x = rnd.gen_range(1..self.width - 1);
        let mut food_y = rnd.gen_range(1..self.height - 1);

        while self.snake.overlap_tail(food_x, food_y) {
          food_x = rnd.gen_range(1..self.width - 1);
          food_y = rnd.gen_range(1..self.height - 1);
        }

        self.food.x = food_x;
        self.food.y = food_y;
        self.food.exists = true;
    }

    fn check_eating(&mut self) {
        let (x, y) = self.snake.head_position();

        let is_eating = x == self.food.x && y == self.food.y && self.food.exists;

        if is_eating {
            self.food.exists = false;
            self.snake.restore_tail();
        }
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
      if self.check_snake_alive(dir) {
        self.snake.move_forward(dir);
        self.check_eating();
        self.waiting_time = 0.0;
      } else {
        self.is_game_over = true;
      }
    }
    
}
