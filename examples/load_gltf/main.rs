use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use nalgebra::{Vector2, Vector3, Vector4};
use tinyrenderer::{
    common::{basetype::Viewport, color::Color},
    interpolate::Interpolate,
    rasterizer::{
        pass::RenderPass,
        pipeline::{CullMode, DepthCompare, Pipeline},
        shader::{FsPayload, Shader, VsOutput},
        texture::{Data, EdgeBehavior, SamplingMethod, Texture},
    },
};

mod loader;

#[derive(Debug, Interpolate)]
struct Varying {
    pub tex_coord: Vector2<f32>,
}

struct Program<'a> {
    primitive: &'a loader::Primitive,

    texture: Option<&'a Texture<Color>>,

    light_dir: Vector3<f32>,
}

impl<'a> Shader for Program<'a> {
    type Varying = Varying;

    fn vertex_shader(&self, index: usize) -> VsOutput<Self::Varying> {
        let vertex = &self.primitive.vertices[index];
        let position = &vertex.position;
        let ratio = 12.0;
        let offset_y = -0.25;
        let position = Vector4::new(
            position[0] * ratio,
            position[1] * ratio + offset_y,
            position[2] + 0.5,
            1.0,
        );
        VsOutput {
            position,
            varying: Self::Varying {
                tex_coord: vertex.tex_coord.unwrap(),
            },
        }
    }

    fn fragment_shader(&self, payload: FsPayload<Self::Varying>) -> Color {
        let tex_coord = payload.varying.tex_coord;
        if let Some(texture) = self.texture {
            let color = texture.sample(&tex_coord, SamplingMethod::Bilinear, EdgeBehavior::Clamp);
            color
        } else {
            Color::WHITE
        }
    }
}

const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 800;

pub fn main() {
    let app = fltk::app::App::default();
    let mut win = fltk::window::Window::new(100, 100, WIN_WIDTH as i32, WIN_HEIGHT as i32, "Test");

    let viewport = Viewport::new(WIN_WIDTH, WIN_HEIGHT);
    let mut pass = RenderPass::new(viewport);

    let (document, buffers, images) = gltf::import("models/Avocado/glTF/Avocado.gltf").unwrap();

    win.draw(move |_| {
        pass.clear();

        let texture_map = loader::load_textures(&document, &images);
        let meshes = loader::load_meshs(&document, &buffers);
        println!("meshes: {:?}", meshes.len());
        for mesh in meshes {
            println!("mesh: {:?}", mesh.primitives.len());
            for primitive in mesh.primitives {
                println!("primitive: {:?}", primitive.vertices.len());
                let texture = primitive
                    .texture_index
                    .map(|index| texture_map.get(&index).unwrap());
                let program = Program {
                    primitive: &primitive,
                    texture,
                    light_dir: Vector3::new(0.0, 0.0, 1.0),
                };
                let mut pipeline = Pipeline {
                    program: &program,
                    cull_mode: CullMode::None,
                    depth_write_enable: true,
                    depth_compare: DepthCompare::Less,
                };
                pass.draw::<Varying>(&mut pipeline, primitive.vertices.len());
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
