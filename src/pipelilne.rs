use crate::shader::ShaderProgramStruct;
use interpolate::Interpolate;

pub enum CullMode {
    None,
    Front,
    Back,
}

pub enum DepthCompare {
    Always,
    Never,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
}

pub struct Pipeline<'a, V: Interpolate> {
    pub program: &'a ShaderProgramStruct<'a, V>,

    pub cull_mode: CullMode,

    pub depth_write_enable: bool,

    pub depth_compare: DepthCompare,
}
