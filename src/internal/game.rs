use raylib::{drawing::RaylibDrawHandle, RaylibHandle, RaylibThread};

use super::{food::Food, snake::Snake};

pub struct Game {
    pub snake: Snake,
    pub food: Food,
}

impl Game {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let food = Food::new(rl, thread);
        let snake = Snake::new(rl, thread);

        Self { snake, food }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.snake.draw(d);
        self.food.draw(d);
    }

    pub fn update(&mut self) {
        self.snake.update();
    }
}
