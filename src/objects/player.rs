use raylib::prelude::{Color, RaylibDraw};

use super::VisibleObject;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub acceleration_y: f32,
    pub width: i32,
    pub height: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 0,
            y: 0,
            acceleration_y: 0.0,
            width: 50,
            height: 50,
        }
    }
    pub fn x(mut self, x: i32) -> Player {
        self.x = x;
        self
    }
    pub fn y(mut self, y: i32) -> Player {
        self.y = y;
        self
    }
}

impl VisibleObject for Player {
    fn draw(&self, d: &mut raylib::prelude::RaylibDrawHandle) {
        d.draw_rectangle(self.x, self.y, self.width, self.height, Color::YELLOW);
    }
}
