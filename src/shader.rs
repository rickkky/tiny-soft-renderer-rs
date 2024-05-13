use crate::color::Color;
use interpolate::Interpolate;
use nalgebra::{Vector3, Vector4};

#[derive(Debug, Copy, Clone, Interpolate)]
pub struct VsOutput<V: Interpolate> {
    pub position: Vector4<f32>,

    pub varying: V,
}

#[derive(Debug, Copy, Clone)]
pub struct FsPayload<V: Interpolate> {
    pub position: Vector4<f32>,

    pub varying: V,

    pub bary_coord: Vector3<f32>,
}

#[allow(unused_variables)]
pub trait ShaderProgram {
    type Varying: Interpolate;

    fn vertex_shader(&self, index: usize) -> VsOutput<Self::Varying> {
        unimplemented!()
    }

    fn fragment_shader(&self, payload: FsPayload<Self::Varying>) -> Color {
        unimplemented!()
    }
}
