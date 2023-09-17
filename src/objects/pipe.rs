use raylib::prelude::{Color, RaylibDraw};

use crate::SCREEN_HEIGHT;

use super::VisibleObject;

pub const PIPE_WIDTH: i32 = 80;

#[derive(Clone, Copy)]
pub struct PipePair {
    pub opening_y: i32,
    pub opening_height: i32,

    pub x: i32,
}

impl PipePair {
    pub fn new() -> PipePair {
        PipePair {
            opening_y: 0,
            opening_height: 0,
            x: 0,
        }
    }
    pub fn opening_y(mut self, opening_y: i32) -> PipePair {
        self.opening_y = opening_y;
        self
    }
    pub fn opening_height(mut self, opening_height: i32) -> PipePair {
        self.opening_height = opening_height;
        self
    }
    pub fn x(mut self, x: i32) -> PipePair {
        self.x = x;
        self
    }
}

impl VisibleObject for PipePair {
    fn draw(&self, d: &mut raylib::prelude::RaylibDrawHandle) {
        d.draw_rectangle(self.x, 0, PIPE_WIDTH, self.opening_y, Color::GREEN);
        d.draw_rectangle(
            self.x,
            self.opening_y + self.opening_height,
            PIPE_WIDTH,
            SCREEN_HEIGHT,
            Color::GREEN,
        );
    }
}
