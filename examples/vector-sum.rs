use rand::prelude::*;
use raylib::prelude::*;

use vector_implementation::{draw_arrow, draw_text, vector::Vec2};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Vector Sum Toy")
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();

    let mut origin: Option<Vec2> = None;
    let mut last_click: Option<Vec2> = None;
    let mut vectors: Vec<Vec2> = Vec::new();
    let mut rng = rand::rng();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        draw_text(
            &mut d,
            "Controls:\nLMB: Create Line\t\tRMB: Randomize Points Order\t\t ESC: Quit",
            Vec2::new(10., 10.),
            16,
            Color::BLACK,
        );

        let mouse_pos: Vec2 = d.get_mouse_position().into();

        d.draw_circle_lines_v(mouse_pos, 5., Color::RED);

        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(pos) = last_click {
                vectors.push(mouse_pos - pos);
            } else {
                origin = Some(mouse_pos);
            }
            last_click = Some(mouse_pos);
        }

        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
            vectors.shuffle(&mut rng);
        }

        if let Some(pos) = origin {
            draw_arrow(&mut d, last_click.unwrap(), mouse_pos, 2., Color::ORANGERED);
            let sum = vectors.iter().fold(pos, |acc, v| {
                draw_arrow(&mut d, acc, acc + *v, 2., Color::PURPLE);
                acc + *v
            });
            draw_arrow(&mut d, pos, sum, 2., Color::GREEN);
        }
    }
}
