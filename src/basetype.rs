use nalgebra::Vector2;
use num_traits::Float;

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    pub fn new(width: u32, height: u32) -> Self {
        Viewport { width, height }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bbox2<T: Float> {
    pub l: T,
    pub r: T,
    pub b: T,
    pub t: T,
}

impl<T: Float> Bbox2<T> {
    pub fn new(l: T, r: T, b: T, t: T) -> Self {
        Bbox2 { l, r, b, t }
    }

    pub fn from_points(points: &Vec<&Vector2<T>>) -> Self {
        if points.is_empty() {
            panic!("The input points are empty.");
        }
        let mut l = points[0][0];
        let mut r = points[0][0];
        let mut b = points[0][1];
        let mut t = points[0][1];
        for p in points {
            if p[0] < l {
                l = p[0];
            }
            if p[0] > r {
                r = p[0];
            }
            if p[1] < b {
                b = p[1];
            }
            if p[1] > t {
                t = p[1];
            }
        }
        Bbox2 { l, r, b, t }
    }
}
