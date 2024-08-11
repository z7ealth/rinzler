use super::{config::CELL_COUNT, food::Food, snake::Snake};
use raylib::{consts, drawing::RaylibDrawHandle, math::Vector2, RaylibHandle, RaylibThread};

pub struct Game {
    pub snake: Snake,
    pub food: Food,
    pub running: bool,
}

impl Game {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let food = Food::new(rl, thread);
        let snake = Snake::new(rl, thread);

        Self {
            snake,
            food,
            running: false,
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
        }
    }

    pub fn update_snake_direction(&mut self, rl: &mut RaylibHandle) {
        let pressed_key: Option<consts::KeyboardKey> = rl.get_key_pressed();
        if let Some(key) = pressed_key {
            if key == consts::KeyboardKey::KEY_UP && self.snake.direction.y != 1.0 {
                self.snake.direction = Vector2::new(0.0, -1.0);
                self.snake.texture_rotation = Snake::get_texture_rotation(self.snake.direction);
                self.running = true;
                return;
            }
            if key == consts::KeyboardKey::KEY_DOWN && self.snake.direction.y != -1.0 {
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

    fn game_over(&mut self) {
        self.snake.reset();
        self.food.position = Food::generate_random_pos(&self.snake.body);
        self.running = false
    }
}
