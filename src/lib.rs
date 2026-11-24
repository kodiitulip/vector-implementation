use raylib::prelude::*;
pub mod line;
pub mod vector;

pub use line::Intersection;
pub use line::Line;
pub use vector::Vec2;

pub fn draw_arrow(
    d: &mut RaylibDrawHandle,
    start_pos: Vec2,
    end_pos: Vec2,
    thick: f32,
    color: Color,
) {
    d.draw_line_ex(start_pos, end_pos, thick, color);
    let mut guard = d.rl_push_matrix();
    let angle = (end_pos - start_pos).angle_to(Vec2::RIGHT).to_degrees();
    guard.rl_translatef(end_pos.x, end_pos.y, 0.0);
    guard.rl_rotatef(angle, 0., 0., 1.);
    guard.draw_triangle(Vec2::ZERO, Vec2::new(-10., -6.), Vec2::new(-10., 6.), color);
    drop(guard)
}

pub fn draw_text(
    d: &mut RaylibDrawHandle,
    text: &str,
    position: Vec2,
    font_size: i32,
    color: Color,
) {
    d.draw_text(text, position.x as i32, position.y as i32, font_size, color);
}

pub fn draw_wall(d: &mut RaylibDrawHandle, start_pos: Vec2, end_pos: Vec2, color: Color) {
    d.draw_line_ex(start_pos, end_pos, 5., color);
}

pub fn create_carteasian_grid(d: &mut RaylibDrawHandle, w: i32, h: i32) {
    // let mut guard = d.rl_push_matrix();
    let w: f32 = w.as_f32();
    let h: f32 = h.as_f32();

    draw_arrow(
        d,
        Vec2::new(-w / 2., 0.),
        Vec2::new(w / 2., 0.),
        2.,
        Color::RED,
    );

    draw_arrow(
        d,
        Vec2::new(0., -h / 2.),
        Vec2::new(0., h / 2.),
        2.,
        Color::BLUE,
    );

    d.rl_translatef(w / 2., h / 2., 0.);
    // guard.rl_translatef(w / 2., h / 2., 0.);
}
