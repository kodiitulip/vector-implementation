#![allow(dead_code)]

use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle};

use crate::vector::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub p1: Vec2,
    pub p2: Vec2,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Intersection {
    pub point: Vec2,
    pub normal: Vec2,
}

impl Line {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Self { p1, p2 }
    }

    pub fn intersects(self, rhs: Line) -> bool {
        let ab = self.p2 - (self.p1);
        let ac = rhs.p1 - (self.p1);
        let ad = rhs.p2 - (self.p1);

        let s1 = ab.cross(ac);
        let s2 = ab.cross(ad);

        // se s1 e s2 tem o mesmo sinal, C e D estao no mesmo lado de AB.
        // Logo, nao tem interseçao
        if s1 * s2 >= 0.0 {
            return false;
        }

        let cd = rhs.p2 - rhs.p1;
        let ca = self.p1 - rhs.p1;
        let cb = self.p2 - rhs.p1;

        let s3 = cd.cross(ca);
        let s4 = cd.cross(cb);

        // Se s3 e s4 não tem o mesmo sinal, A e B estao no mesmo lado de CD.
        // Logo, ha interseção.
        s3 * s4 < 0.0
    }

    pub fn normal(self) -> Vec2 {
        let v = self.p2 - self.p1;
        Vec2::new(v.y, -v.x).normalized()
    }

    pub fn direction(self) -> Vec2 {
        (self.p2 - self.p1).normalized()
    }

    pub fn intersection(self, rhs: Line) -> Option<Intersection> {
        let r_line = self.p2 - self.p1;
        let l_line = rhs.p2 - rhs.p1;
        let normal = Vec2::new(-l_line.y, l_line.x);

        let dotn = r_line.dot(normal);
        if dotn.abs() < 1e-8 {
            return None;
        }

        let t = (rhs.p1 - self.p1).cross(l_line) / r_line.cross(l_line);
        // let u = (rhs.p1 - self.p1).cross(r_line) / r_line.cross(l_line);
        Some(Intersection {
            point: self.p1 + r_line * t,
            normal: rhs.normal(),
        })
    }

    pub fn len_squared(self) -> f32 {
        (self.p2 - self.p1).len_squared()
    }

    pub fn len(self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn is_degenerate(self) -> bool {
        self.len_squared() <= 1e-12
    }

    pub fn draw(self, d: &mut RaylibDrawHandle, thick: f32, color: Color) {
        d.draw_line_ex(self.p1, self.p2, thick, color);
    }
}

impl From<(Vec2, Vec2)> for Line {
    fn from(value: (Vec2, Vec2)) -> Self {
        Line {
            p1: value.0,
            p2: value.1,
        }
    }
}
