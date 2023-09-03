use raylib::prelude::RaylibDrawHandle;

pub mod pipe;
pub mod player;

pub trait VisibleObject {
    fn draw(&self, d: &mut RaylibDrawHandle);
}
