use crate::common::color::Color;
use nalgebra::Vector2;

pub use gltf::image::{Data, Format};

pub enum SamplingMethod {
    Nearest,
    Bilinear,
}

pub enum EdgeBehavior {
    Clamp,
    Wrap,
}

impl EdgeBehavior {
    fn edge(self, uv: &Vector2<f32>) -> Vector2<f32> {
        match self {
            EdgeBehavior::Clamp => Vector2::new(uv.x.min(1.0).max(0.0), uv.y.min(1.0).max(0.0)),
            EdgeBehavior::Wrap => Vector2::new(uv.x.fract(), uv.y.fract()),
        }
    }
}

#[derive(Debug)]
pub struct Texture<T> {
    pub width: u32,

    pub height: u32,

    pub data: Vec<T>,
}

impl<T: Clone + Default> Texture<T> {
    pub fn new(width: u32, height: u32) -> Self {
        let data: Vec<T> = vec![T::default(); (width * height) as usize];
        Self {
            width,
            height,
            data,
        }
    }

    pub fn get_texel(&self, p: &Vector2<u32>) -> T {
        self.data[(p.x + p.y * self.width) as usize].clone()
    }

    pub fn set_texel(&mut self, p: &Vector2<u32>, value: T) {
        self.data[(p.x + p.y * self.width) as usize] = value;
    }
}

impl Texture<Color> {
    pub fn sample(&self, uv: &Vector2<f32>, method: SamplingMethod, edge: EdgeBehavior) -> Color {
        let uv = edge.edge(uv);

        match method {
            SamplingMethod::Nearest => {
                let x = (uv.x * (self.width - 1) as f32).round() as u32;
                let y = (uv.y * (self.height - 1) as f32).round() as u32;

                self.get_texel(&Vector2::new(x, y))
            }

            SamplingMethod::Bilinear => {
                let x = uv.x * (self.width - 1) as f32;
                let y = uv.y * (self.height - 1) as f32;

                let x_0 = u32::min(x.floor() as u32, self.width as u32 - 2);
                let y_0 = u32::min(y.floor() as u32, self.height as u32 - 2);
                let x_1 = x_0 + 1;
                let y_1 = y_0 + 1;

                let t_00 = self.get_texel(&Vector2::new(x_0, y_0));
                let t_01 = self.get_texel(&Vector2::new(x_0, y_1));
                let t_10 = self.get_texel(&Vector2::new(x_1, y_0));
                let t_11 = self.get_texel(&Vector2::new(x_1, y_1));

                let dx = x - x_0 as f32;
                let dy = y - y_0 as f32;

                let p_00 = (1.0 - dx) * (1.0 - dy);
                let p_01 = (1.0 - dx) * dy;
                let p_10 = dx * (1.0 - dy);
                let p_11 = dx * dy;

                t_00 * p_00 + t_01 * p_01 + t_10 * p_10 + t_11 * p_11
            }
        }
    }

    pub fn from_vec_u8(data: Vec<u8>, width: u32, height: u32) -> Self {
        let mut texture = Texture::new(width, height);

        for x in 0..(width - 1) {
            for y in 0..(height - 1) {
                let i = ((x + y * width) * 4) as usize;
                let r = data[i] as f32 / 255.0;
                let g = data[i + 1] as f32 / 255.0;
                let b = data[i + 2] as f32 / 255.0;
                let a = data[i + 3] as f32 / 255.0;

                texture.set_texel(&Vector2::new(x, y), Color::new(r, g, b, a));
            }
        }

        texture
    }

    pub fn to_vec_u8(&self) -> Vec<u8> {
        let mut data = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.get_texel(&Vector2::new(x, y));
                data.push((color.r * 255.0) as u8);
                data.push((color.g * 255.0) as u8);
                data.push((color.b * 255.0) as u8);
                data.push((color.a * 255.0) as u8);
            }
        }

        data
    }
}

impl From<&Data> for Texture<Color> {
    fn from(data: &Data) -> Self {
        let mut texture = Texture::new(data.width, data.height);

        for x in 0..(data.width - 1) {
            for y in 0..(data.height - 1) {
                match data.format {
                    Format::R8 => {
                        let i = ((x + y * data.width) * 1) as usize;
                        let r = data.pixels[i] as f32 / 255.0;
                        texture.set_texel(&Vector2::new(x, y), Color::new(r, r, r, 1.0));
                    }

                    Format::R8G8B8 => {
                        let i = ((x + y * data.width) * 3) as usize;
                        let r = data.pixels[i] as f32 / 255.0;
                        let g = data.pixels[i + 1] as f32 / 255.0;
                        let b = data.pixels[i + 2] as f32 / 255.0;
                        texture.set_texel(&Vector2::new(x, y), Color::new(r, g, b, 1.0));
                    }

                    Format::R8G8B8A8 => {
                        let i = ((x + y * data.width) * 4) as usize;
                        let r = data.pixels[i] as f32 / 255.0;
                        let g = data.pixels[i + 1] as f32 / 255.0;
                        let b = data.pixels[i + 2] as f32 / 255.0;
                        let a = data.pixels[i + 3] as f32 / 255.0;
                        texture.set_texel(&Vector2::new(x, y), Color::new(r, g, b, a));
                    }

                    _ => {
                        panic!("Unsupported image format");
                    }
                }
            }
        }

        texture
    }
}

impl Into<Data> for &Texture<Color> {
    fn into(self) -> Data {
        let mut data = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.get_texel(&Vector2::new(x, y));
                data.push((color.r * 255.0) as u8);
                data.push((color.g * 255.0) as u8);
                data.push((color.b * 255.0) as u8);
                data.push((color.a * 255.0) as u8);
            }
        }

        gltf::image::Data {
            width: self.width,
            height: self.height,
            format: Format::R8G8B8A8,
            pixels: data,
        }
    }
}
