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
    math::{Rectangle, Vector2},
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};

use super::config::{CELL_SIZE, SNAKE_MOVEMENT_INTERVAL};

pub struct Snake {
    pub body: VecDeque<Vector2>,
    texture: Texture2D,
    direction: Vector2,
    last_update: Instant,
}

impl Snake {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let image = Image::load_image(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets/graphics/objects/snake.png")
                .to_str()
                .unwrap(),
        )
        .expect("Unable to load Food Image");

        let texture = RaylibHandle::load_texture_from_image(rl, thread, &image)
            .expect("Unable to load Food texture");

        Self {
            body: VecDeque::from([
                Vector2::new(6.0, 9.0),
                Vector2::new(5.0, 9.0),
                Vector2::new(4.0, 9.0),
            ]),
            texture,
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
            let rectangle = Rectangle::new(
                body_part.x * CELL_SIZE as f32,
                body_part.y * CELL_SIZE as f32,
                8.0,
                8.0,
            );
            if index == 0 {
                /*d.draw_texture(
                    &self.texture,
                    body_part.x as i32 * (CELL_SIZE),
                    body_part.y as i32 * (CELL_SIZE),
                    Color::WHITE,
                );*/
                d.draw_rectangle_rounded(rectangle, 0.5, 6, Color::ORANGE);

                continue;
            }
            d.draw_rectangle_rounded(rectangle, 0.5, 6, Color::WHITE);
        }
    }
}
