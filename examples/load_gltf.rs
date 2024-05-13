use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use nalgebra::{Vector3, Vector4};
use tinyrenderer::{
    basetype::Viewport,
    color::Color,
    interpolate::Interpolate,
    pass::RenderPass,
    pipeline::{CullMode, DepthCompare, Pipeline},
    shader::{FsPayload, ShaderProgram, VsOutput},
};

#[derive(Interpolate)]
struct Varying {
    pub color: Color,
}

struct Renderer {
    vertices: Vec<Vector4<f32>>,

    normals: Vec<Vector3<f32>>,

    colors: Vec<Color>,

    light_dir: Vector3<f32>,
}

impl ShaderProgram for Renderer {
    type Varying = Varying;

    fn vertex_shader(&self, index: usize) -> VsOutput<Self::Varying> {
        let normal = self.normals[index];
        let intensity = self.light_dir.dot(&normal).max(0.0);
        let color = self.colors[index / 3] * intensity;
        VsOutput {
            position: self.vertices[index],
            varying: Self::Varying { color },
        }
    }

    fn fragment_shader(&self, payload: FsPayload<Self::Varying>) -> Color {
        payload.varying.color
    }
}

const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 800;

fn transform_position(position: [f32; 3]) -> Vector4<f32> {
    let ratio = 10.0;
    let offset_y = -200.0;
    Vector4::new(
        position[0] * WIN_WIDTH as f32 * ratio + WIN_WIDTH as f32 / 2.0,
        position[1] * WIN_HEIGHT as f32 * ratio + WIN_HEIGHT as f32 / 2.0 + offset_y,
        position[2],
        1.0,
    )
}

pub fn main() {
    let app = fltk::app::App::default();
    let mut win = fltk::window::Window::new(100, 100, WIN_WIDTH as i32, WIN_HEIGHT as i32, "Test");

    let viewport = Viewport::new(0, 0, WIN_WIDTH, WIN_HEIGHT);
    let mut pass = RenderPass::new(viewport);

    let (document, buffers, _images) = gltf::import("models/Avocado/glTF/Avocado.gltf").unwrap();

    win.draw(move |_| {
        pass.clear();

        for mesh in document.meshes() {
            for primitive in mesh.primitives() {
                if primitive.mode() != gltf::mesh::Mode::Triangles {
                    panic!("Only support triangles.");
                }

                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();
                let indices = reader
                    .read_indices()
                    .unwrap()
                    .into_u32()
                    .collect::<Vec<u32>>();
                let raw_normals = reader.read_normals().unwrap().collect::<Vec<[f32; 3]>>();

                let mut vertices = Vec::new();
                let mut normals = Vec::new();
                let mut colors = Vec::new();

                for i in (0..indices.len()).step_by(3) {
                    let p_0 = transform_position(positions[indices[i] as usize]);
                    let p_1 = transform_position(positions[indices[i + 1] as usize]);
                    let p_2 = transform_position(positions[indices[i + 2] as usize]);
                    vertices.push(p_0);
                    vertices.push(p_1);
                    vertices.push(p_2);
                    let n_0 = Vector3::from(raw_normals[indices[i] as usize]);
                    let n_1 = Vector3::from(raw_normals[indices[i + 1] as usize]);
                    let n_2 = Vector3::from(raw_normals[indices[i + 2] as usize]);
                    normals.push(n_0);
                    normals.push(n_1);
                    normals.push(n_2);
                    let color = Color::new(0.5, 0.5, 0.5, 1.0);
                    colors.push(color);
                }

                let times = vertices.len();
                let renderer = Renderer {
                    vertices,
                    normals,
                    colors,
                    light_dir: Vector3::new(0.0, 0.0, -1.0),
                };
                let mut pipeline = Pipeline {
                    program: &renderer,
                    cull_mode: CullMode::None,
                    depth_write_enable: true,
                    depth_compare: DepthCompare::Less,
                };
                pass.draw::<Varying>(&mut pipeline, times);
            }
        }

        fltk::draw::draw_image(
            &pass.frame_buffer,
            0,
            0,
            WIN_WIDTH as i32,
            WIN_HEIGHT as i32,
            fltk::enums::ColorDepth::Rgba8,
        )
        .unwrap();
    });

    win.end();
    win.show();
    app.run().unwrap();
}
