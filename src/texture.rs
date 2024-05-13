pub enum SamplingMethod {
    Nearest,
    Bilinear,
}

pub enum EdgeBehavior {
    Clamp,
    Wrap,
}

impl EdgeBehavior {
    fn edge(self, u: f32, v: f32) -> (f32, f32) {
        match self {
            EdgeBehavior::Clamp => (u.min(1.0).max(0.0), v.min(1.0).max(0.0)),
            EdgeBehavior::Wrap => (u.fract(), v.fract()),
        }
    }
}

pub struct Texture {
    pub width: u32,

    pub height: u32,

    pub stride: u32,

    pub data: Vec<f32>,
}

impl Texture {
    pub fn new(width: u32, height: u32, stride: u32) -> Self {
        let data = vec![0.0; (width * height * stride) as usize];
        Self {
            width,
            height,
            stride,
            data,
        }
    }

    pub fn get_texel(&self, x: u32, y: u32) -> &[f32] {
        let index = (x + y * self.width) as usize * self.stride as usize;
        &self.data[index..index + self.stride as usize]
    }

    pub fn sample(&self, u: f32, v: f32, method: SamplingMethod, edge: EdgeBehavior) -> Vec<f32> {
        let (u, v) = edge.edge(u, v);

        match method {
            SamplingMethod::Nearest => {
                let x = (u * (self.width - 1) as f32).round() as u32;
                let y = (v * (self.height - 1) as f32).round() as u32;
                self.get_texel(x, y).to_vec()
            }
            SamplingMethod::Bilinear => {
                let u = (u * (self.width - 1) as f32) + 0.5;
                let v = (v * (self.height - 1) as f32) + 0.5;
                let x = u.floor() as u32;
                let y = v.floor() as u32;

                let u_ratio = u - x as f32;
                let v_ratio = v - y as f32;
                let u_opposite = 1.0 - u_ratio;
                let v_opposite = 1.0 - v_ratio;

                let texel_00 = self.get_texel(x, y);
                let texel_10 = self.get_texel(x + 1, y);
                let texel_01 = self.get_texel(x, y + 1);
                let texel_11 = self.get_texel(x + 1, y + 1);

                let mut result = vec![0.0; self.stride as usize];
                for i in 0..self.stride as usize {
                    result[i] = texel_00[i] * u_opposite * v_opposite
                        + texel_10[i] * u_ratio * v_opposite
                        + texel_01[i] * u_opposite * v_ratio
                        + texel_11[i] * u_ratio * v_ratio;
                }

                result
            }
        }
    }
}
