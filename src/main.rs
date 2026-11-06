use crate::vector::{RIGHT, Vec2};
use raylib::prelude::*;

mod vector;

fn main() {
    let (mut rl, thread) = raylib::init().size(400, 400).title("text").build();

    let vector = RIGHT;
    assert_eq!(vector * 3, Vec2 { x: 3, y: 0 });

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        let pos = Vec2 { x: 20, y: 20 };
        let opos = pos + Vec2 { x: 30, y: 3 };
        d.draw_line(0, 0, pos.x, pos.y, Color::BLACK);
        d.draw_line(0, 0, opos.x, opos.y, Color::RED);
    }
}
