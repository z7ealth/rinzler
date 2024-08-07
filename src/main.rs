mod internal;

use internal::{
    colors::{get_color, BACKGROUND_COLOR, FPS_COLOR},
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    food::Food,
};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Rinzler")
        .build();

    rl.set_target_fps(60);

    let food = Food::new(&mut rl, &thread);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        food.draw(&mut d);

        d.clear_background(get_color(BACKGROUND_COLOR));

        let fps = format!("FPS: {}", d.get_fps());
        d.draw_text(&fps, WINDOW_WIDTH - 60, 12, 12, get_color(FPS_COLOR));
    }
}
