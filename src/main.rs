use crate::vector::Vec2;
use rand::prelude::*;
use raylib::prelude::*;

mod vector;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("text")
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();

    let mut origin: Option<Vec2> = None;
    let mut last_click: Option<Vec2> = None;
    let mut vectors: Vec<Vec2> = Vec::new();
    let mut rng = rand::rng();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        draw_text(&mut d,"Controls:\nLMB: Create Line\t\tRMB: Randomize Points Order\t\t ESC: Quit", Vec2::new(10., 10.), 16, Color::BLACK);

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

fn draw_arrow(d: &mut RaylibDrawHandle, start_pos: Vec2, end_pos: Vec2, thick: f32, color: Color) {
    d.draw_line_ex(start_pos, end_pos, thick, color);
    let mut guard = d.rl_push_matrix();
    let angle = (end_pos - start_pos).angle_to(Vec2::RIGHT).to_degrees();
    guard.rl_translatef(end_pos.x, end_pos.y, 0.0);
    guard.rl_rotatef(angle, 0., 0., 1.);
    guard.draw_triangle(
        Vec2::ZERO,
        Vec2::new(-10., -6.),
        Vec2::new(-10., 6.),
        color,
    );
    drop(guard)
}

fn draw_text(d: &mut RaylibDrawHandle, text: &str, position: Vec2, font_size: i32, color: Color) {
    d.draw_text(text, position.x as i32, position.y as i32, font_size, color);
}
