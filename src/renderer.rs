use crate::basetype::Viewport;
use crate::color::Color;
use crate::line::clip_line;
use nalgebra::Vector2;

pub struct Renderer {
    pub viewport: Viewport,
    pub frame_buffer: Vec<u8>,
}
impl Renderer {
    pub fn new(viewport: Viewport) -> Self {
        let pixel_count = (viewport.width * viewport.height) as usize;
        Self {
            viewport,
            frame_buffer: vec![0; pixel_count * 4],
        }
    }

    pub fn clear(&mut self) {
        self.frame_buffer.fill(0);
    }

    pub fn draw_pixel(&mut self, p: &Vector2<i32>, color: &Color) {
        let (offset_x, offset_y) = (self.viewport.offset_x, self.viewport.offset_y);
        let (width, height) = (self.viewport.width as i32, self.viewport.height as i32);
        let (x, y) = (p.x, p.y);
        if x < offset_x || x >= offset_x + width || y < offset_y || y >= offset_y + height {
            return;
        }
        let x = x - offset_x;
        // Flip the y coordinate.
        let y = height - 1 - (y - offset_y);
        let index = (x + y * width) as usize;
        self.frame_buffer[index * 4] = (color.r * 255.0) as u8;
        self.frame_buffer[index * 4 + 1] = (color.g * 255.0) as u8;
        self.frame_buffer[index * 4 + 2] = (color.b * 255.0) as u8;
        self.frame_buffer[index * 4 + 3] = (color.a * 255.0) as u8;
    }

    /**
     * Bresenham's line drawing algorithm.
     */
    pub fn draw_line(&mut self, p_0: &Vector2<f32>, p_1: &Vector2<f32>, color: &Color) {
        let clip_result = clip_line(
            p_0,
            p_1,
            &Vector2::<f32>::zeros(),
            &Vector2::new(self.viewport.width as f32, self.viewport.height as f32),
        );
        if clip_result.is_none() {
            return;
        }
        let (p_0, p_1) = clip_result.unwrap();

        let (mut x_0, mut y_0) = (p_0.x as i32, p_0.y as i32);
        let (mut x_1, mut y_1) = (p_1.x as i32, p_1.y as i32);
        let steep = (y_1 - y_0).abs() > (x_1 - x_0).abs();
        if steep {
            // Swap x and y so that the slope is always less than 1.
            std::mem::swap(&mut x_0, &mut y_0);
            std::mem::swap(&mut x_1, &mut y_1);
        }
        if x_0 > x_1 {
            // Swap p_0 and p_1 so that the traversal is from left to right.
            std::mem::swap(&mut x_0, &mut x_1);
            std::mem::swap(&mut y_0, &mut y_1);
        }

        let delta_x = x_1 - x_0;
        let delta_y = y_1 - y_0;
        /*
         * Let step_y = delta_y.abs() / delta_x .
         * For each x in [x_0, x_1], y += step_y .
         *
         * Because the line is continuous,
         * we define n to store the continuous change of y .
         * For each x , n += step_y ;
         * if n > 0.5, y += 1 (or -1 if delta_y < 0), n -= 1 .
         *
         * To avoid using floating point numbers and division method,
         * we multiply the original step_y by 2 * delta_x .
         *
         * Let incr_n = delta_y.abs() * 2 , decr_n = 2 * delta_x .
         * For each x , n += incr_n ;
         * if n > delta_x, y += 1 (or -1 if delta_y < 0), n -= decr_n .
         *
         */
        let incr_n = 2 * delta_y.abs();
        let decr_n = 2 * delta_x;
        let mut n = 0;
        let mut y = y_0;

        for x in x_0..=x_1 {
            if steep {
                self.draw_pixel(&Vector2::new(y, x), color);
            } else {
                self.draw_pixel(&Vector2::new(x, y), color);
            }

            n += incr_n;
            if n > delta_x {
                y += if delta_y > 0 { 1 } else { -1 };
                n -= decr_n;
            }
        }
    }
}
