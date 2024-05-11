use crate::color::Color;
use interpolate::Interpolate;
use nalgebra::{Vector3, Vector4};

#[derive(Debug, Copy, Clone, Interpolate)]
pub struct Varying<I: Interpolate> {
    pub position: Vector4<f32>,

    pub extra: I,
}

#[derive(Debug, Copy, Clone)]
pub struct FsPayload<I: Interpolate> {
    pub varying: Varying<I>,

    pub bary_coord: Vector3<f32>,
}

#[allow(unused_variables)]
pub trait ShaderProgram {
    type VaryingExtra: Interpolate;

    fn vertex_shader(&self, index: usize) -> Varying<Self::VaryingExtra> {
        unimplemented!()
    }

    fn fragment_shader(&self, payload: FsPayload<Self::VaryingExtra>) -> Color {
        unimplemented!()
    }
}
