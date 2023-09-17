use crate::objects::{pipe::PipePair, player::Player};

pub enum GameState {
    Running,
    Idle,
}

pub struct Game {
    pub pipes: Vec<PipePair>,
    pub player: Player,
    pub state: GameState,
}
impl Game {
    pub fn new() -> Game {
        Game {
            pipes: Vec::new(),
            player: Player::new(),
            state: GameState::Idle,
        }
    }
    pub fn player(mut self, player: Player) -> Game {
        self.player = player;
        self
    }
}
