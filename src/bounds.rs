use super::*;

#[derive(Default, Debug, PartialEq)]
pub struct BoundingBox {
    pub x_min: i32,
    pub y_min: i32,
    pub x_max: i32,
    pub y_max: i32,
}

impl BoundingBox {
    fn new(x_min: i32, y_min: i32, x_max: i32, y_max: i32) -> BoundingBox {
        debug_assert!(x_min <= x_max);
        debug_assert!(y_min <= y_max);
        BoundingBox {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }
}

pub trait Boundable: HasVerticies {
    fn bounding_box(&self) -> BoundingBox {
        let vertex_array = self.vertices();

        let mut x_min = vertex_array[0].x;
        let mut x_max = vertex_array[0].x;
        let mut y_min = vertex_array[0].y;
        let mut y_max = vertex_array[0].y;

        for vertex in &vertex_array[1..] {
            if vertex.x < x_min {
                x_min = vertex.x
            };
            if vertex.x > x_max {
                x_max = vertex.x
            };
            if vertex.y < y_min {
                y_min = vertex.y
            };
            if vertex.y > y_max {
                y_max = vertex.y
            };
        }
        BoundingBox {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounding_box_test() {
        let t = Triangle::new([
            Vertex { x: 0, y: 10, z: 30 },
            Vertex {
                x: 100,
                y: -50,
                z: 30,
            },
            Vertex { x: 0, y: 25, z: 30 },
        ]);
        assert_eq!(
            t.bounding_box(),
            BoundingBox {
                x_min: 0,
                x_max: 100,
                y_min: -50,
                y_max: 25
            }
        );
    }
}
