mod game;
mod objects;

use objects::pipe::{PipePair, PIPE_WIDTH};
use objects::player::Player;
use rand::Rng;
use raylib::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use game::GameState;
use objects::VisibleObject;

pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 600;

pub const GRAVITY: f32 = 1.0;
pub const JUMP_FORCE: f32 = 12.0;

const PIPE_SPEED: f32 = 15.0;
const PIPE_STEP: f32 = 4.0;
const PIPE_SPAWN_COOLDOWN_SECS: u64 = 3;

type SharedGameState = Arc<Mutex<GameState>>;

#[tokio::main]
async fn main() {
    let game_state: SharedGameState =
        Arc::new(Mutex::new(GameState::new().player(Player::new().x(50))));
    let game_state_clone = game_state.clone();
    let render_handle = tokio::spawn(async move {
        let (mut rl, thread) = init().size(SCREEN_WIDTH, SCREEN_HEIGHT).build();
        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            let mut game_state = game_state_clone.lock().unwrap();
            d.clear_background(Color::BLUE);
            for pipe in &game_state.pipes {
                pipe.draw(&mut d);
            }
            game_state.player.draw(&mut d);
            if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
                game_state.player.acceleration_y = -JUMP_FORCE;
            }
        }
    });
    let game_state_clone = game_state.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(30));
        let mut game_state = game_state_clone.lock().unwrap();
        game_state.player.y += game_state.player.acceleration_y as i32;
        game_state.player.acceleration_y += GRAVITY;
    });
    let game_state_clone = game_state.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(PIPE_SPAWN_COOLDOWN_SECS));
        let mut game_state = game_state_clone.lock().unwrap();
        let mut rng = rand::thread_rng();
        let opening_height = 200;
        let pipe_y = rng.gen_range(50..(SCREEN_HEIGHT - opening_height - 50));
        game_state.pipes.push(
            PipePair::new()
                .x(SCREEN_WIDTH)
                .opening_height(opening_height)
                .opening_y(pipe_y),
        );
    });
    let game_state_clone = game_state.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(
            (PIPE_STEP * 100.0 / PIPE_SPEED) as u64,
        ));
        let mut game_state = game_state_clone.lock().unwrap();
        for pipe in &mut game_state.pipes {
            pipe.x -= PIPE_STEP as i32;
        }
    });
    let game_state_clone = game_state.clone();
    thread::spawn(move || loop {
        // Remove pipes that are outside of the screen every 2 seconds
        thread::sleep(Duration::from_secs(2));
        let mut game_state = game_state_clone.lock().unwrap();
        for (i, pipe) in game_state.pipes.clone().iter().enumerate() {
            if pipe.x + PIPE_WIDTH < 0 {
                game_state.pipes.remove(i);
            }
        }
    });
    render_handle.await.unwrap();
}
