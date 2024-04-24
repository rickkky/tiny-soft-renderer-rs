use nalgebra::{ArrayStorage, Const, Matrix, Scalar, Vector2, Vector3};
use num_traits::{Float, NumCast, Zero};
use std::ops::{Add, Mul};

pub trait Interpolate {
    fn linear_interpolate<F: Float>(v_0: &Self, v_1: &Self, linear_coord: &Vector2<F>) -> Self;

    fn barycentric_interpolate<F: Float>(
        v_0: &Self,
        v_1: &Self,
        v_2: &Self,
        bary_coord: &Vector3<F>,
    ) -> Self;
}

#[inline]
pub fn linear_interpolate<F: Float, T>(v_0: T, v_1: T, linear_coord: &Vector2<F>) -> T
where
    T: Add<T, Output = T> + Mul<F, Output = T>,
{
    v_0 * linear_coord[0] + v_1 * linear_coord[1]
}

#[inline]
pub fn barycentric_interpolate<F: Float, T>(v_0: T, v_1: T, v_2: T, bary_coord: &Vector3<F>) -> T
where
    T: Add<T, Output = T> + Mul<F, Output = T>,
{
    v_0 * bary_coord[0] + v_1 * bary_coord[1] + v_2 * bary_coord[2]
}

impl Interpolate for () {
    #[inline(always)]
    fn linear_interpolate<F: Float>(_: &Self, _: &Self, _: &Vector2<F>) -> Self {
        ()
    }

    #[inline(always)]
    fn barycentric_interpolate<F: Float>(_: &Self, _: &Self, _: &Self, _: &Vector3<F>) -> Self {
        ()
    }
}

macro_rules! impl_interpolate_for_primitive {
    ($($t:ty),+) => {
        $(
            impl Interpolate for $t {
                #[inline(always)]
                fn linear_interpolate<F: Float>(
                    v_0: &$t,
                    v_1: &$t,
                    linear_coord: &Vector2<F>,
                ) -> $t {
                     <$t as NumCast>::from(
                        F::from(*v_0).unwrap() * linear_coord[0] +
                        F::from(*v_1).unwrap() * linear_coord[1]
                     ).unwrap()
                }

                #[inline(always)]
                fn barycentric_interpolate<F: Float>(
                    v_0: &$t,
                    v_1: &$t,
                    v_2: &$t,
                    bary_coord: &Vector3<F>,
                ) -> $t {
                    <$t as NumCast>::from(
                        F::from(*v_0).unwrap() * bary_coord[0] +
                        F::from(*v_1).unwrap() * bary_coord[1] +
                        F::from(*v_2).unwrap() * bary_coord[2]
                    ).unwrap()
                }
            }
        )+
    }
}

impl_interpolate_for_primitive!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64);

impl<T, const R: usize, const C: usize> Interpolate
    for Matrix<T, Const<R>, Const<C>, ArrayStorage<T, R, C>>
where
    T: Scalar + Zero + Interpolate,
{
    #[inline]
    fn linear_interpolate<F: Float>(v_0: &Self, v_1: &Self, linear_coord: &Vector2<F>) -> Self {
        let mut res = Self::zeros();
        for i in 0..R {
            for j in 0..C {
                res[(i, j)] = T::linear_interpolate(&v_0[(i, j)], &v_1[(i, j)], linear_coord);
            }
        }
        res
    }

    #[inline]
    fn barycentric_interpolate<F: Float>(
        v_0: &Self,
        v_1: &Self,
        v_2: &Self,
        bary_coord: &Vector3<F>,
    ) -> Self {
        let mut res = Self::zeros();
        for i in 0..R {
            for j in 0..C {
                res[(i, j)] = T::barycentric_interpolate(
                    &v_0[(i, j)],
                    &v_1[(i, j)],
                    &v_2[(i, j)],
                    bary_coord,
                );
            }
        }
        res
    }
}

pub use interpolate_derive::Interpolate;
