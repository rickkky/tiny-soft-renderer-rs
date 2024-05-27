use interpolate::Interpolate;
use rand::Rng;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, Interpolate)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Self = Self::new_const(0.0, 0.0, 0.0, 1.0);
    pub const RED: Self = Self::new_const(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Self = Self::new_const(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Self = Self::new_const(0.0, 0.0, 1.0, 1.0);
    pub const WHITE: Self = Self::new_const(1.0, 1.0, 1.0, 1.0);

    pub const fn new_const(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        let r = r.min(1.0).max(0.0);
        let g = g.min(1.0).max(0.0);
        let b = b.min(1.0).max(0.0);
        let a = a.min(1.0).max(0.0);
        Self { r, g, b, a }
    }

    pub fn new_rand() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            r: rng.gen_range(0.0..1.0),
            g: rng.gen_range(0.0..1.0),
            b: rng.gen_range(0.0..1.0),
            a: 1.0,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a,
        }
    }
}
