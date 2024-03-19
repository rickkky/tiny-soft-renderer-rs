use crate::basetype::Bbox2;
use nalgebra::Vector2;

const INSIDE: u8 = /**/ 0b0000;
const LEFT: u8 = /****/ 0b0001;
const RIGHT: u8 = /***/ 0b0010;
const BOTTOM: u8 = /**/ 0b0100;
const TOP: u8 = /*****/ 0b1000;

fn compute_area_code(p: &Vector2<f32>, bbox: &Bbox2<f32>) -> u8 {
    let horizontal_code = if p.x < bbox.l {
        LEFT
    } else if p.x > bbox.r {
        RIGHT
    } else {
        INSIDE
    };
    let vertical_code = if p.y < bbox.b {
        BOTTOM
    } else if p.y > bbox.t {
        TOP
    } else {
        INSIDE
    };
    horizontal_code | vertical_code
}

/**
 * Cohen-Sutherland line clipping algorithm.
 */
pub fn clip_line(
    p_0: &Vector2<f32>,
    p_1: &Vector2<f32>,
    bbox: &Bbox2<f32>,
) -> Option<(Vector2<f32>, Vector2<f32>)> {
    let mut p_0 = p_0.clone();
    let mut p_1 = p_1.clone();
    let mut code_0 = compute_area_code(&p_0, bbox);
    let mut code_1 = compute_area_code(&p_1, bbox);

    loop {
        // Both points are on the same outside region.
        if code_0 & code_1 != INSIDE {
            return None;

        // Both points are inside the region.
        } else if code_0 | code_1 == INSIDE {
            return Some((p_0, p_1));
        }

        // At least one point is outside the region.
        let code = if code_1 > code_0 { code_1 } else { code_0 };

        // Compute intersection point.
        let mut p = Vector2::<f32>::new(0., 0.);
        if code & TOP != INSIDE {
            p.y = bbox.t;
            p.x = p_0.x + (p_1.x - p_0.x) * (p.y - p_0.y) / (p_1.y - p_0.y);
        } else if code & BOTTOM != INSIDE {
            p.y = bbox.b;
            p.x = p_0.x + (p_1.x - p_0.x) * (p.y - p_0.y) / (p_1.y - p_0.y);
        } else if code & RIGHT != INSIDE {
            p.x = bbox.r;
            p.y = p_0.y + (p_1.y - p_0.y) * (p.x - p_0.x) / (p_1.x - p_0.x);
        } else if code & LEFT != INSIDE {
            p.x = bbox.l;
            p.y = p_0.y + (p_1.y - p_0.y) * (p.x - p_0.x) / (p_1.x - p_0.x);
        }

        // Replace the outside point with the intersection point.
        if code == code_0 {
            p_0.x = p.x;
            p_0.y = p.y;
            code_0 = compute_area_code(&p_0, bbox);
        } else {
            p_1.x = p.x;
            p_1.y = p.y;
            code_1 = compute_area_code(&p_1, bbox);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_line() {
        let bbox = Bbox2::new(0.0, 1.0, 0.0, 1.0);
        let p_0 = Vector2::new(0.5, 0.5);
        let p_1 = Vector2::new(0.5, 1.5);
        let (p_0, p_1) = clip_line(&p_0, &p_1, &bbox).unwrap();
        assert_eq!(p_0, Vector2::new(0.5, 0.5));
        assert_eq!(p_1, Vector2::new(0.5, 1.));
        let p_0 = Vector2::new(-1., -1.);
        let p_1 = Vector2::new(2., 2.);
        let (p_0, p_1) = clip_line(&p_0, &p_1, &bbox).unwrap();
        assert_eq!(p_0, Vector2::new(0., 0.));
        assert_eq!(p_1, Vector2::new(1., 1.));
    }
}

/**
 * Bresenham line drawing algorithm.
 */
pub fn travel_line_bresenham<T: FnMut(Vector2<i32>)>(
    p_0: &Vector2<f32>,
    p_1: &Vector2<f32>,
    mut action: T,
) {
    let (mut x_0, mut y_0) = (p_0.x.round() as i32, p_0.y.round() as i32);
    let (mut x_1, mut y_1) = (p_1.x.round() as i32, p_1.y.round() as i32);
    let steep = (y_1 - y_0).abs() > (x_1 - x_0).abs();
    if steep {
        // Swap x and y so that the slope abs is always less than 1.
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
     * Let step_y = delta_y / delta_x .
     * For each x += 1 , y += step_y .
     *
     * Because the line is continuous,
     * we define n to store the continuous change of y .
     * Let step_y = delta_y.abs() / delta_x .
     * For each x += 1 , n += step_y ;
     * if n > 0.5, y += 1 (or -1, depending on delta_y), n -= 1 .
     *
     * To avoid using floating point numbers and division method,
     * we multiply the original step_y by 2 * delta_x .
     *
     * Let incr_n = delta_y.abs() * 2 , decr_n = 2 * delta_x .
     * For each x += 1 , n += incr_n ;
     * if n > delta_x, y += 1 (or -1, depending on delta_y), n -= decr_n .
     *
     */
    let incr_n = 2 * delta_y.abs();
    let decr_n = 2 * delta_x;
    let mut n = -incr_n;
    let mut y = y_0;

    for x in x_0..=x_1 {
        n += incr_n;
        if n > delta_x {
            y += if delta_y > 0 { 1 } else { -1 };
            n -= decr_n;
        }
        let p = if steep {
            Vector2::new(y, x)
        } else {
            Vector2::new(x, y)
        };
        action(p);
    }
}
