use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use nalgebra::{Vector2, Vector4};
use tinyrenderer::{
    basetype::Viewport, color::Color, interpolate::Interpolate, rasterizer::Rasterizer,
};

#[derive(Interpolate)]
struct Varying {
    pub color: Color,
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
    let mut rasterizer = Rasterizer::new(viewport);

    let (document, buffers, images) = gltf::import("models/Avocado/glTF/Avocado.gltf").unwrap();

    win.draw(move |_| {
        rasterizer.clear();

        for mesh in document.meshes() {
            for primitive in mesh.primitives() {
                if primitive.mode() != gltf::mesh::Mode::Triangles {
                    panic!("Only support triangles.");
                }

                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                let positions: Vec<[f32; 3]> = reader.read_positions().unwrap().collect();
                let Some(indices) = reader.read_indices() else {
                    continue;
                };
                let indices: Vec<u32> = indices.into_u32().collect();

                let mut vertices = Vec::new();

                for i in (0..indices.len()).step_by(3) {
                    let p_0 = transform_position(positions[indices[i] as usize]);
                    let p_1 = transform_position(positions[indices[i + 1] as usize]);
                    let p_2 = transform_position(positions[indices[i + 2] as usize]);
                    let color = Color::new_rand();
                    let v_0 = VertexFs {
                        position: p_0,
                        varying: Varying { color },
                    };
                    let v_1 = VertexFs {
                        position: p_1,
                        varying: Varying { color },
                    };
                    let v_2 = VertexFs {
                        position: p_2,
                        varying: Varying { color },
                    };
                    vertices
                }
            }
        }

        fltk::draw::draw_image(
            &rasterizer.frame_buffer,
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
