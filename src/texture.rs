use nalgebra::Vector2;

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

pub struct Texture {
    pub width: usize,

    pub height: usize,

    pub stride: usize,

    pub data: Vec<f32>,
}

impl Texture {
    pub fn new(width: usize, height: usize, stride: usize) -> Self {
        let data = vec![0.0; width * height * stride];
        Self {
            width,
            height,
            stride,
            data,
        }
    }

    pub fn get_texel(&self, p: &Vector2<usize>) -> &[f32] {
        let index = (p.x + p.y * self.width) * self.stride;
        &self.data[index..index + self.stride]
    }

    pub fn set_texel(&mut self, p: &Vector2<usize>, value: &[f32]) {
        let index = (p.x + p.y * self.width) * self.stride;
        self.data[index..index + self.stride].copy_from_slice(value);
    }

    pub fn sample(
        &self,
        uv: &Vector2<f32>,
        method: SamplingMethod,
        edge: EdgeBehavior,
    ) -> Vec<f32> {
        let uv = edge.edge(uv);

        match method {
            SamplingMethod::Nearest => {
                let x = (uv.x * (self.width - 1) as f32).round() as usize;
                let y = (uv.y * (self.height - 1) as f32).round() as usize;
                self.get_texel(&Vector2::new(x, y)).to_vec()
            }
            SamplingMethod::Bilinear => {
                let x = uv.x * (self.width - 1) as f32;
                let y = uv.y * (self.height - 1) as f32;

                let x_0 = usize::max(x.floor() as usize, self.width as usize - 2);
                let y_0 = usize::max(y.floor() as usize, self.height as usize - 2);
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

                t_00.iter()
                    .zip(t_01.iter())
                    .zip(t_10.iter())
                    .zip(t_11.iter())
                    .map(|(((v_00, v_01), v_10), v_11)| {
                        p_00 * v_00 + p_01 * v_01 + p_10 * v_10 + p_11 * v_11
                    })
                    .collect()
            }
        }
    }
}
