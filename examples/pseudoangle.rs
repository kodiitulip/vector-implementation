use raylib::{
    ffi::{CSSPalette, RaylibPalette},
    prelude::*,
};
use vector_implementation::{Line, Vec2, create_carteasian_grid, draw_arrow};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("PseudoAngle")
        .log_level(TraceLogLevel::LOG_ERROR)
        .build();

    let mut line1 = Line::new(Vec2::ZERO, Vec2::new(100., 100.));
    let mut line2 = Line::new(Vec2::ZERO, Vec2::new(100., -100.));
    let mut line3 = line1;
    let mut line4 = line2;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);
        let mouse: Vec2 = Vec2::from(d.get_mouse_position()) - Vec2::new(400., 300.);

        create_carteasian_grid(&mut d, 800, 600);

        line1.p2 = mouse;
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            line2.p2 = mouse;
        }

        if line1.is_degenerate() {
            line3.p2 = line1.p2;
        } else {
            line3.p2 = line1.p2.normalized() * 200.
        }

        if line2.is_degenerate() {
            line4.p2 = line2.p2;
        } else {
            line4.p2 = line2.p2.normalized() * 200.
        }

        line3.draw(&mut d, 2., Color::BLACK.alpha(0.2));
        line4.draw(&mut d, 2., Color::BLACK.alpha(0.2));
        line1.draw(&mut d, 2., Color::SKYBLUE);
        line2.draw(&mut d, 2., Color::ORANGE);

        d.draw_circle_lines_v(Vec2::ZERO, 200., Color::BLACK.alpha(0.2));
        let vec1_angle = line1.p2.angle_to_dot(Vec2::RIGHT).to_degrees();
        if line1.p2.cross(Vec2::RIGHT) > 0.0 {
            d.draw_circle_sector_lines(Vec2::ZERO, 200., -vec1_angle, 0., 64, Color::BLUE);
        } else {
            d.draw_circle_sector_lines(Vec2::ZERO, 200., 0., vec1_angle, 64, Color::BLUE);
        }

        d.draw_rectangle_lines_ex(
            Rectangle::new(-200., -200., 400., 400.),
            2.,
            Color::BLACK.alpha(0.2),
        );

        let a_vec1_pseudo_sqr = if line1.is_degenerate() {
            0.0
        } else {
            line1.p2.pseudo_angle_square_arith()
        };

        draw_pseudoangle_arc(&mut d, a_vec1_pseudo_sqr);

        // Ângulo entre vetores via produto escalar
        let a_vecs_dot = if line1.is_degenerate() || line2.is_degenerate() {
            0.0
        } else {
            line1.p2.angle_to_dot(line2.p2).to_degrees()
        };

        // Ângulo entre vetores via produto vetorial
        let a_vecs_cross = if line1.is_degenerate() || line2.is_degenerate() {
            0.0
        } else {
            line1.p2.angle_to_cross(line2.p2).to_degrees()
        };

        // Pseudoângulo do cosseno entre vetores
        let a_vecs_pseudo_cos = if line1.is_degenerate() || line2.is_degenerate() {
            0.0
        } else {
            line1.p2.pseudo_angle_to(line2.p2)
        };

        // Pseudoângulo entre vetores no quadrado
        let a_vecs_pseudo_sqr = if line1.is_degenerate() || line2.is_degenerate() {
            0.0
        } else {
            line1.p2.pseudo_angle_square_to(line2.p2)
        };

        d.draw_text(
            format!("Ângulo (via produto escalar): {a_vecs_dot}").as_str(),
            10,
            10,
            16,
            Color::BLACK,
        );
        d.draw_text(
            format!("Ângulo (via produto vetorial): {a_vecs_cross}").as_str(),
            10,
            26,
            16,
            Color::BLACK,
        );
        d.draw_text(
            format!("Pseudoângulo do cosseno: {a_vecs_pseudo_cos}").as_str(),
            10,
            42,
            16,
            Color::BLACK,
        );
        d.draw_text(
            format!("Pseudoângulo no quadrado: {a_vec1_pseudo_sqr}").as_str(),
            10,
            58,
            16,
            Color::BLACK,
        );
        d.draw_text(
            format!("Pseudoângulo entre os vetores no quadrado: {a_vecs_pseudo_sqr}").as_str(),
            10,
            74,
            16,
            Color::BLACK,
        );
    }
}

fn draw_pseudoangle_arc(d: &mut RaylibDrawHandle, progress: f32) {
    let mut p = progress;

    // Primeira linha -> direita-topo
    let y1 = (p * 200.).min(200.0);
    p -= y1 / 200.;
    d.draw_line_v(Vec2::new(200., 0.), Vec2::new(200., y1), Color::GREEN);

    if p <= 0.0 {
        return;
    }

    // Segunda linha -> topo
    let x1 = (p * 200.).min(400.);
    p -= x1 / 200.;
    d.draw_line_v(
        Vec2::new(200., 200.),
        Vec2::new(200. - x1, 200.),
        Color::GREEN,
    );

    if p <= 0.0 {
        return;
    }

    // Terceira linha -> esquerda
    let y2 = (p * 200.).min(400.);
    p -= y2 / 200.;
    d.draw_line_v(
        Vec2::new(-200., 200.),
        Vec2::new(-200., 200. - y2),
        Color::GREEN,
    );

    if p <= 0.0 {
        return;
    }

    // Quarta linha -> baixo
    let x2 = (p * 200.).min(400.);
    p -= x2 / 200.;
    d.draw_line_v(
        Vec2::new(-200., -200.),
        Vec2::new(-200. + x2, -200.),
        Color::GREEN,
    );

    if p <= 0.0 {
        return;
    }

    // Quinta linha -> direita-baixo
    let y3 = (p * 200.0).min(200.0);
    d.draw_line_v(
        Vec2::new(200., -200.),
        Vec2::new(200., -200. + y3),
        Color::GREEN,
    );
}
