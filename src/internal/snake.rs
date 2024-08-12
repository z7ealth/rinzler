use std::{
    collections::VecDeque,
    ops::{Add, Sub},
    path::Path,
    time::{Duration, Instant},
};

use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
    math::{Rectangle, Vector2},
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};

use super::config::{CELL_COUNT, CELL_SIZE, SNAKE_MOVEMENT_INTERVAL};

pub struct Snake {
    pub body: VecDeque<Vector2>,
    head_texture: Texture2D,
    tail_texture: Texture2D,
    pub direction: Vector2,
    pub texture_rotation: f32,
    last_update: Instant,
    pub should_increase: bool,
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

        let direction = Vector2::new(1.0, 0.0);

        Self {
            body: Self::generate_initial_pos(),
            head_texture,
            tail_texture,
            texture_rotation: Self::get_texture_rotation(direction),
            direction,
            last_update: Instant::now(),
            should_increase: false,
        }
    }

    pub fn get_texture_rotation(direction: Vector2) -> f32 {
        match (direction.x, direction.y) {
            (0.0, 1.0) => 0.0,    // up,
            (0.0, -1.0) => 180.0, // down
            (1.0, 0.0) => 90.0,   // left
            (-1.0, 0.0) => 270.0, // right
            _ => 0.0,
        }
    }

    fn generate_initial_pos() -> VecDeque<Vector2> {
        VecDeque::from([
            Vector2::new((CELL_COUNT / 2) as f32, (CELL_COUNT - 4) as f32),
            Vector2::new((CELL_COUNT / 2) as f32, (CELL_COUNT - 3) as f32),
            Vector2::new((CELL_COUNT / 2) as f32, (CELL_COUNT - 2) as f32),
        ])
    }

    pub fn update(&mut self) {
        self.body
            .push_front(Vector2::add(self.body[0], self.direction));

        if self.should_increase {
            self.should_increase = false;
        } else {
            self.body.pop_back();
        }
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

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for (index, body_part) in self.body.clone().into_iter().enumerate() {
            if index == 0 {
                /*
                d.draw_texture_v(
                    &self.head_texture,
                    Vector2::new(
                        body_part.x * CELL_SIZE as f32,
                        body_part.y * CELL_SIZE as f32,
                    ),
                    Color::WHITE,
                );


                d.draw_texture_ex(
                    &self.head_texture,
                    Vector2::new(
                        body_part.x * CELL_SIZE as f32,
                        body_part.y * CELL_SIZE as f32,
                    ),
                    self.texture_rotation,
                    1.0,
                    Color::WHITE,
                );
                */

                let rectangle = Rectangle::new(
                    body_part.x * CELL_SIZE as f32,
                    body_part.y * CELL_SIZE as f32,
                    16.0,
                    16.0,
                );
                d.draw_rectangle_rounded(rectangle, 0.5, 6, Color::ORANGE);

                continue;
            }

            // UP X needs +1
            // DOWN OK
            // LEFT Y needs +1
            // RIGHT X needs +1
            /*
               d.draw_texture_ex(
                   &self.tail_texture,
                   Vector2::new(
                       body_part.x * CELL_SIZE as f32,
                       body_part.y * CELL_SIZE as f32,
                   ),
                   self.texture_rotation,
                   1.0,
                   Color::WHITE,
               );

               d.draw_texture_v(
                   &self.tail_texture,
                   Vector2::new(
                       body_part.x * CELL_SIZE as f32,
                       body_part.y * CELL_SIZE as f32,
                   ),
                   Color::WHITE,
               );
            */
            let rectangle = Rectangle::new(
                body_part.x * CELL_SIZE as f32,
                body_part.y * CELL_SIZE as f32,
                16.0,
                16.0,
            );
            d.draw_rectangle_rounded(rectangle, 0.5, 6, Color::WHITE);
        }
    }

    pub fn reset(&mut self) {
        let direction = Vector2::new(1.0, 0.0);

        let body = Self::generate_initial_pos();

        self.body = body;
        self.direction = direction;
    }
}
