use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(400, 400).title("text").build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_text("text", 20, 20, 20, Color::BLACK);
    }
}
