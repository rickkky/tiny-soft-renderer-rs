use nalgebra::{Vector3, Vector4};

pub struct Vertex {
    pub position: Vector4<f32>,

    pub normal: Option<Vector3<f32>>,
}
