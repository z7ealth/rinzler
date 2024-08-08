mod internal;

use internal::{
    colors::{get_color, BACKGROUND_COLOR},
    config::{CELL_COUNT, CELL_SIZE},
    food::Food,
    snake::Snake,
};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(CELL_SIZE * CELL_COUNT, CELL_SIZE * CELL_COUNT)
        .title("Rinzler")
        .build();

    rl.set_target_fps(60);

    let food = Food::new(&mut rl, &thread);
    let mut snake = Snake::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(get_color(BACKGROUND_COLOR));

        snake.update();

        food.draw(&mut d);
        snake.draw(&mut d);

        let fps = format!("FPS: {}", d.get_fps());
        d.draw_text(&fps, CELL_SIZE * CELL_COUNT - 60, 12, 12, Color::WHITE);
    }
}
