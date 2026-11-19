#![allow(dead_code)]

use raylib::ffi::Vector2;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

const EPSILON: f32 = 1e-6;
const EPSILON_SQUARED: f32 = EPSILON * EPSILON;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2::new(0., 0.);
    pub const ONE: Vec2 = Vec2::new(1., 1.);
    pub const RIGHT: Vec2 = Vec2::new(1., 0.);
    pub const LEFT: Vec2 = Vec2::new(-1., 0.);
    pub const DOWN: Vec2 = Vec2::new(0., 1.);
    pub const UP: Vec2 = Vec2::new(0., -1.);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn from_angle(theta: f32) -> Self {
        Self {
            x: theta.cos(),
            y: theta.sin(),
        }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(self, rhs: Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn len(self) -> f32 {
        f32::sqrt(self.len_squared())
    }

    pub fn len_squared(self) -> f32 {
        self.x.powf(2.) + self.y.powf(2.)
    }

    pub fn project(self, rhs: Self) -> Self {
        let dot = self.dot(rhs);
        let len = rhs.len();
        rhs * (dot / (len.powf(2.)))
    }

    pub fn normalized(self) -> Self {
        self / self.len()
    }

    pub fn is_normalized(self) -> bool {
        (self.len_squared() - 1.).abs() <= EPSILON_SQUARED
    }

    pub fn angle_to(self, rhs: Self) -> f32 {
        f32::atan2(self.y - rhs.y, self.x - rhs.x)
    }

    pub fn rotated(self, theta: f32) -> Self {
        let st = theta.sin();
        let ct = theta.cos();
        Self {
            x: ct * self.x - st * self.y,
            y: st * self.x + ct * self.y,
        }
    }

    pub fn bounce(self, normal: Self) -> Self {
        self - (normal * self.dot(normal) * 2.)
    }

    pub fn reflect(self, normal: Self) -> Self {
        2. * self.project(normal) - self
    }

    pub fn is_parallel(self, rhs: Self) -> bool {
        self.cross(rhs).abs() <= EPSILON
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: rhs.x / self,
            y: rhs.y / self,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl From<Vec2> for Vector2 {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Vector2> for Vec2 {
    fn from(value: Vector2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<&Vec2> for Vector2 {
    fn from(value: &Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<&Vector2> for Vec2 {
    fn from(value: &Vector2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}
