use std::path::Path;

use raylib::{
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::GetRandomValue,
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};

use crate::internal::config::CELL_COUNT;

use super::{
    colors::{get_color, FPS_COLOR},
    config::CELL_SIZE,
};

pub struct Food {
    pub position: (i32, i32),
    texture: Texture2D,
}

impl Food {
    fn generate_random_pos() -> (i32, i32) {
        let x = unsafe { GetRandomValue(0, CELL_COUNT - 1) };
        let y = unsafe { GetRandomValue(0, CELL_COUNT - 1) };
        (x, y)
    }

    pub fn new(rl: &mut RaylibHandle, rt: &RaylibThread) -> Self {
        let image = Image::load_image(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets/graphics/objects/food.png")
                .to_str()
                .unwrap(),
        )
        .expect("Unable to load Food Image");

        let texture = RaylibHandle::load_texture_from_image(rl, rt, &image)
            .expect("Unable to load Food texture");

        Self {
            position: Self::generate_random_pos(),
            texture,
        }
    }

    pub fn draw(&self, rdh: &mut RaylibDrawHandle) {
        rdh.draw_texture(
            &self.texture,
            self.position.0 * CELL_SIZE,
            self.position.1 * CELL_SIZE,
            get_color(FPS_COLOR),
        )
    }
}
