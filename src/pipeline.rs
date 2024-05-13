use crate::shader::ShaderProgram;
use interpolate::Interpolate;

pub enum CullMode {
    None,
    Front,
    Back,
}

#[derive(Debug, Clone, Copy)]
pub enum DepthCompare {
    Never,
    Always,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
}

impl DepthCompare {
    pub fn test(self, depth: f32, prev_depth: f32) -> bool {
        match self {
            DepthCompare::Never => false,
            DepthCompare::Always => true,
            DepthCompare::Less => depth < prev_depth,
            DepthCompare::LessEqual => depth <= prev_depth,
            DepthCompare::Greater => depth > prev_depth,
            DepthCompare::GreaterEqual => depth >= prev_depth,
            DepthCompare::Equal => depth == prev_depth,
            DepthCompare::NotEqual => depth != prev_depth,
        }
    }
}

pub struct Pipeline<'a, V: Interpolate = ()> {
    pub program: &'a dyn ShaderProgram<Varying = V>,

    pub cull_mode: CullMode,

    pub depth_write_enable: bool,

    pub depth_compare: DepthCompare,
}
