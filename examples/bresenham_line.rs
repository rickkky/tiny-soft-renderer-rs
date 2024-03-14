use fltk::prelude::{GroupExt, WidgetBase, WidgetExt};
use nalgebra::Vector2;
use tinyrenderer::{basetype::Viewport, color::Color, renderer::Renderer};

const WIN_WIDTH: u32 = 1024;
const WIN_HEIGHT: u32 = 720;

pub fn main() {
    let app = fltk::app::App::default();
    let mut win = fltk::window::Window::new(100, 100, WIN_WIDTH as i32, WIN_HEIGHT as i32, "Test");

    let viewport = Viewport::new(0, 0, WIN_WIDTH, WIN_HEIGHT);
    let mut renderer = Renderer::new(viewport);

    win.draw(move |_| {
        renderer.clear();

        let colors = [Color::WHITE, Color::RED, Color::GREEN, Color::BLUE];
        for i in 0..16 {
            let angle = i as f32 * std::f32::consts::PI / 8.0;
            let x = angle.cos() * WIN_WIDTH as f32 / 2.0;
            let y = angle.sin() * WIN_HEIGHT as f32 / 2.0;
            renderer.draw_line(
                &Vector2::new(WIN_WIDTH as f32 / 2.0, WIN_HEIGHT as f32 / 2.0),
                &Vector2::new(x + WIN_WIDTH as f32 / 2.0, y + WIN_HEIGHT as f32 / 2.0),
                &colors[i % 4],
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
