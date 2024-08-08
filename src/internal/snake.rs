use std::{
    collections::VecDeque,
    ops::{Add, Sub},
    path::Path,
    time::{Duration, Instant},
};

use raylib::{
    color::Color,
    consts,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::Vector2,
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};

use super::config::{CELL_SIZE, SNAKE_MOVEMENT_INTERVAL};

pub struct Snake {
    pub body: VecDeque<Vector2>,
    head_texture: Texture2D,
    tail_texture: Texture2D,
    direction: Vector2,
    last_update: Instant,
}

impl Snake {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let head_image = Image::load_image(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets/graphics/objects/snake.png")
                .to_str()
                .unwrap(),
        )
        .expect("Unable to load Food Image");

        let head_texture = RaylibHandle::load_texture_from_image(rl, thread, &head_image)
            .expect("Unable to load Food texture");

        let tail_image = Image::load_image(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets/graphics/objects/snake_tail.png")
                .to_str()
                .unwrap(),
        )
        .expect("Unable to load Food Image");

        let tail_texture = RaylibHandle::load_texture_from_image(rl, thread, &tail_image)
            .expect("Unable to load Food texture");

        Self {
            body: VecDeque::from([
                Vector2::new(6.0, 9.0),
                Vector2::new(5.0, 9.0),
                Vector2::new(4.0, 9.0),
            ]),
            head_texture,
            tail_texture,
            direction: Vector2::new(1.0, 0.0),
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.body.pop_back();
        self.body
            .push_front(Vector2::add(self.body[0], self.direction))
    }

    pub fn should_update(&mut self) -> bool {
        let current_instant = Instant::now();
        let elapsed = current_instant.sub(self.last_update);
        let interval = Duration::from_millis(SNAKE_MOVEMENT_INTERVAL);

        if elapsed.ge(&interval) {
            self.last_update = current_instant;

            return true;
        }

        false
    }

    pub fn update_direction(&mut self, rl: &mut RaylibHandle) {
        let pressed_key: Option<consts::KeyboardKey> = rl.get_key_pressed();
        if let Some(key) = pressed_key {
            if key == consts::KeyboardKey::KEY_UP && self.direction.y != 1.0 {
                self.direction = Vector2::new(0.0, -1.0);
                return;
            }
            if key == consts::KeyboardKey::KEY_DOWN && self.direction.y != -1.0 {
                self.direction = Vector2::new(0.0, 1.0);
                return;
            }
            if key == consts::KeyboardKey::KEY_LEFT && self.direction.x != 1.0 {
                self.direction = Vector2::new(-1.0, 0.0);
                return;
            }
            if key == consts::KeyboardKey::KEY_RIGHT && self.direction.x != -1.0 {
                self.direction = Vector2::new(1.0, 0.0);
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for (index, body_part) in self.body.clone().into_iter().enumerate() {
            if index == 0 {
                d.draw_texture(
                    &self.head_texture,
                    body_part.x as i32 * (CELL_SIZE),
                    body_part.y as i32 * (CELL_SIZE),
                    Color::WHITE,
                );
                continue;
            }

            d.draw_texture(
                &self.tail_texture,
                body_part.x as i32 * (CELL_SIZE),
                body_part.y as i32 * (CELL_SIZE),
                Color::WHITE,
            );
        }
    }
}
