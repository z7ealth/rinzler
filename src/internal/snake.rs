use std::{collections::VecDeque, ops::Add};

use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
};

use super::config::CELL_SIZE;

pub struct Snake {
    pub body: VecDeque<Vector2>,
    direction: Vector2,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: VecDeque::from([
                Vector2::new(6.0, 9.0),
                Vector2::new(5.0, 9.0),
                Vector2::new(4.0, 9.0),
            ]),
            direction: Vector2::new(1.0, 0.0),
        }
    }

    pub fn update(&mut self) {
        self.body.pop_back();
        self.body.push_front(Vector2::add(self.body[0], self.direction))
    }

    pub fn draw(&self, rdh: &mut RaylibDrawHandle) {
        for body_part in &self.body {
            let rectangle = Rectangle::new(
                body_part.x * CELL_SIZE as f32,
                body_part.y * CELL_SIZE as f32,
                CELL_SIZE as f32,
                CELL_SIZE as f32,
            );

            rdh.draw_rectangle_rounded(rectangle, 0.5, 6, Color::WHITE)
        }
    }
}
