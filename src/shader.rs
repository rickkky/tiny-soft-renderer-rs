use crate::{color::Color, triangle::travel_triangle_barycentric};
use interpolate::Interpolate;
use nalgebra::{Vector2, Vector3, Vector4};

pub trait ShaderProgramTrait {
    type Varying: Interpolate;

    fn vertex_shader(&self, index: usize) -> VertexFs<Self::Varying>;

    fn fragment_shader(&self, vertex: VertexFs<Self::Varying>) -> Color;
}

pub struct ShaderProgramStruct<'a, V: Interpolate> {
    pub vertex_shader: Box<dyn FnMut(usize) -> VertexFs<V> + 'a>,

    pub fragment_shader: Box<dyn FnMut(VertexFs<V>) -> Color + 'a>,
}

#[derive(Debug, Copy, Clone, Interpolate)]
pub struct VertexFs<V: Interpolate> {
    pub position: Vector4<f32>,

    pub varying: V,
}

impl<V: Interpolate> VertexFs<V> {
    pub fn collect_triangle_barycentric(v_0: &Self, v_1: &Self, v_2: &Self) -> Vec<Self> {
        let mut vertices = Vec::new();
        let action = |p: Vector2<i32>, bary_coord: Vector3<f32>| {
            let z = f32::barycentric_interpolate(
                &v_0.position.z,
                &v_1.position.z,
                &v_2.position.z,
                &bary_coord,
            );
            let position = Vector4::new(p.x as f32, p.y as f32, z, 1.0);
            let varying =
                V::barycentric_interpolate(&v_0.varying, &v_1.varying, &v_2.varying, &bary_coord);
            vertices.push(Self { position, varying });
        };
        travel_triangle_barycentric(
            &v_0.position.xy(),
            &v_1.position.xy(),
            &v_2.position.xy(),
            action,
        );
        vertices
    }
}
