use nalgebra::{Vector3, Vector4};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vector4<f32>,

    pub normal: Option<Vector3<f32>>,

    pub textcoord: Option<Vector3<f32>>,

    pub color: Option<Vector4<f32>>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
}
