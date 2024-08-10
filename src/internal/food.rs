use std::{collections::VecDeque, path::Path};

use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::GetRandomValue,
    math::Vector2,
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};

use super::config::{CELL_COUNT, CELL_SIZE};

pub struct Food {
    pub position: Vector2,
    texture: Texture2D,
}

impl Food {
    fn random_position() -> Vector2 {
        let x = unsafe { GetRandomValue(0, CELL_COUNT) as f32 };
        let y = unsafe { GetRandomValue(0, CELL_COUNT) as f32 };
        Vector2::new(x, y)
    }

    pub fn generate_random_pos(snake_body: &VecDeque<Vector2>) -> Vector2 {
        let mut position = Self::random_position();

        while snake_body.contains(&position) {
            position = Self::random_position();
        }

        position
    }

    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let image = Image::load_image(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets/graphics/objects/food.png")
                .to_str()
                .unwrap(),
        )
        .expect("Unable to load Food Image");

        let texture = RaylibHandle::load_texture_from_image(rl, thread, &image)
            .expect("Unable to load Food texture");

        Self {
            position: Self::random_position(),
            texture,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_texture(
            &self.texture,
            self.position.x as i32 * CELL_SIZE,
            self.position.y as i32 * CELL_SIZE,
            Color::WHITE,
        )
    }
}
