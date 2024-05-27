use std::collections::HashMap;

use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use nalgebra::{Vector2, Vector3, Vector4};
use tinyrenderer::{
    basetype::Viewport,
    color::Color,
    interpolate::Interpolate,
    pass::RenderPass,
    pipeline::{CullMode, DepthCompare, Pipeline},
    shader::{FsPayload, ShaderProgram, VsOutput},
    texture::{Data, Texture},
};

#[derive(Debug, Interpolate)]
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
        let color = self.colors[index] * intensity;
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
    let ratio = 12.0;
    let offset_y = -0.25;
    Vector4::new(
        position[0] * ratio,
        position[1] * ratio + offset_y,
        position[2] + 0.5,
        1.0,
    )
}

pub fn main() {
    let app = fltk::app::App::default();
    let mut win = fltk::window::Window::new(100, 100, WIN_WIDTH as i32, WIN_HEIGHT as i32, "Test");

    let viewport = Viewport::new(WIN_WIDTH, WIN_HEIGHT);
    let mut pass = RenderPass::new(viewport);

    let (document, buffers, images) = gltf::import("models/Avocado/glTF/Avocado.gltf").unwrap();

    let mut texture_map = HashMap::new();

    for texture_origin in document.textures() {
        let source = texture_origin.source();
        let image = images.get(source.index()).unwrap();

        let texture = Texture::<Color>::from(image);

        texture_map.insert(texture_origin.index(), texture);
    }

    win.draw(move |_| {
        pass.clear();

        for mesh in document.meshes() {
            for primitive in mesh.primitives() {
                if primitive.mode() != gltf::mesh::Mode::Triangles {
                    panic!("Only support triangles.");
                }

                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                let mut raw_positions: Vec<[f32; 3]> = Vec::new();
                let mut raw_normals: Vec<[f32; 3]> = Vec::new();
                let mut raw_colors: Vec<[f32; 3]> = Vec::new();
                let mut raw_texcoords: Vec<[f32; 2]> = Vec::new();

                for (semantic, _) in primitive.attributes() {
                    match semantic {
                        gltf::Semantic::Positions => {
                            raw_positions = reader.read_positions().unwrap().collect();
                        }
                        gltf::Semantic::Normals => {
                            raw_normals = reader.read_normals().unwrap().collect();
                        }
                        gltf::Semantic::Colors(set) => {
                            raw_colors = reader.read_colors(set).unwrap().into_rgb_f32().collect();
                        }
                        gltf::Semantic::TexCoords(set) => {
                            raw_texcoords =
                                reader.read_tex_coords(set).unwrap().into_f32().collect();
                        }
                        _ => {}
                    }
                }

                let Some(indices) = reader.read_indices() else {
                    continue;
                };
                let indices = indices.into_u32();

                let mut positions = Vec::new();
                let mut normals = Vec::new();
                let mut colors = Vec::new();
                let mut texcoords = Vec::new();

                for index in indices {
                    let vertex = transform_position(raw_positions[index as usize]);
                    positions.push(vertex);
                    let normal = Vector3::from(raw_normals[index as usize]);
                    normals.push(normal);
                    // let color = Color {
                    //     r: raw_colors[index as usize][0],
                    //     g: raw_colors[index as usize][1],
                    //     b: raw_colors[index as usize][2],
                    //     a: 1.0,
                    // };
                    // colors.push(color);
                    let texcoord = Vector2::from(raw_texcoords[index as usize]);
                    texcoords.push(texcoord);
                    let color = texture_map
                        .get(
                            &primitive
                                .material()
                                .pbr_metallic_roughness()
                                .base_color_texture()
                                .unwrap()
                                .texture()
                                .index(),
                        )
                        .unwrap()
                        .sample(
                            &texcoord,
                            tinyrenderer::texture::SamplingMethod::Bilinear,
                            tinyrenderer::texture::EdgeBehavior::Clamp,
                        );
                    colors.push(color);
                }

                let vertex_count = positions.len();
                let renderer = Renderer {
                    vertices: positions,
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
                pass.draw::<Varying>(&mut pipeline, vertex_count);
            }
        }

        fltk::draw::draw_image(
            &Into::<Data>::into(&pass.frame_texture).pixels,
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
