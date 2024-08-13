mod internal;

use internal::{
    colors::{get_color, BACKGROUND_COLOR, BORDER_COLOR},
    config::{CELL_COUNT, CELL_SIZE, OFFSET},
    game::Game,
};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(CELL_SIZE * CELL_COUNT, CELL_SIZE * CELL_COUNT)
        .title("Rinzler")
        .build();

    rl.set_target_fps(60);

    let sound = RaylibAudio::init_audio_device().unwrap();

    let mut game = Game::new(&mut rl, &thread, &sound);
    game.wall_sound.set_volume(0.1);
    game.eat_sound.set_volume(0.1);
    game.motorcycle_intro_sound.set_volume(0.1);
    game.motorcycle_running_sound.set_volume(0.1);

    while !rl.window_should_close() {
        // Play motorcycle intro once
        if game.running && !game.motorcycle_intro_sound_played {
            game.motorcycle_intro_sound.play();
            game.motorcycle_intro_sound_played = true;
        }

        if game.running
            && !game.motorcycle_intro_sound.is_playing()
            && !game.motorcycle_running_sound.is_playing()
        {
            game.motorcycle_running_sound.play();
        }

        if game.snake.should_update() {
            game.update();
        }

        game.update_snake_direction(&mut rl);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(get_color(BACKGROUND_COLOR));

        let fps = format!("FPS: {}", d.get_fps());
        d.draw_text(&fps, CELL_SIZE * CELL_COUNT - 60, 12, 12, Color::WHITE);

        let score = format!("Score: {}", game.score);
        d.draw_text(&score, 12, 12, 12, Color::WHITE);

        d.draw_rectangle_lines_ex(
            Rectangle::new(
                OFFSET as f32,
                OFFSET as f32,
                (CELL_SIZE * CELL_COUNT) as f32,
                (CELL_SIZE * CELL_COUNT) as f32,
            ),
            1.0,
            get_color(BORDER_COLOR),
        );

        game.draw(&mut d);
    }
}
