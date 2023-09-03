use crate::objects::{pipe::PipePair, player::Player, VisibleObject};

pub struct GameState {
    pub pipes: Vec<Box<PipePair>>,
    pub player: Player,
}
impl GameState {
    pub fn new() -> GameState {
        GameState {
            pipes: Vec::new(),
            player: Player::new(),
        }
    }
    pub fn player(mut self, player: Player) -> GameState {
        self.player = player;
        self
    }
}
