pub mod barycentric;
mod fillable;
use crate::vertex::HasNormal;

use super::*;

use vector::Vector3;
#[derive(Debug)]
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

pub type Triangles = Vec<Triangle>;

impl Drawable for Triangles {
    fn draw(&self, drawer: &mut dyn Canvas) {
        for triangle in self {
            triangle.draw(drawer)
        }
    }
}

impl Fillable for Triangles {
    // not sure if we want this
    fn fill(&self, drawer: &mut dyn Canvas) {
        for triangle in self {
            triangle.fill(drawer)
        }
    }
}

impl Colorful for Triangles {}
impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Triangle {
        Triangle { vertices }
    }
    fn sorted_triangle_vertices(&self) -> (Vertex, Vertex, Vertex) {
        let mut va = self.vertices();
        va.sort_by(|a, b| -> std::cmp::Ordering { a.y.partial_cmp(&b.y).unwrap() });

        (va[0], va[1], va[2])
    }
}

trait ToLines {
    fn lines(&self) -> [Line; 3];
}

impl ToLines for Triangle {
    fn lines(&self) -> [Line; 3] {
        [
            Line::from_vertices(&self.vertices[0], &self.vertices[1]),
            Line::from_vertices(&self.vertices[0], &self.vertices[2]),
            Line::from_vertices(&self.vertices[1], &self.vertices[2]),
        ]
    }
}

impl HasNormal for Triangle {
    fn normal(&self) -> Vector3<f64> {
        let [v1, v2] = self.side_vectors();
        v1.cross(&v2)
    }
}

impl Brightness for Triangle {
    fn brightness(&self) -> u8 {
        let camera_direction = Vector3::<f64>::new(0.0, 0.0, -1.0).unit();

        println!(
            "brightnewss:{:?}",
            self.normal().unit().dot(&camera_direction)
        );

        match self.normal().unit().dot(&camera_direction) {
            num if num > 0. && num <= 1. => (num * 255.0) as u8,
            num if num > 1. => 255,
            _ => 0,
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut dyn Canvas) {
        for line in self.lines() {
            image.draw(&line)
        }
    }

    // fn draw2(&self, image: &mut Image) {
    //     todo!()
    // }
}

impl HasTriangleVertices for Triangle {
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

        let color = Color {
            r: ((r as f64) * (brightness as f64 / 255.0)) as u8,
            g: ((g as f64) * (brightness as f64 / 255.0)) as u8,
            b: ((b as f64) * (brightness as f64 / 255.0)) as u8,
        };
        color
    }
}

// impl Fillable for Triangle {
// fn fill(&self, image: &mut dyn Canvas) {
//     // // sort the vertices, v0, t1, t2 lower−to−upper (bubblesort yay!)
//     // if v0.y>v1.y {std::swap(v0, t1)};
//     // if v0.y>v2.y {std::swap(v0, t2)};
//     // if v1.y>v2.y {std::swap(v1, t2)};
//     let vn = Self::sorted_triangle_vertices(self);
//     let (v0, v1, v2): (Vertex, Vertex, Vertex) = (vn.0, vn.1, vn.2);
//     let total_height: i32 = v2.y as i32 - v0.y as i32;

//     let color = self.color(); //TO FIX.. this is causing stack overflowj

//     let black = Color { r: 0, g: 0, b: 0 };
//     if color == black {
//         return;
//     }
//     //let color = random_color();

//     {
//         let mut y = v0.y as i32;

//         while y as f64 <= v1.y {
//             let segment_height = v1.y - v0.y + 1.;

//             if total_height == 0 {
//                 // some triangles are all on the same y.. not sure what to do here.. just returning for now
//                 println!("flat triangle {:?}", self.vertices());
//                 return;
//             }
//             assert!(total_height != 0, "total height can not be 0");
//             let alpha: f64 = (y as f64 - v0.y) as f64 / total_height as f64;
//             let beta: f64 = (y as f64 - v0.y) as f64 / segment_height as f64;

//             let a = v0 + (v2 - v0) * alpha;
//             let b = v0 + (v1 - v0) * beta;
//             //if a.x > b.x {
//             //    //double check this is working
//             //    mem::swap(&mut a, &mut b);
//             //}

//             // let (a, b) = if a.x > b.x { (b, a) } else { (a, b) };
//             swap_vars!(a.x > b.x, a, b);

//             {
//                 let mut j = a.x.round() as i32;
//                 let bx_int = b.x.round() as i32;
//                 while j <= bx_int {
//                     image.set(Xy(j - 1, y), &color);
//                     j += 1;
//                 }
//             }
//             y += 1;
//         }
//     }

//     // for (int y=v0.y; y<=v1.y; y++) {
//     //     int segment_height = v1.y-v0.y+1;
//     //     float alpha = (float)(y-v0.y)/total_height;
//     //     float beta  = (float)(y-v0.y)/segment_height; // be careful with divisions by zero
//     //     Vec2i A = v0 + (v2-v0)*alpha;
//     //     Vec2i B = v0 + (v1-v0)*beta;
//     //     if (A.x>B.x) std::swap(A, B);
//     //     for (int j=A.x; j<=B.x; j++) {
//     //         image.set(j, y, color); // attention, due to int casts v0.y+i != A.y
//     //     }
//     // }

//     {
//         let mut y = v1.y as i32;

//         while y as f64 <= v2.y {
//             let segment_height = v2.y - v1.y + 1.;

//             let alpha: f64 = (y as f64 - v0.y) as f64 / total_height as f64;
//             let beta: f64 = (y as f64 - v1.y) as f64 / segment_height as f64;

//             let a = v0 + (v2 - v0) * alpha;
//             let b = v1 + (v2 - v1) * beta;

//             swap_vars!(a.x > b.x, a, b);

//             {
//                 let mut j = a.x.round() as i32;
//                 let bx_int = b.x.round() as i32;
//                 while j <= bx_int {
//                     image.set(Xy(j, y), &color);
//                     j += 1;
//                 }
//             }
//             y += 1;
//         }
//     }
//     // for (int y=v1.y; y<=t2.y; y++) {
//     //     int segment_height =  v2.y-t1.y+1;
//     //     float alpha = (float)(y-v0.y)/total_height;
//     //     float beta  = (float)(y-v1.y)/segment_height; // be careful with divisions by zero
//     //     Vec2i A = v0 + (t2-t0)*alpha;
//     //     Vec2i B = v1 + (t2-t1)*beta;
//     //     if (A.x>B.x) std::swap(A, B);
//     //     for (int j=A.x; j<=B.x; j++) {
//     //         image.set(j, y, color); // attention, due to int casts v0.y+i != A.y
//     //     }
//     // }
// }
// }
#[cfg(test)]
#[allow(dead_code)] //todo turn back on once more code is established
#[allow(unused_variables)]
mod tests {
    use crate::test_helper::tests::assert_vector_eq;

    use super::*;
    use float_eq::assert_float_eq;
    // use pretty_assertions::{assert_eq, assert_ne};

    fn assert_fvector_eq(lhs: Vector3<f64>, rhs: Vector3<f64>) {
        // TODO stop using na vector library
        assert_float_eq!(lhs.x, rhs.x, abs <= 0.000_000_1);
        assert_float_eq!(lhs.y, rhs.y, abs <= 0.000_000_1);
        assert_float_eq!(lhs.z, rhs.z, abs <= 0.000_000_1);
    }

    fn triangles() -> Vec<Triangle> {
        let m = ModelFile::open_file("assets/head.obj").unwrap();

        let verts = m.vertex_parse();
        let triangles = m.face_parse(&verts);
        triangles
    }
    #[test]
    //TODO convert to unit vector
    fn triangle_vertex_test() {
        let t = Triangle {
            vertices: [
                Vertex::new(0.1, 0.2, 0.3),
                Vertex::new(0.4, 0.5, 0.6),
                Vertex::new(0.7, 0.8, 0.9),
            ],
        };
        assert_eq!(t.vertices[1].y, 0.5);
        assert_eq!(t.vertices[2].z, 0.9);
        assert_eq!(t.vertices[0].x, 0.1);
    }
    #[test]
    fn normal_test() {
        let t = Triangle {
            vertices: [
                Vertex::new(0.0, 0.0, 0.0),
                Vertex::new(0.0, 0.1, 0.0),
                Vertex::new(0.2, 0.0, 0.1),
            ],
        };
        assert_vector_eq(t.normal().unit(), Vector3::new(0.447, 0.00, -0.894)); // correct but not normalized
    }
    #[test]
    fn normal2_test() {
        let t = Triangle {
            vertices: [
                Vertex::new(0.1, 0.1, 0.0),
                Vertex::new(0.0, 0.1, 0.0),
                Vertex::new(0.1, 0.0, 0.0),
            ],
        };
        assert_fvector_eq(t.normal(), Vector3::new(0.00, 0.00, 0.010));
    }
    #[test]
    fn normal3_test() {
        let t = Triangle {
            vertices: [
                Vertex::new(0.1, 0.1, 0.0),
                Vertex::new(0.1, 0.0, 0.0),
                Vertex::new(0.0, 0.1, 0.0),
            ],
        };
        assert_fvector_eq(t.normal(), Vector3::new(0.00, 0.00, -0.010));
    }

    #[test]
    fn normal4_test() {
        let t = Triangle {
            vertices: [
                Vertex::new(0.1, 0.2, 0.3),
                Vertex::new(0.4, 0.5, 0.6),
                Vertex::new(-0.1, -0.2, 0.3),
            ],
        };
        assert_fvector_eq(t.normal(), Vector3::new(0.120, -0.060, -0.060));
    }
    #[test]
    fn normal_from_file_test() {
        todo!();
        let triangles = triangles();
        // Vector3::<f64>::new(6 as f64, 5 as f64, 4 as f64);
        let a = Vector3::new(92.0, -6.0, 130.0);
        // panic!("{:?}", triangles[300]);
        assert_fvector_eq(triangles[300].normal().unit(), a);
    }
    #[test]
    fn brightness_45_test() {
        todo!();
        let t = Triangle {
            vertices: [
                Vertex::new(0.0, 0.0, 0.0),
                Vertex::new(0.0, 0.1, 0.1), //tilted out 45 degrees
                Vertex::new(0.2, 0.0, 0.0),
            ],
        };
        assert_eq!(t.brightness(), 127);
    }

    #[test]
    fn brightness_45_in_test() {
        todo!();
        let t = Triangle {
            vertices: [
                Vertex::new(0., 0., 0.),
                Vertex::new(0.0, 0.10, -0.10), //tilted in 45 degrees
                Vertex::new(0.20, 0.0, 0.0),
            ],
        };
        assert_eq!(t.brightness(), 127);
    }

    #[test]
    fn brightness_0_test() {
        let t = Triangle {
            vertices: [
                Vertex::new(0.0, 0.0, 0.0),
                Vertex::new(0.10, -0.1, 0.10),
                Vertex::new(0.20, 0.0, 0.0),
            ],
        };
        assert_eq!(t.brightness(), 0);
    }

    #[test]
    fn brightness_full_test() {
        let t = Triangle {
            vertices: [
                Vertex::new(0.0, 0.0, 0.0),
                Vertex::new(0.0, 0.10, 0.0),
                Vertex::new(0.2, 0.0, 0.0), //tilted out 45 degrees
            ],
        };
        assert_eq!(t.brightness(), 255);
    }
}
