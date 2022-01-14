use super::bounds::*;
use super::*;

// triangle(vec2 points[3]) {
//     vec2 bbox[2] = find_bounding_box(points);
//     for (each pixel in the bounding box) {
//         if (inside(points, pixel)) {
//             put_pixel(pixel);
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn front_facing_triangle() -> Triangle {
        Triangle::new([
            Vertex {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            Vertex {
                x: 5.,
                y: 10.,
                z: 0.,
            },
            Vertex {
                x: 10.,
                y: 0.,
                z: 0.,
            },
        ])
    }

    #[test]
    fn bounding_box_test() {
        let t = front_facing_triangle();

        assert_eq!(
            t.bounding_box(),
            BoundingBox {
                x_min: 0.,
                x_max: 10.,
                y_min: 0.,
                y_max: 10.
            }
        )
    }
}
