use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use nalgebra::Vector2;
use rand::Rng;
use tinyrenderer::{basetype::Viewport, color::Color, renderer::Renderer};

const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 800;

pub fn main() {
    let app = fltk::app::App::default();
    let mut win = fltk::window::Window::new(100, 100, WIN_WIDTH as i32, WIN_HEIGHT as i32, "Test");

    let viewport = Viewport::new(0, 0, WIN_WIDTH, WIN_HEIGHT);
    let mut renderer = Renderer::new(viewport);

    win.draw(move |_| {
        renderer.clear();

        let colors = [Color::WHITE, Color::RED, Color::GREEN, Color::BLUE];

        let x_center = WIN_WIDTH as f32 / 2.0;
        let y_center = WIN_HEIGHT as f32 / 2.0;
        let r_0 = 100.0;
        let r_1 = 300.0;
        for i in 0..16 {
            let angle = i as f32 * std::f32::consts::PI / 8.0;
            let cos = angle.cos();
            let sin = angle.sin();
            renderer.draw_line(
                &Vector2::new(x_center + r_0 * cos, y_center + r_0 * sin),
                &Vector2::new(x_center + r_1 * cos, y_center + r_1 * sin),
                &colors[i % 4],
            );
        }

        // let mut rng = rand::thread_rng();
        // for i in 0..16 {
        //     renderer.draw_line(
        //         &Vector2::new(
        //             rng.gen_range(0.0..WIN_WIDTH as f32),
        //             rng.gen_range(0.0..WIN_HEIGHT as f32),
        //         ),
        //         &Vector2::new(
        //             rng.gen_range(0.0..WIN_WIDTH as f32),
        //             rng.gen_range(0.0..WIN_HEIGHT as f32),
        //         ),
        //         &colors[i % 4],
        //     );
        // }

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
