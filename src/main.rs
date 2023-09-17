mod game;
mod objects;

use objects::pipe::{PipePair, PIPE_WIDTH};
use objects::player::Player;
use rand::Rng;
use raylib::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use game::{Game, GameState};
use objects::VisibleObject;

pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 600;

pub const GRAVITY: f32 = 1.0;
pub const JUMP_FORCE: f32 = 12.0;

const PIPE_SPEED: f32 = 15.0;
const PIPE_STEP: f32 = 4.0;
const PIPE_SPAWN_COOLDOWN_SECS: u64 = 3;

type SharedGameInstance = Arc<Mutex<Game>>;

#[tokio::main]
async fn main() {
    let game: SharedGameInstance = Arc::new(Mutex::new(
        Game::new().player(Player::new().x(50).y(SCREEN_HEIGHT / 2)),
    ));
    let game_clone = game.clone();
    let render_handle = tokio::spawn(async move {
        let (mut rl, thread) = init().size(SCREEN_WIDTH, SCREEN_HEIGHT).build();
        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            let mut game = game_clone.lock().unwrap();
            d.clear_background(Color::BLUE);
            for pipe in &game.pipes {
                pipe.draw(&mut d);
            }
            game.player.draw(&mut d);
            if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
                game.state = GameState::Running;
                game.player.acceleration_y = -JUMP_FORCE;
            }
        }
    });
    let game_clone = game.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(30));
        let mut game = game_clone.lock().unwrap();
        if let GameState::Running = game.state {
            game.player.y += game.player.acceleration_y as i32;
            game.player.acceleration_y += GRAVITY;
        }
    });
    let game_clone = game.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(PIPE_SPAWN_COOLDOWN_SECS));
        let mut game = game_clone.lock().unwrap();
        if let GameState::Running = game.state {
            let mut rng = rand::thread_rng();
            let opening_height = 200;
            let pipe_y = rng.gen_range(50..(SCREEN_HEIGHT - opening_height - 50));
            game.pipes.push(
                PipePair::new()
                    .x(SCREEN_WIDTH)
                    .opening_height(opening_height)
                    .opening_y(pipe_y),
            );
        }
    });
    let game_clone = game.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(
            (PIPE_STEP * 100.0 / PIPE_SPEED) as u64,
        ));
        let mut game = game_clone.lock().unwrap();
        for pipe in &mut game.pipes {
            pipe.x -= PIPE_STEP as i32;
        }
    });
    let game_clone = game.clone();
    thread::spawn(move || loop {
        // Remove pipes that are outside of the screen every 2 seconds
        thread::sleep(Duration::from_secs(2));
        let mut game = game_clone.lock().unwrap();
        for (i, pipe) in game.pipes.clone().iter().enumerate() {
            if pipe.x + PIPE_WIDTH < 0 {
                game.pipes.remove(i);
            }
        }
    });
    let game_clone = game.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(150));
        let mut game = game_clone.lock().unwrap();
        if game.player.y > SCREEN_HEIGHT {
            game.reset();
        }
        for pipe in game.pipes.clone().iter() {
            let is_player_colliding_pipe = (game.player.x + game.player.width > pipe.x
                && game.player.x < pipe.x + PIPE_WIDTH)
                && ((game.player.y < pipe.opening_y)
                    || (game.player.y + game.player.height > pipe.opening_y + pipe.opening_height));
            if is_player_colliding_pipe {
                game.reset();
            }
        }
    });
    render_handle.await.unwrap();
}
