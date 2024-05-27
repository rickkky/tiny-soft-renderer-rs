use crate::common::color::Color;
use nalgebra::{Vector2, Vector3, Vector4};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vector4<f32>,

    pub normal: Option<Vector3<f32>>,

    pub color: Option<Color>,

    pub texcoord: Option<Vector2<f32>>,
}
