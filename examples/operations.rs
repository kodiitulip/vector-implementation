use raylib::prelude::*;
use vector_implementation::{draw_text, vector::Vec2};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 640)
        .title("Vector Operations")
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        draw_text(
            &mut d,
            "Vector Operations",
            Vec2::new(10., 10.),
            16,
            Color::BLACK,
        );
        draw_text(&mut d, "Addition", Vec2::new(10., 30.), 16, Color::BLACK);

        let origin = Vec2::new(10., 60.);
        let a = Vec2::new(150., 0.);
        let b = Vec2::new(50., 0.);

        d.draw_line_ex(origin, origin + a, 2., Color::BLUE);
        d.draw_line_ex(
            origin + Vec2::DOWN * 5.,
            origin + (Vec2::DOWN * 5.) + b,
            2.,
            Color::RED,
        );
        d.draw_line_ex(
            origin + Vec2::DOWN * 10.,
            origin + (Vec2::DOWN * 10.) + a + b,
            2.,
            Color::GREEN,
        );

        draw_text(&mut d, "Subtraction", Vec2::new(10., 80.), 16, Color::BLACK);

        let origin = Vec2::new(10., 100.);

        d.draw_line_ex(origin, origin + a, 2., Color::BLUE);
        d.draw_line_ex(
            origin + Vec2::DOWN * 5.,
            origin + (Vec2::DOWN * 5.) + b,
            2.,
            Color::RED,
        );
        d.draw_line_ex(
            origin + Vec2::DOWN * 10.,
            origin + (Vec2::DOWN * 10.) + a - b,
            2.,
            Color::GREEN,
        );

        draw_text(
            &mut d,
            "Scalar Multiplication",
            Vec2::new(10., 120.),
            16,
            Color::BLACK,
        );

        let origin = Vec2::new(10., 140.);

        d.draw_line_ex(origin, origin + a, 2., Color::BLUE);
        d.draw_line_ex(
            origin + Vec2::DOWN * 5.,
            origin + (Vec2::DOWN * 5.) + a * 2.,
            2.,
            Color::GREEN,
        );
        draw_text(&mut d, "x2", origin + Vec2::DOWN * 6., 16, Color::RED);

        draw_text(
            &mut d,
            "Scalar Product / Dot Product",
            Vec2::new(10., 180.),
            16,
            Color::BLACK,
        );

        let origin = Vec2::new(10., 200.);
        let a = Vec2::new(100., 0.);
        let b = Vec2::new(0., 20.);

        d.draw_line_ex(origin, origin + a, 2., Color::BLUE);
        d.draw_line_ex(
            origin + Vec2::DOWN * 5.,
            origin + (Vec2::DOWN * 5.) + b,
            2.,
            Color::RED,
        );
        let dot = a.dot(b);
        draw_text(
            &mut d,
            &format!("[100, 0] dot [0, 20] = {dot}"),
            origin + Vec2::new(20., 10.),
            16,
            Color::RED,
        );

        draw_text(
            &mut d,
            "Vectorial Product / Cross Product",
            Vec2::new(10., 240.),
            16,
            Color::BLACK,
        );

        let origin = Vec2::new(10., 260.);
        let a = Vec2::new(100., 0.);
        let b = Vec2::new(0., 20.);

        d.draw_line_ex(origin, origin + a, 2., Color::BLUE);
        d.draw_line_ex(
            origin + Vec2::DOWN * 5.,
            origin + (Vec2::DOWN * 5.) + b,
            2.,
            Color::RED,
        );
        let cross = a.cross(b);
        draw_text(
            &mut d,
            &format!("[100, 0] cross [0, 20] = {cross}"),
            origin + Vec2::new(20., 10.),
            16,
            Color::RED,
        );

        draw_text(&mut d, "Projection", Vec2::new(10., 300.), 16, Color::BLACK);

        let origin = Vec2::new(10., 320.);
        let a = Vec2::new(100., 0.);
        let b = Vec2::new(20., 20.);

        d.draw_line_ex(origin, origin + a, 2., Color::BLUE);
        d.draw_line_ex(
            origin + Vec2::DOWN * 5.,
            origin + (Vec2::DOWN * 5.) + b,
            2.,
            Color::RED,
        );
        d.draw_line_ex(
            origin + Vec2::DOWN * 2.,
            origin + Vec2::DOWN * 2. + b.project(a),
            2.,
            Color::GREEN,
        );
    }
}
