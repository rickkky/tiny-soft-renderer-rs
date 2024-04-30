use crate::{color::Color, triangle::travel_triangle_barycentric};
use interpolate::Interpolate;
use nalgebra::{Vector3, Vector4};
use num_traits::Float;

pub trait Shader<V: Interpolate> {
    fn vertex_shader(&self, index: usize) -> VertexFs<V>;

    fn fragment_shader(&self, vertex: VertexFs<V>) -> Color;
}

#[derive(Debug, Copy, Clone)]
pub struct VertexFs<V: Interpolate> {
    pub position: Vector4<f32>,

    pub varying: V,
}

impl<V: Interpolate> Interpolate for VertexFs<V> {
    fn linear_interpolate<F: Float>(
        v_0: &Self,
        v_1: &Self,
        linear_coord: &nalgebra::Vector2<F>,
    ) -> Self {
        let position = Interpolate::linear_interpolate(&v_0.position, &v_1.position, linear_coord);
        let varying = V::linear_interpolate(&v_0.varying, &v_1.varying, linear_coord);
        Self { position, varying }
    }

    fn barycentric_interpolate<F: Float>(
        v_0: &Self,
        v_1: &Self,
        v_2: &Self,
        bary_coord: &Vector3<F>,
    ) -> Self {
        let position = Interpolate::barycentric_interpolate(
            &v_0.position,
            &v_1.position,
            &v_2.position,
            bary_coord,
        );
        let varying =
            V::barycentric_interpolate(&v_0.varying, &v_1.varying, &v_2.varying, bary_coord);
        Self { position, varying }
    }
}

impl<V: Interpolate> VertexFs<V> {
    pub fn collect_triangle_barycentric(v_0: &Self, v_1: &Self, v_2: &Self) -> Vec<Self> {
        let mut vertices = Vec::new();
        travel_triangle_barycentric(
            &v_0.position.xy(),
            &v_1.position.xy(),
            &v_2.position.xy(),
            |p, bary_coord| {
                let z = f32::barycentric_interpolate(
                    &v_0.position.z,
                    &v_1.position.z,
                    &v_2.position.z,
                    &bary_coord,
                );
                let position = Vector4::new(p.x as f32, p.y as f32, z, 1.0);
                let varying = V::barycentric_interpolate(
                    &v_0.varying,
                    &v_1.varying,
                    &v_2.varying,
                    &bary_coord,
                );
                vertices.push(Self { position, varying });
            },
        );
        vertices
    }
}
