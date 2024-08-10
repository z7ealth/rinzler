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
        self.check_snake_collision_with_food()
    }

    pub fn check_snake_collision_with_food(&mut self) {

        println!("Snake head position: {:?}", self.snake.body[0]);
        println!("Food position: {:?}", self.food.position);

        if self.snake.body[0].eq(&self.food.position) {
            self.food.position = Food::generate_random_pos(&self.snake.body);
        }
    }
}
