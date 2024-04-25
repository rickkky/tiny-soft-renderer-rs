use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use nalgebra::{Vector2, Vector3};
use rand::Rng;
use tinyrenderer::{
    basetype::Viewport, color::Color, rasterizer::Raster, triangle::travel_triangle_barycentric,
};

const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 800;

pub fn main() {
    let app = fltk::app::App::default();
    let mut win = fltk::window::Window::new(100, 100, WIN_WIDTH as i32, WIN_HEIGHT as i32, "Test");

    let viewport = Viewport::new(0, 0, WIN_WIDTH, WIN_HEIGHT);
    let mut renderer = Raster::new(viewport);

    win.draw(move |_| {
        renderer.clear();

        let mut rng = rand::thread_rng();

        for i in 0..16 {
            let row = i / 4;
            let col = i % 4;
            let x_min = (WIN_WIDTH / 4) * col;
            let x_max = (WIN_WIDTH / 4) * (col + 1);
            let y_min = (WIN_HEIGHT / 4) * row;
            let y_max = (WIN_HEIGHT / 4) * (row + 1);
            renderer.draw_line(
                &Vector2::new(x_max as f32, y_min as f32),
                &Vector2::new(x_max as f32, y_max as f32),
                &Color::WHITE,
            );
            renderer.draw_line(
                &Vector2::new(x_min as f32, y_max as f32),
                &Vector2::new(x_max as f32, y_max as f32),
                &Color::WHITE,
            );
            let p_0 = Vector2::new(
                rng.gen_range(x_min..x_max) as f32,
                rng.gen_range(y_min..y_max) as f32,
            );
            let p_1 = Vector2::new(
                rng.gen_range(x_min..x_max) as f32,
                rng.gen_range(y_min..y_max) as f32,
            );
            let p_2 = Vector2::new(
                rng.gen_range(x_min..x_max) as f32,
                rng.gen_range(y_min..y_max) as f32,
            );
            let color = Color::new_rand();
            let draw = |p: Vector2<i32>, _: Vector3<f32>| {
                renderer.draw_pixel(&p, &color);
            };
            travel_triangle_barycentric(&p_0, &p_1, &p_2, draw);
        }

        fltk::draw::draw_image(
            &renderer.frame_buffer,
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
