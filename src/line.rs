use nalgebra::Vector2;

const INSIDE: u8 = /**/ 0b0000;
const LEFT: u8 = /****/ 0b0001;
const RIGHT: u8 = /***/ 0b0010;
const BOTTOM: u8 = /**/ 0b0100;
const TOP: u8 = /*****/ 0b1000;
fn compute_area_code(p: &Vector2<f32>, rect_min: &Vector2<f32>, rect_max: &Vector2<f32>) -> u8 {
    let horizontal_code = if p.x < rect_min.x {
        LEFT
    } else if p.x > rect_max.x {
        RIGHT
    } else {
        INSIDE
    };
    let vertical_code = if p.y < rect_min.y {
        BOTTOM
    } else if p.y > rect_max.y {
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
    rect_min: &Vector2<f32>,
    rect_max: &Vector2<f32>,
) -> Option<(Vector2<f32>, Vector2<f32>)> {
    let mut p_0 = p_0.clone();
    let mut p_1 = p_1.clone();
    let mut code_0 = compute_area_code(&p_0, &rect_min, &rect_max);
    let mut code_1 = compute_area_code(&p_1, &rect_min, &rect_max);

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
            p.y = rect_max.y;
            p.x = p_0.x + (p_1.x - p_0.x) * (p.y - p_0.y) / (p_1.y - p_0.y);
        } else if code & BOTTOM != INSIDE {
            p.y = rect_min.y;
            p.x = p_0.x + (p_1.x - p_0.x) * (p.y - p_0.y) / (p_1.y - p_0.y);
        } else if code & RIGHT != INSIDE {
            p.x = rect_max.x;
            p.y = p_0.y + (p_1.y - p_0.y) * (p.x - p_0.x) / (p_1.x - p_0.x);
        } else if code & LEFT != INSIDE {
            p.x = rect_min.x;
            p.y = p_0.y + (p_1.y - p_0.y) * (p.x - p_0.x) / (p_1.x - p_0.x);
        }

        // Replace the outside point with the intersection point.
        if code == code_0 {
            p_0.x = p.x;
            p_0.y = p.y;
            code_0 = compute_area_code(&p_0, &rect_min, &rect_max);
        } else {
            p_1.x = p.x;
            p_1.y = p.y;
            code_1 = compute_area_code(&p_1, &rect_min, &rect_max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_line() {
        let rect_min = Vector2::new(0., 0.);
        let rect_max = Vector2::new(1., 1.);
        let p_0 = Vector2::new(0.5, 0.5);
        let p_1 = Vector2::new(0.5, 1.5);
        let (p_0, p_1) = clip_line(&p_0, &p_1, &rect_min, &rect_max).unwrap();
        assert_eq!(p_0, Vector2::new(0.5, 0.5));
        assert_eq!(p_1, Vector2::new(0.5, 1.));
        let p_0 = Vector2::new(-1., -1.);
        let p_1 = Vector2::new(2., 2.);
        let (p_0, p_1) = clip_line(&p_0, &p_1, &rect_min, &rect_max).unwrap();
        assert_eq!(p_0, Vector2::new(0., 0.));
        assert_eq!(p_1, Vector2::new(1., 1.));
    }
}
