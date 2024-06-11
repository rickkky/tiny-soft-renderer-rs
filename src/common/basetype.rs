use nalgebra::{Vector2, Vector3};
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

    pub fn from_vector2(vectors: &Vec<&Vector2<T>>) -> Self {
        let mut l = vectors[0][0];
        let mut r = vectors[0][0];
        let mut b = vectors[0][1];
        let mut t = vectors[0][1];
        for p in vectors {
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

    pub fn intersect(&self, other: &Bbox2<T>) -> Option<Bbox2<T>> {
        let l = self.l.max(other.l);
        let r = self.r.min(other.r);
        let b = self.b.max(other.b);
        let t = self.t.min(other.t);
        if l <= r && b <= t {
            Some(Bbox2 { l, r, b, t })
        } else {
            None
        }
    }
}

impl<T: Float> From<Viewport> for Bbox2<T> {
    fn from(viewport: Viewport) -> Self {
        Bbox2 {
            l: T::zero(),
            r: T::from(viewport.width).unwrap(),
            b: T::zero(),
            t: T::from(viewport.height).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bbox3<T: Float> {
    pub l: T,
    pub r: T,
    pub b: T,
    pub t: T,
    pub n: T,
    pub f: T,
}

impl<T: Float> Bbox3<T> {
    pub fn new(l: T, r: T, b: T, t: T, n: T, f: T) -> Self {
        Bbox3 { l, r, b, t, n, f }
    }

    pub fn from_vector3(vectors: &Vec<&Vector3<T>>) -> Self {
        let mut l = vectors[0][0];
        let mut r = vectors[0][0];
        let mut b = vectors[0][1];
        let mut t = vectors[0][1];
        let mut n = vectors[0][2];
        let mut f = vectors[0][2];
        for p in vectors {
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
            if p[2] < n {
                n = p[2];
            }
            if p[2] > f {
                f = p[2];
            }
        }
        Bbox3 { l, r, b, t, n, f }
    }

    pub fn intersect(&self, other: &Bbox3<T>) -> Option<Bbox3<T>> {
        let l = self.l.max(other.l);
        let r = self.r.min(other.r);
        let b = self.b.max(other.b);
        let t = self.t.min(other.t);
        let n = self.n.max(other.n);
        let f = self.f.min(other.f);
        if l <= r && b <= t && n <= f {
            Some(Bbox3 { l, r, b, t, n, f })
        } else {
            None
        }
    }
}
