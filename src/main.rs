use game::{draw::to_coords_u32, common::BACK_COLOR};
use piston_window::{PistonWindow, WindowSettings, clear, UpdateEvent, Button, PressEvent};

extern crate piston_window;
extern crate rand;
mod game;

use game::core::Game;

fn main() {
    let (width, height) = (20, 20);
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coords_u32(width), to_coords_u32(height)])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {

      if let Some(Button::Keyboard(key)) = event.press_args() {
        game.key_pressed(key);
      }

      window.draw_2d(&event, |c, g, _| {
        clear(BACK_COLOR, g);
        game.draw(&c, g);
      });
      event.update(|arg| {
        game.update(arg.dt);
      });
    }
}
