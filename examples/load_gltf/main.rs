use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use nalgebra::{Matrix4, Point3, Vector2, Vector3};
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
    pub normal: Vector3<f32>,

    pub tex_coord: Vector2<f32>,
}

struct Program<'a> {
    primitive: &'a loader::Primitive,

    texture: Option<&'a Texture<Color>>,

    matrix: &'a Matrix4<f32>,

    light_dir: Vector3<f32>,
}

impl<'a> Shader for Program<'a> {
    type Varying = Varying;

    fn vertex_shader(&self, index: usize) -> VsOutput<Self::Varying> {
        let vertex = &self.primitive.vertices[index];
        let position = &vertex.position;
        println!("position 0: {:?}", position);
        let position = self.matrix * position;
        VsOutput {
            position,
            varying: Self::Varying {
                normal: vertex.normal.unwrap(),
                tex_coord: vertex.tex_coord.unwrap(),
            },
        }
    }

    fn fragment_shader(&self, payload: FsPayload<Self::Varying>) -> Color {
        let normal = payload.varying.normal.normalize();
        let intensity = normal.dot(&self.light_dir).max(0.0);
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
    let texture_map = loader::load_textures(&document, &images);
    let meshes = loader::load_meshs(&document, &buffers);

    let perspective = Matrix4::new_perspective(1.0, 10.0_f32.to_radians(), 0.1, 1.0);
    let transform = Matrix4::new(
        -0.7289686799049377,
        0.0,
        -0.6845470666885376,
        0.0,
        -0.4252049028873444,
        0.7836934328079224,
        0.4527972936630249,
        0.0,
        0.5364750623703003,
        0.6211478114128113,
        -0.571287989616394,
        0.0,
        400.1130065917969,
        463.2640075683594,
        -431.0780334472656,
        1.0,
    );
    let look_at = Matrix4::look_at_rh(
        &Point3::new(0.0, 0.0, 1.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vector3::new(0.0, 1.0, 0.0),
    );
    let translate = Matrix4::new_translation(&Vector3::new(0.0, 0.0, -1.0));
    let matrix = perspective;

    win.draw(move |_| {
        pass.clear();

        println!("meshes: {:?}", meshes.len());
        for mesh in &meshes {
            println!("primitives: {:?}", mesh.primitives.len());
            for primitive in &mesh.primitives {
                println!("vertices: {:?}", primitive.vertices.len());
                // println!("vertices: {:?}", primitive.vertices);
                let texture = primitive
                    .texture_index
                    .map(|index| texture_map.get(&index).unwrap());

                let program = Program {
                    primitive: &primitive,
                    texture,
                    matrix: &matrix.clone(),
                    light_dir: Vector3::new(0.0, 0.0, -1.0),
                };

                let mut pipeline = Pipeline {
                    program: &program,
                    cull_mode: CullMode::None,
                    depth_write_enable: true,
                    depth_compare: DepthCompare::Greater,
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
