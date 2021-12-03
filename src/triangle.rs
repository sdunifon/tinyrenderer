use na::Vector3;

use na::vector;

use crate::vertex::HasNormal;

use super::*;
pub struct Triangle {
    pub vertices: [Vertex; 3],
}

pub type Triangles = Vec<Triangle>;

impl Drawable for Triangles {
    fn draw(&self, drawer: &mut dyn DrawTools) {
        for triangle in self {
            triangle.draw(drawer)
        }
    }
}
impl Fillable for Triangles {
    fn fill(&self, drawer: &mut dyn DrawTools) {
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
        todo!();
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
impl Drawable for Triangle {
    fn draw(&self, image: &mut dyn DrawTools) {
        for line in self.lines() {
            image.draw(&line)
        }
    }

    // fn draw2(&self, image: &mut Image) {
    //     todo!()
    // }
}

impl HasTriangleVerticies for Triangle {
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

impl Fillable for Triangle {
    fn fill(&self, image: &mut dyn DrawTools) {
        // // sort the vertices, v0, t1, t2 lower−to−upper (bubblesort yay!)
        // if v0.y>v1.y {std::swap(v0, t1)};
        // if v0.y>v2.y {std::swap(v0, t2)};
        // if v1.y>v2.y {std::swap(v1, t2)};
        let vn = Self::sorted_triangle_vertices(self);
        let (v0, v1, v2): (Vertex, Vertex, Vertex) = (vn.0, vn.1, vn.2);
        let total_height: i32 = v2.y as i32 - v0.y as i32;

        let color = self.color(); //TO FIX.. this is causing stack overflowj

        let black = Color { r: 0, g: 0, b: 0 };
        if color == black {
            return;
        }
        //let color = random_color();

        {
            let mut y = v0.y as i32;

            while y as f64 <= v1.y {
                let segment_height = v1.y - v0.y + 1.;

                if total_height == 0 {
                    // some triangles are all on the same y.. not sure what to do here.. just returning for now
                    println!("flat triangle {:?}", self.vertices());
                    return;
                }
                assert!(total_height != 0, "total height can not be 0");
                let alpha: f64 = (y as f64 - v0.y) as f64 / total_height as f64;
                let beta: f64 = (y as f64 - v0.y) as f64 / segment_height as f64;

                let a = v0 + (v2 - v0) * alpha;
                let b = v0 + (v1 - v0) * beta;
                //if a.x > b.x {
                //    //double check this is working
                //    mem::swap(&mut a, &mut b);
                //}

                // let (a, b) = if a.x > b.x { (b, a) } else { (a, b) };
                swap_vars!(a.x > b.x, a, b);

                {
                    let mut j: u32 = a.x as u32; // TODO might need to round instead of truncxate here
                    while j <= b.x as u32 {
                        image.set(Pt(j as u32 - 1, y as u32), &color);
                        j += 1;
                    }
                }
                y += 1;
            }
        }

        // for (int y=v0.y; y<=v1.y; y++) {
        //     int segment_height = v1.y-v0.y+1;
        //     float alpha = (float)(y-v0.y)/total_height;
        //     float beta  = (float)(y-v0.y)/segment_height; // be careful with divisions by zero
        //     Vec2i A = v0 + (v2-v0)*alpha;
        //     Vec2i B = v0 + (v1-v0)*beta;
        //     if (A.x>B.x) std::swap(A, B);
        //     for (int j=A.x; j<=B.x; j++) {
        //         image.set(j, y, color); // attention, due to int casts v0.y+i != A.y
        //     }
        // }

        {
            let mut y = v1.y as i32;

            while y as f64 <= v2.y {
                let segment_height = v2.y - v1.y + 1.;

                let alpha: f64 = (y as f64 - v0.y) as f64 / total_height as f64;
                let beta: f64 = (y as f64 - v1.y) as f64 / segment_height as f64;

                let a = v0 + (v2 - v0) * alpha;
                let b = v1 + (v2 - v1) * beta;

                swap_vars!(a.x > b.x, a, b);

                {
                    let mut j: u32 = a.x as u32;
                    while j <= b.x as u32 {
                        image.set(Pt(j, y as u32), &color);
                        j += 1;
                    }
                }
                y += 1;
            }
        }

        // for (int y=v1.y; y<=t2.y; y++) {
        //     int segment_height =  v2.y-t1.y+1;
        //     float alpha = (float)(y-v0.y)/total_height;
        //     float beta  = (float)(y-v1.y)/segment_height; // be careful with divisions by zero
        //     Vec2i A = v0 + (t2-t0)*alpha;
        //     Vec2i B = v1 + (t2-t1)*beta;
        //     if (A.x>B.x) std::swap(A, B);
        //     for (int j=A.x; j<=B.x; j++) {
        //         image.set(j, y, color); // attention, due to int casts v0.y+i != A.y
        //     }
        // }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    // use pretty_assertions::{assert_eq, assert_ne};

    fn triangles() -> Vec<Triangle> {
        let m = ModelFile::open_file("assets/head.obj");

        let verts = m.vertex_parse();
        let triangles = m.face_parse(&verts);
        triangles
    }
    #[test]
    fn triangle_vertex_test() {
        let t = Triangle {
            vertices: [
                Vertex {
                    x: 1.,
                    y: 2.,
                    z: 3.,
                },
                Vertex {
                    x: 4.,
                    y: 5.,
                    z: 6.,
                },
                Vertex {
                    x: 7.,
                    y: 8.,
                    z: 9.,
                },
            ],
        };
        assert_eq!(t.vertices[1].y, 5.);
        assert_eq!(t.vertices[2].z, 9.);
        assert_eq!(t.vertices[0].x, 1.);
    }
    #[test]
    fn normal_test() {
        let t = Triangle {
            vertices: [
                Vertex {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Vertex {
                    x: 0.,
                    y: 1.,
                    z: 0.,
                },
                Vertex {
                    x: 2.,
                    y: 0.,
                    z: 1.,
                },
            ],
        };
        assert_eq!(t.normal(), vector!(1.0, 0.0, -2.0));
    }
    #[test]
    fn normal2_test() {
        let t = Triangle {
            vertices: [
                Vertex {
                    x: 1.,
                    y: 1.,
                    z: 0.,
                },
                Vertex {
                    x: 0.,
                    y: 1.,
                    z: 0.,
                },
                Vertex {
                    x: 1.,
                    y: 0.,
                    z: 0.,
                },
            ],
        };
        assert_eq!(t.normal(), vector!(0.0, 0.0, 1.0));
    }
    #[test]
    fn normal3_test() {
        let t = Triangle {
            vertices: [
                Vertex {
                    x: 1.,
                    y: 1.,
                    z: 0.,
                },
                Vertex {
                    x: 1.,
                    y: 0.,
                    z: 0.,
                },
                Vertex {
                    x: 0.,
                    y: 1.,
                    z: 0.,
                },
            ],
        };
        assert_eq!(t.normal(), vector!(0.0, 0.0, -1.0));
    }

    #[test]
    fn normal4_test() {
        let t = Triangle {
            vertices: [
                Vertex {
                    x: 1.,
                    y: 2.,
                    z: 3.,
                },
                Vertex {
                    x: 4.,
                    y: 5.,
                    z: 6.,
                },
                Vertex {
                    x: -1.,
                    y: -2.,
                    z: 3.,
                },
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
                Vertex {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Vertex {
                    x: 0.,
                    y: 1.,
                    z: 1.,
                }, //tilted out 45 degrees
                Vertex {
                    x: 2.,
                    y: 0.,
                    z: 0.,
                },
            ],
        };
        assert_eq!(t.brightness(), 127);
    }

    #[test]
    fn brightness_45_in_test() {
        let t = Triangle {
            vertices: [
                Vertex {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Vertex {
                    x: 0.,
                    y: 10.,
                    z: -10.,
                }, //tilted in 45 degrees
                Vertex {
                    x: 20.,
                    y: 0.,
                    z: 0.,
                },
            ],
        };
        assert_eq!(t.brightness(), 127);
    }

    #[test]
    fn brightness_0_test() {
        let t = Triangle {
            vertices: [
                Vertex {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Vertex {
                    x: 10.,
                    y: -1.,
                    z: 10.,
                },
                Vertex {
                    x: 20.,
                    y: 0.,
                    z: 0.,
                },
            ],
        };
        assert_eq!(t.brightness(), 0);
    }

    #[test]
    fn brightness_full_test() {
        let t = Triangle {
            vertices: [
                Vertex {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                Vertex {
                    x: 0.,
                    y: 10.,
                    z: 0.,
                },
                Vertex {
                    x: 2.,
                    y: 0.,
                    z: 0.,
                }, //tilted out 45 degrees
            ],
        };
        assert_eq!(t.brightness(), 1);
    }
}
