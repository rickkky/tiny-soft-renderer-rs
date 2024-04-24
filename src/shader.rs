use crate::color::Color;
use nalgebra::Vector4;

pub struct VertexFs {
    pub position: Vector4<f32>,

    pub varyingBuffer: Vec<f32>,
}

type VertesShader = fn(index: usize) -> VertexFs;

type FragmentShader = fn(v: VertexFs) -> Color;
