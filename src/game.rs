use crate::{
    objects::{pipe::PipePair, player::Player},
    SCREEN_HEIGHT,
};

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
            player: Player::new().x(80).y(SCREEN_HEIGHT / 2),
            state: GameState::Idle,
        }
    }
    pub fn player(mut self, player: Player) -> Game {
        self.player = player;
        self
    }
    pub fn reset(&mut self) {
        self.pipes = Vec::new();
        self.player = Player::new().x(80).y(SCREEN_HEIGHT / 2);
        self.state = GameState::Idle;
    }
}
