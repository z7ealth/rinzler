mod internal;

use internal::{
    colors::{get_color, BACKGROUND_COLOR},
    config::{CELL_COUNT, CELL_SIZE},
    game::Game,
};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(CELL_SIZE * CELL_COUNT, CELL_SIZE * CELL_COUNT)
        .title("Rinzler")
        .build();

    rl.set_target_fps(60);

    let mut game = Game::new(&mut rl, &thread);

    while !rl.window_should_close() {
        if game.snake.should_update() {
            game.update();
        }

        game.snake.update_direction(&mut rl);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(get_color(BACKGROUND_COLOR));

        let fps = format!("FPS: {}", d.get_fps());
        d.draw_text(&fps, CELL_SIZE * CELL_COUNT - 60, 12, 12, Color::WHITE);
        game.draw(&mut d);
    }
}
