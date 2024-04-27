use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use nalgebra::Vector2;
use tinyrenderer::{basetype::Viewport, color::Color, rasterizer::Rasterizer};

const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 800;

pub fn main() {
    let app = fltk::app::App::default();
    let mut win = fltk::window::Window::new(100, 100, WIN_WIDTH as i32, WIN_HEIGHT as i32, "Test");

    let viewport = Viewport::new(0, 0, WIN_WIDTH, WIN_HEIGHT);
    let mut renderer = Rasterizer::new(viewport);

    win.draw(move |_| {
        renderer.clear();

        let x_center = WIN_WIDTH as f32 / 2.0;
        let y_center = WIN_HEIGHT as f32 / 2.0;
        let r_0 = WIN_WIDTH as f32 / 8.0;
        let r_1 = 3.0 * r_0;
        for i in 0..48 {
            let angle = i as f32 * std::f32::consts::PI / (2.0 * 12.0);
            let cos = angle.cos();
            let sin = angle.sin();
            renderer.draw_line(
                &Vector2::new(x_center + r_0 * cos, y_center + r_0 * sin),
                &Vector2::new(x_center + r_1 * cos, y_center + r_1 * sin),
                &Color::new_rand(),
            );
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
