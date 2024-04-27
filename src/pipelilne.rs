use interpolate::Interpolate;

use crate::shader::{FragmentShader, VertesShader};

pub struct Pipeline<V: Interpolate> {
    pub vertex_shader: VertesShader<V>,

    pub fragment_shader: FragmentShader<V>,
}
