use std::{collections::VecDeque, path::Path};

use super::{config::CELL_COUNT, food::Food, snake::Snake};
use raylib::{
    audio::{RaylibAudio, Sound},
    consts,
    drawing::RaylibDrawHandle,
    math::Vector2,
    RaylibHandle, RaylibThread,
};

pub struct Game<'a> {
    pub snake: Snake,
    pub food: Food,
    pub running: bool,
    pub score: u32,
    pub eat_sound: Sound<'a>,
    pub wall_sound: Sound<'a>,
    pub motorcycle_intro_sound: Sound<'a>,
    pub motorcycle_running_sound: Sound<'a>,
    pub motorcycle_intro_sound_played: bool,
}

impl<'a> Game<'a> {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, sound: &'a RaylibAudio) -> Self {
        let food = Food::new(rl, thread);
        let snake = Snake::new(rl, thread);

        let eat_sound = sound
            .new_sound(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets/audio/motorcycle_gets_disc.wav")
                    .to_str()
                    .unwrap(),
            )
            .expect("Unable to load eat sound");

        let wall_sound = sound
            .new_sound(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets/audio/motorcycle_crash.wav")
                    .to_str()
                    .unwrap(),
            )
            .expect("Unable to load eat sound");

        let motorcycle_intro_sound = sound
            .new_sound(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets/audio/motorcycle_intro.wav")
                    .to_str()
                    .unwrap(),
            )
            .expect("Unable to load eat sound");

        let motorcycle_running_sound = sound
            .new_sound(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets/audio/motorcycle_running.wav")
                    .to_str()
                    .unwrap(),
            )
            .expect("Unable to load eat sound");

        Self {
            snake,
            food,
            running: false,
            score: 0,
            eat_sound,
            wall_sound,
            motorcycle_intro_sound,
            motorcycle_running_sound,
            motorcycle_intro_sound_played: false,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.snake.draw(d);
        self.food.draw(d);
    }

    pub fn update(&mut self) {
        if self.running {
            self.snake.update();
            self.check_snake_collision_with_food();
            self.check_snake_collision_with_edges();
            self.check_snake_collision_with_tail();
        }
    }

    pub fn update_snake_direction(&mut self, rl: &mut RaylibHandle) {
        let pressed_key: Option<consts::KeyboardKey> = rl.get_key_pressed();
        if let Some(key) = pressed_key {
            if key == consts::KeyboardKey::KEY_UP && self.snake.direction.y != 1.0
                || !(key != consts::KeyboardKey::KEY_UP || self.running)
            {
                self.snake.direction = Vector2::new(0.0, -1.0);
                self.snake.texture_rotation = Snake::get_texture_rotation(self.snake.direction);
                self.running = true;
                return;
            }
            if key == consts::KeyboardKey::KEY_DOWN
                && self.snake.direction.y != -1.0
                && self.running
            {
                self.snake.direction = Vector2::new(0.0, 1.0);
                self.snake.texture_rotation = Snake::get_texture_rotation(self.snake.direction);
                self.running = true;
                return;
            }
            if key == consts::KeyboardKey::KEY_LEFT && self.snake.direction.x != 1.0 {
                self.snake.direction = Vector2::new(-1.0, 0.0);
                self.snake.texture_rotation = Snake::get_texture_rotation(self.snake.direction);
                self.running = true;
                return;
            }
            if key == consts::KeyboardKey::KEY_RIGHT && self.snake.direction.x != -1.0 {
                self.snake.direction = Vector2::new(1.0, 0.0);
                self.snake.texture_rotation = Snake::get_texture_rotation(self.snake.direction);
                self.running = true;
            }
        }
    }

    pub fn check_snake_collision_with_food(&mut self) {
        println!("Snake head position: {:?}", self.snake.body[0]);
        println!("Food position: {:?}", self.food.position);

        if self.snake.body[0].eq(&self.food.position) {
            self.food.position = Food::generate_random_pos(&self.snake.body);
            self.snake.should_increase = true;
            self.score += 10;
            self.eat_sound.play()
        }
    }

    pub fn check_snake_collision_with_edges(&mut self) {
        if self.snake.body[0].x == CELL_COUNT as f32 || self.snake.body[0].x == -1.0 {
            Self::game_over(self);
        }
        if self.snake.body[0].y == CELL_COUNT as f32 || self.snake.body[0].y == -1.0 {
            Self::game_over(self);
        }
    }

    pub fn check_snake_collision_with_tail(&mut self) {
        let mut headless_body: VecDeque<Vector2> = self.snake.body.clone();
        headless_body.pop_front();
        if headless_body.contains(&self.snake.body[0]) {
            Self::game_over(self);
        }
    }

    fn game_over(&mut self) {
        self.snake.reset();
        self.food.position = Food::generate_random_pos(&self.snake.body);
        self.score = 0;
        self.running = false;
        if self.motorcycle_intro_sound.is_playing() {
            self.motorcycle_intro_sound.stop()
        };
        if self.motorcycle_running_sound.is_playing() {
            self.motorcycle_running_sound.stop()
        };
        self.motorcycle_intro_sound_played = false;
        self.wall_sound.play();
    }
}
