use crate::{
    basetype::{Bbox2, Viewport},
    color::Color,
    line::{clip_line, travel_line_bresenham},
    pipeline::{DepthCompare, Pipeline},
    shader::{FsPayload, VsOutput},
    texture::Texture,
    triangle::travel_triangle_barycentric,
};
use interpolate::Interpolate;
use nalgebra::{Vector2, Vector3, Vector4};

pub struct RenderPass {
    pub viewport: Viewport,

    pub frame_texture: Texture,

    pub depth_texture: Texture,
}

impl RenderPass {
    pub fn new(viewport: Viewport) -> Self {
        let pixel_count = (viewport.width * viewport.height) as usize;
        Self {
            viewport,
            frame_texture: Texture::new(viewport.width as usize, viewport.height as usize, 4),
            depth_texture: Texture::new(viewport.width as usize, viewport.height as usize, 1),
        }
    }

    pub fn clear(&mut self) {
        self.frame_texture.data.fill(0.0);
        self.depth_texture.data.fill(f32::MAX);
    }

    pub fn draw_pixel(&mut self, p: &Vector2<i32>, color: &Color) {
        let (offset_x, offset_y) = (self.viewport.offset_x, self.viewport.offset_y);
        let (width, height) = (self.viewport.width as i32, self.viewport.height as i32);
        let (x, y) = (p.x, p.y);

        if x < offset_x || x >= offset_x + width || y < offset_y || y >= offset_y + height {
            return;
        }

        let x = (x - offset_x) as usize;
        // Flip the y coordinate.
        let y = (height - 1 - (y - offset_y)) as usize;
        // let texel = self.frame_texture.get_texel(&Vector2::new(x, y));

        self.frame_texture
            .set_texel(&Vector2::new(x, y), &[color.r, color.g, color.b, color.a]);
    }

    pub fn draw_line(&mut self, p_0: &Vector2<f32>, p_1: &Vector2<f32>, color: &Color) {
        let bbox = Bbox2::new(
            self.viewport.offset_x as f32,
            self.viewport.offset_x as f32 + self.viewport.width as f32,
            self.viewport.offset_y as f32,
            self.viewport.offset_y as f32 + self.viewport.height as f32,
        );
        let clip_result = clip_line(p_0, p_1, &bbox);

        if clip_result.is_none() {
            return;
        }

        let (p_0, p_1) = clip_result.unwrap();
        travel_line_bresenham(&p_0, &p_1, |p: Vector2<i32>| {
            self.draw_pixel(&p, color);
        });
    }

    pub fn draw<'a, V: Interpolate>(
        &mut self,
        pipeline: &mut Pipeline<'a, V>,
        vertex_count: usize,
    ) {
        let varyings: Vec<VsOutput<V>> = (0..vertex_count)
            .map(|index| pipeline.program.vertex_shader(index))
            .collect();

        for i in (0..vertex_count).step_by(3) {
            let v_0 = &varyings[i];
            let v_1 = &varyings[i + 1];
            let v_2 = &varyings[i + 2];
            let collection = collect_triangle(v_0, v_1, v_2);

            for payload in collection {
                let position = payload.position.xy().map(|v| v as i32);

                if pipeline.depth_write_enable {
                    let depth = payload.position.z;
                    let prev = self
                        .depth_texture
                        .get_texel(&Vector2::new(position.x as usize, position.y as usize))[0];

                    if !DepthCompare::test(pipeline.depth_compare, depth, prev) {
                        continue;
                    }

                    self.depth_texture.set_texel(
                        &Vector2::new(position.x as usize, position.y as usize),
                        &[depth],
                    );
                }

                self.draw_pixel(&position, &pipeline.program.fragment_shader(payload));
            }
        }
    }
}

fn collect_triangle<V: Interpolate>(
    v_0: &VsOutput<V>,
    v_1: &VsOutput<V>,
    v_2: &VsOutput<V>,
) -> Vec<FsPayload<V>> {
    let mut vertices = Vec::new();
    let action = |p: Vector2<i32>, bary_coord: Vector3<f32>| {
        let z = f32::barycentric_interpolate(
            &v_0.position.z,
            &v_1.position.z,
            &v_2.position.z,
            &bary_coord,
        );
        let position = Vector4::new(p.x as f32, p.y as f32, z, 1.0);
        let extra =
            V::barycentric_interpolate(&v_0.varying, &v_1.varying, &v_2.varying, &bary_coord);
        vertices.push(FsPayload {
            position,
            varying: extra,
            bary_coord,
        });
    };
    travel_triangle_barycentric(
        &v_0.position.xy(),
        &v_1.position.xy(),
        &v_2.position.xy(),
        action,
    );
    vertices
}
