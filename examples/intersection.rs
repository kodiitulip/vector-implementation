use raylib::prelude::*;
use vector_implementation::{Line, Vec2};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("text")
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();

    let mut mouse = Line::new(Vec2::ZERO, Vec2::ZERO);
    let mut wall: Line = (Vec2::new(0., 300.), Vec2::new(800., 300.)).into();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        draw_text(&mut d, "Intersections", Vec2::ONE * 10.0, 16, Color::BLACK);
        draw_text(
            &mut d,
            "Controls:\nLMB: Change Origin\t\t1: Move Wall P1 to Mouse\t\t2: Move Wall P2 to Mouse\t\tESC: Quit",
            Vec2::new(10., 30.),
            12,
            Color::BLACK,
        );

        mouse.p2 = d.get_mouse_position().into();

        draw_wall(&mut d, wall.p1, wall.p2, Color::BLACK);
        draw_arrow(&mut d, mouse.p1, mouse.p2, 2., Color::BLUE);

        if mouse.intersects(wall)
            && let Some(inte) = mouse.intersection(wall)
        {
            let wall_v = wall.p2 - wall.p1;
            let point = inte.point;
            let reflection = (point - mouse.p1).reflect(wall_v);

            let slide = (point - mouse.p1).project(wall_v);

            draw_arrow(&mut d, mouse.p1, point, 2.0, Color::RED);
            draw_arrow(&mut d, point, point + slide, 2.0, Color::MAGENTA);
            draw_arrow(&mut d, point, point + reflection, 2.0, Color::GREEN);
        }

        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            mouse.p1 = d.get_mouse_position().into();
        }

        match d.get_key_pressed() {
            Some(KeyboardKey::KEY_ONE) => wall.p1 = d.get_mouse_position().into(),
            Some(KeyboardKey::KEY_TWO) => wall.p2 = d.get_mouse_position().into(),
            _ => (),
        }
    }
}

fn draw_wall(d: &mut RaylibDrawHandle, start_pos: Vec2, end_pos: Vec2, color: Color) {
    d.draw_line_ex(start_pos, end_pos, 5., color);
}

fn draw_arrow(d: &mut RaylibDrawHandle, start_pos: Vec2, end_pos: Vec2, thick: f32, color: Color) {
    d.draw_line_ex(start_pos, end_pos, thick, color);
    let mut guard = d.rl_push_matrix();
    let angle = (end_pos - start_pos).angle_to(Vec2::RIGHT).to_degrees();
    guard.rl_translatef(end_pos.x, end_pos.y, 0.0);
    guard.rl_rotatef(angle, 0., 0., 1.);
    guard.draw_triangle(Vec2::ZERO, Vec2::new(-10., -6.), Vec2::new(-10., 6.), color);
    drop(guard);
}

fn draw_text(d: &mut RaylibDrawHandle, text: &str, position: Vec2, font_size: i32, color: Color) {
    d.draw_text(text, position.x as i32, position.y as i32, font_size, color);
}
