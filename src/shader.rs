use crate::color::Color;
use interpolate::Interpolate;
use nalgebra::Vector4;

pub struct VertexFs<V: Interpolate> {
    pub position: Vector4<f32>,

    pub varying: V,
}

pub type VertesShader<V> = fn(index: usize) -> VertexFs<V>;

pub type FragmentShader<V> = fn(v: VertexFs<V>) -> Color;
