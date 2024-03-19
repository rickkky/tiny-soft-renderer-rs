use nalgebra::Vector2;

pub fn travel_triangle_sweep_line<T: FnMut(Vector2<i32>)>(
    p_0: &Vector2<f32>,
    p_1: &Vector2<f32>,
    p_2: &Vector2<f32>,
    mut action: T,
) {
    // Sort points by y.
    let mut points = vec![p_0, p_1, p_2];
    points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    let (p_0, p_1, p_2) = (points[0], points[1], points[2]);

    let (x_0, y_0) = (p_0.x.round(), p_0.y.round());
    let (x_1, y_1) = (p_1.x.round(), p_1.y.round());
    let (x_2, y_2) = (p_2.x.round(), p_2.y.round());

    // If the triangle is degenerate, return.
    if y_0 == y_1 && y_1 == y_2 {
        return;
    }

    let delta_y_20 = y_2 - y_0;
    let delta_y_21 = y_2 - y_1;
    let delta_y_10 = y_1 - y_0;
    let delta_x_20 = x_2 - x_0;
    let delta_x_21 = x_2 - x_1;
    let delta_x_10 = x_1 - x_0;
    let k_20_inv = delta_x_20 / delta_y_20;

    for y in y_0 as i32..=y_2 as i32 {
        let mut x_l = x_0 + k_20_inv * (y as f32 - y_0);

        // Avoid division by zero.
        let second_part = y as f32 > y_1 || y_0 == y_1;
        let x_base = if second_part { x_1 } else { x_0 };
        let k_inv = if second_part {
            delta_x_21 / delta_y_21
        } else {
            delta_x_10 / delta_y_10
        };
        let delta_y = if second_part {
            y as f32 - y_1
        } else {
            y as f32 - y_0
        };
        let mut x_r = x_base + k_inv * delta_y;

        if x_l > x_r {
            std::mem::swap(&mut x_l, &mut x_r);
        }
        let x_l = x_l.round() as i32;
        let x_r = x_r.round() as i32;

        for x in x_l..=x_r {
            action(Vector2::new(x, y));
        }
    }
}
