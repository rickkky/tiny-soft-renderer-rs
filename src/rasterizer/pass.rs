use crate::{
    common::{
        basetype::{Bbox2, Bbox3, Viewport},
        color::Color,
    },
    rasterizer::{
        line::{clip_line, travel_line_bresenham},
        pipeline::{DepthCompare, Pipeline},
        shader::{FsPayload, VsOutput},
        texture::Texture,
        triangle::travel_triangle_barycentric,
    },
};
use interpolate::Interpolate;
use nalgebra::{Vector2, Vector3, Vector4};

pub struct RenderPass {
    pub viewport: Viewport,

    pub frame_texture: Texture<Color>,

    pub depth_texture: Texture<Option<f32>>,
}

impl RenderPass {
    pub fn new(viewport: Viewport) -> Self {
        Self {
            viewport,
            frame_texture: Texture::new(viewport.width, viewport.height),
            depth_texture: Texture::new(viewport.width, viewport.height),
        }
    }

    pub fn clear(&mut self) {
        self.frame_texture.data.fill(Color::WHITE);
        self.depth_texture.data.fill(None);
    }

    pub fn draw_pixel(&mut self, p: &Vector2<i32>, color: &Color) {
        let (width, height) = (self.viewport.width as i32, self.viewport.height as i32);
        let (x, y) = (p.x, p.y);

        if x < 0 || x >= width || y < 0 || y >= height {
            return;
        }

        let x = x as u32;
        // Flip the y coordinate.
        let y = (height - 1 - y) as u32;

        self.frame_texture
            .set_texel(&Vector2::new(x, y), color.clone());
    }

    pub fn draw_line(&mut self, p_0: &Vector2<f32>, p_1: &Vector2<f32>, color: &Color) {
        if let Some(clip_result) = clip_line(p_0, p_1, &Bbox2::from(self.viewport.clone())) {
            let (p_0, p_1) = clip_result;
            travel_line_bresenham(&p_0, &p_1, |p: Vector2<i32>| {
                self.draw_pixel(&p, color);
            });
        }
    }

    pub fn draw<'a, V: std::fmt::Debug + Interpolate>(
        &mut self,
        pipeline: &mut Pipeline<'a, V>,
        vertex_count: usize,
    ) {
        let mut vs_outputs: Vec<VsOutput<V>> = (0..vertex_count)
            .map(|index| {
                let mut o = pipeline.program.vertex_shader(index);
                o.position = o.position / o.position.w;
                o
            })
            .collect();

        for i in (0..vertex_count).step_by(3) {
            if is_triangle_outside_clipspace_sketchy(
                &vs_outputs[i].position,
                &vs_outputs[i + 1].position,
                &vs_outputs[i + 2].position,
            ) {
                continue;
            }

            for v in &mut vs_outputs[i..=i + 2] {
                v.position = clipspace_to_viewport(&v.position, &self.viewport);
            }

            let v_0 = &vs_outputs[i];
            let v_1 = &vs_outputs[i + 1];
            let v_2 = &vs_outputs[i + 2];

            let fs_payloads = collect_triangle(v_0, v_1, v_2, &self.viewport);

            for payload in fs_payloads {
                let position = payload.position;

                if pipeline.depth_write_enable {
                    let depth = payload.position.z;
                    let prev = self
                        .depth_texture
                        .get_texel(&Vector2::new(position.x as u32, position.y as u32));

                    if prev.is_some()
                        && !DepthCompare::test(pipeline.depth_compare, depth, prev.unwrap())
                    {
                        continue;
                    }

                    self.depth_texture.set_texel(
                        &Vector2::new(position.x as u32, position.y as u32),
                        Some(depth),
                    );
                }

                let position = position.xy().map(|v| v as i32);
                self.draw_pixel(&position, &pipeline.program.fragment_shader(payload));
            }
        }
    }
}

fn is_triangle_outside_clipspace_sketchy(
    p_0: &Vector4<f32>,
    p_1: &Vector4<f32>,
    p_2: &Vector4<f32>,
) -> bool {
    let bbox = Bbox3::from_vector3(&vec![
        &Vector3::new(p_0.x, p_0.y, p_0.z),
        &Vector3::new(p_1.x, p_1.y, p_1.z),
        &Vector3::new(p_2.x, p_2.y, p_2.z),
    ]);
    let clipspace = Bbox3::new(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
    bbox.intersect(&clipspace).is_none()
}

fn clipspace_to_viewport(p: &Vector4<f32>, viewport: &Viewport) -> Vector4<f32> {
    let x = (p.x + 1.0) / 2.0 * viewport.width as f32;
    let y = (p.y + 1.0) / 2.0 * viewport.height as f32;
    let z = p.z;
    let w = p.w;
    Vector4::new(x, y, z, w)
}

fn collect_triangle<V: Interpolate>(
    v_0: &VsOutput<V>,
    v_1: &VsOutput<V>,
    v_2: &VsOutput<V>,
    viewport: &Viewport,
) -> Vec<FsPayload<V>> {
    let mut fs_payloads = Vec::new();

    let action = |p: Vector2<i32>, bary_coord: Vector3<f32>| {
        let z = f32::barycentric_interpolate(
            &v_0.position.z,
            &v_1.position.z,
            &v_2.position.z,
            &bary_coord,
        );
        let position = Vector4::new(p.x as f32, p.y as f32, z, 1.0);

        if !is_position_inside_viewport(&position, viewport) {
            return;
        }

        let varying =
            V::barycentric_interpolate(&v_0.varying, &v_1.varying, &v_2.varying, &bary_coord);

        fs_payloads.push(FsPayload {
            position,
            varying,
            bary_coord,
        });
    };

    travel_triangle_barycentric(
        &v_0.position.xy(),
        &v_1.position.xy(),
        &v_2.position.xy(),
        &Bbox2::<f32>::from(viewport.clone()),
        action,
    );

    fs_payloads
}

fn is_position_inside_viewport(p: &Vector4<f32>, viewport: &Viewport) -> bool {
    p.x >= 0.0
        && p.x < viewport.width as f32
        && p.y >= 0.0
        && p.y < viewport.height as f32
        && p.z >= -1.0
        && p.z < 1.0
}
