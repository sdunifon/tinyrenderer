use na::Vector3;

use na::vector;

use crate::vertex::HasNormal;

use super::*;
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

pub type Triangles = Vec<Triangle>;

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Triangle {
        Triangle { vertices }
    }
    pub fn lines(&self) -> [Line; 3] {
        [
            Line::from_vertices(&self.vertices[0], &self.vertices[1]),
            Line::from_vertices(&self.vertices[0], &self.vertices[2]),
            Line::from_vertices(&self.vertices[1], &self.vertices[2]),
        ]
    }
}

impl HasNormal for Triangle {
    fn normal(&self) -> Vector3<f64> {
        let vector_array = self.vectors();
        let v1 = vector_array[1] - vector_array[0];
        let v2 = vector_array[2] - vector_array[0];
        let normal = v2.cross(&v1);
        normal
    }
}

impl Brightness for Triangle {
    fn brightness(&self) -> u8 {
        let normal = self.normal();
        let camera_direction = Vector3::<f64>::new(0.0, 0.0, 1.0);
        let offset_angle = math::angle_between_vectors(&normal, &camera_direction).to_degrees();
        if offset_angle >= 90.0 {
            ((((offset_angle - 90.0) / 90.0) * 256.0) - 1.0) as u8
        } else {
            0
        }
    }
}

// struct Vector {
//     x: f64,
//     y: f64,
//     z: f64_u
// }
impl<const H: usize, const W: usize> Drawable<H, W> for Triangle
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    fn draw(&self, image: &mut Image<H, W>) {
        for line in self.lines() {
            image.draw(&line)
        }
    }

    // fn draw2(&self, image: &mut Image<H, W>) {
    //     todo!()
    // }
}

impl<const H: usize, const W: usize> Fillable<H, W> for Triangle where [u8; (H + 1) * (W + 1)]: Sized
{}

impl HasVerticies for Triangle {
    fn vertices(&self) -> [Vertex; 3] {
        //TODO make vertex a borrow instead of copy
        self.vertices.map(|v| v)
    }
}

impl Boundable for Triangle {}

impl Colorful for Triangle {
    fn color(&self) -> Color {
        let brightness: u8 = self.brightness();
        let Color { r, g, b } = self.base_color();

        let c = Color {
            r: ((r as f64) * (brightness as f64 / 255.0)) as u8,
            g: ((g as f64) * (brightness as f64 / 255.0)) as u8,
            b: ((b as f64) * (brightness as f64 / 255.0)) as u8,
        };
        let v = 5;
        c
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // use pretty_assertions::{assert_eq, assert_ne};

    fn triangles() -> Vec<Triangle> {
        let m = ModelFile::open("head.obj");

        let verts = m.vertex_parse(500, 500);
        let triangles = m.face_parse(&verts);
        triangles
    }
    #[test]
    fn triangle_vertex_test() {
        let t = Triangle {
            vertices: [
                Vertex { x: 1, y: 2, z: 3 },
                Vertex { x: 4, y: 5, z: 6 },
                Vertex { x: 7, y: 8, z: 9 },
            ],
        };
        assert_eq!(t.vertices[1].y, 5);
        assert_eq!(t.vertices[2].z, 9);
        assert_eq!(t.vertices[0].x, 1);
    }
    #[test]
    fn normal_test() {
        let t = Triangle {
            vertices: [
                Vertex { x: 0, y: 0, z: 0 },
                Vertex { x: 0, y: 1, z: 0 },
                Vertex { x: 2, y: 0, z: 1 },
            ],
        };
        assert_eq!(t.normal(), vector!(1.0, 0.0, -2.0));
    }
    #[test]
    fn normal2_test() {
        let t = Triangle {
            vertices: [
                Vertex { x: 1, y: 1, z: 0 },
                Vertex { x: 0, y: 1, z: 0 },
                Vertex { x: 1, y: 0, z: 0 },
            ],
        };
        assert_eq!(t.normal(), vector!(0.0, 0.0, 1.0));
    }
    #[test]
    fn normal3_test() {
        let t = Triangle {
            vertices: [
                Vertex { x: 1, y: 1, z: 0 },
                Vertex { x: 1, y: 0, z: 0 },
                Vertex { x: 0, y: 1, z: 0 },
            ],
        };
        assert_eq!(t.normal(), vector!(0.0, 0.0, -1.0));
    }

    #[test]
    fn normal4_test() {
        let t = Triangle {
            vertices: [
                Vertex { x: 1, y: 2, z: 3 },
                Vertex { x: 4, y: 5, z: 6 },
                Vertex { x: -1, y: -2, z: 3 },
            ],
        };
        assert_eq!(t.normal(), vector!(12.0, -6.0, -6.0));
    }
    #[test]
    fn normal_from_file_test() {
        let triangles = triangles();
        // Vector3::<f64>::new(6 as f64, 5 as f64, 4 as f64);
        let a = vector!(92.0, -6.0, 130.0);
        assert_eq!(triangles[300].normal(), a);
    }
    #[test]
    fn brightness_45_test() {
        let t = Triangle {
            vertices: [
                Vertex { x: 0, y: 0, z: 0 },
                Vertex { x: 0, y: 1, z: 1 }, //tilted out 45 degrees
                Vertex { x: 2, y: 0, z: 0 },
            ],
        };
        assert_eq!(t.brightness(), 127);
    }

    #[test]
    fn brightness_45_in_test() {
        let t = Triangle {
            vertices: [
                Vertex { x: 0, y: 0, z: 0 },
                Vertex {
                    x: 0,
                    y: 10,
                    z: -10,
                }, //tilted in 45 degrees
                Vertex { x: 20, y: 0, z: 0 },
            ],
        };
        assert_eq!(t.brightness(), 127);
    }

    #[test]
    fn brightness_0_test() {
        let t = Triangle {
            vertices: [
                Vertex { x: 0, y: 0, z: 0 },
                Vertex {
                    x: 10,
                    y: -1,
                    z: 10,
                },
                Vertex { x: 20, y: 0, z: 0 },
            ],
        };
        assert_eq!(t.brightness(), 0);
    }

    fn brightness_full_test() {
        let t = Triangle {
            vertices: [
                Vertex { x: 0, y: 0, z: 0 },
                Vertex { x: 0, y: 10, z: 0 },
                Vertex { x: 2, y: 0, z: 0 }, //tilted out 45 degrees
            ],
        };
        assert_eq!(t.brightness(), 1);
    }
}
