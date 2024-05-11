use nalgebra::Scalar;

pub struct Texture<T: Scalar> {
    pub width: u32,

    pub height: u32,

    pub stride: u32,

    pub data: Vec<T>,
}
