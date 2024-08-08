use std::collections::VecDeque;

use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::Rectangle,
};

use super::config::CELL_SIZE;

pub struct Snake {
    pub body: VecDeque<(i32, i32)>,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: VecDeque::from([(6, 9), (5, 9), (4, 9)]),
        }
    }

    pub fn draw(&self, rdh: &mut RaylibDrawHandle) {
        for body_part in &self.body {
            let rectangle = Rectangle::new(
                (body_part.0 * CELL_SIZE) as f32,
                (body_part.1 * CELL_SIZE) as f32,
                CELL_SIZE as f32,
                CELL_SIZE as f32,
            );

            rdh.draw_rectangle_rounded(rectangle, 0.5, 6, Color::WHITE)
        }
    }
}
