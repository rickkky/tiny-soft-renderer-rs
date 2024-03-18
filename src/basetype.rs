use nalgebra::Vector2;
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar: 'static + PartialEq + Copy + Debug {}

impl<T> Scalar for T where T: 'static + PartialEq + Copy + Debug {}

pub trait Number:
    Scalar
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
}

impl<T> Number for T where
    T: Scalar
        + PartialOrd
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
{
}

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub offset_x: i32,
    pub offset_y: i32,
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    pub fn new(offset_x: i32, offset_y: i32, width: u32, height: u32) -> Self {
        Viewport {
            offset_x,
            offset_y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bbox2<T: Number> {
    pub l: T,
    pub r: T,
    pub b: T,
    pub t: T,
}

impl<T: Number> Bbox2<T> {
    pub fn new(l: T, r: T, b: T, t: T) -> Self {
        Bbox2 { l, r, b, t }
    }

    pub fn from_points(points: &Vec<&Vector2<T>>) -> Self {
        if points.is_empty() {
            panic!("The input points are empty.");
        }
        let mut l = points[0].x;
        let mut r = points[0].x;
        let mut b = points[0].y;
        let mut t = points[0].y;
        for p in points {
            if p.x < l {
                l = p.x;
            }
            if p.x > r {
                r = p.x;
            }
            if p.y < b {
                b = p.y;
            }
            if p.y > t {
                t = p.y;
            }
        }
        Bbox2 { l, r, b, t }
    }
}
