use crate::vector::{DOWN, RIGHT, Vec2};
use raylib::prelude::*;

mod vector;

fn main() {
    let (mut rl, thread) = raylib::init().size(400, 400).title("text").build();

    let vector = RIGHT;
    assert_eq!(vector * 3, Vec2 { x: 3, y: 0 });
    let pos = (RIGHT + DOWN) * 20;
    let opos = Vec2 { x: 30, y: -3 };
    let dp = pos.dot(opos);
    let cross = pos.cross(opos);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        d.draw_line(0, 0, pos.x, pos.y, Color::BLACK);
        d.draw_line(pos.x, pos.y, (pos + opos).x, (pos + opos).y, Color::RED);
        d.draw_text(&*format!("pos . opos = {dp}"), 20, 40, 16, Color::BLACK);
        d.draw_text(&*format!("pos x opos = {cross}"), 20, 60, 16, Color::BLACK);
    }
}
