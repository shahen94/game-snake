use piston_window::{Context, G2d};
use std::collections::LinkedList;

use super::common::SNAKE_COLOR;
use super::draw::draw_block;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct SnakeBlock {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<SnakeBlock>,
    tail: Option<SnakeBlock>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body: LinkedList<SnakeBlock> = LinkedList::new();
        body.push_back(SnakeBlock { x: x + 2, y });
        body.push_back(SnakeBlock { x: x + 1, y });
        body.push_back(SnakeBlock { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
      for block in &self.body {
        draw_block(SNAKE_COLOR, block.x, block.y, ctx, g);
      }
    }

    pub fn head_position(&self) -> (i32, i32) {
      let head_block = self.body.front().unwrap();
      (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
      match dir {
          Some(direction) => self.direction = direction,
          None => ()
      };

      let new_block = self.create_block_for(self.direction);

      self.body.push_front(new_block);
      let removed_block = self.body.pop_back().unwrap();
      self.tail = Some(removed_block);
    }

    fn create_block_for(&self, dir: Direction) -> SnakeBlock {
      let (x, y) = self.next_head(Some(dir));

      SnakeBlock { x, y }
    }

    pub fn head_direction(&self) -> Direction {
      self.direction
    }

    pub fn next_head(&self, direction: Option<Direction>) -> (i32, i32) {
      let (head_x, head_y) = self.head_position();

      let mut moving_dir = self.direction;

      match direction {
        Some(dir) => moving_dir = dir,
        None => ()
      };

      match moving_dir {
        Direction::Up => (head_x, head_y - 1),
        Direction::Down => (head_x, head_y + 1),
        Direction::Left => (head_x - 1, head_y),
        Direction::Right => (head_x + 1, head_y),
      }
    }

    pub fn restore_tail(&mut self) {
      let blk = self.tail.clone().unwrap();
      self.body.push_back(blk);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
      let mut c = 0;
      for block in &self.body {
        if x == block.x && y == block.y {
          return true;
        }
        c += 1;
        if c == self.body.len() - 1 {
          break;
        }
      }
      return false;
    }
}
