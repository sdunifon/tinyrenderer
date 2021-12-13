mod normalized_vertices;

use super::image::Canvas;
use super::*;
pub use normalized_vertices::NormalizedVertices;
use std::cmp::Ordering;
use std::ops;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)] //TODO remove copy
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Eq for Vertex {}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.partial_cmp(&other.y).expect("NAN")
    }
}

struct SortedVertices(Vertices);

impl From<Vertices> for SortedVertices {
    fn from(mut vertices: Vertices) -> Self {
        vertices.sort();
        Self(vertices)
    }
}

impl Deref for SortedVertices {
    type Target = Vertices;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Vertices = Vec<Vertex>; // abstract into own impl with all the needed vertice functions

trait ToSortedVertices {
    fn to_sorted_vertices(self) -> SortedVertices;
    // {
    //     self.sort();
    //     SortedVertices(self)
    // }
}

impl ToSortedVertices for Vertices {
    fn to_sorted_vertices(mut self) -> SortedVertices {
        self.sort();
        SortedVertices(self)
    }
}
pub trait HasTriangleVerticies {
    // fn veuprtices(&self) -> [Vertex; N];
    fn vertices(&self) -> [Vertex; 3];

    fn vectors(&self) -> [Vector3<f64>; 3] {
        self.vertices()
            .map(|v| Vector3::<f64>::new(v.x as f64, v.y as f64, v.z as f64))
    }
}

pub trait HasNormal: HasTriangleVerticies {
    fn normal(&self) -> Vector3<f64>;
}

impl Vertex {
    pub fn nw_resized(x: f64, y: f64, z: f64, height: f64, width: f64) -> Vertex {
        let avg_resize = (height + width) / 2.0;
        Self {
            x: ((x + 1.0) * (width / 2.0)).round() as f64,
            y: ((y + 1.0) * (height / 2.0)).round() as f64,
            z: ((z + 1.0) * (avg_resize / 2.0)).round() as f64, //not sure if that should be resized
        }
    }
    pub fn new(x: f64, y: f64, z: f64) -> Vertex {
        //TODO force new to enforce normalized coordinates
        debug_assert!(x <= 1., "x must be less than or equal to 1");
        debug_assert!(y <= 1., "y must be less than or equal to 1");
        debug_assert!(z <= 1., "z must be less than or equal to 1");
        Vertex { x, y, z }
    }
}

pub trait ToPoint {
    fn to_point(&self) -> Xy;
}

// impl ToPoint for Vertex {
// fn to_point(&self) -> Xy {
//     Xy::pt_on_image(self)
// }
// }

impl Drawable for Vertices {
    fn draw(&self, image: &mut dyn Canvas) {
        self.iter().for_each(|v| v.draw(image));
    }
}

impl Drawable for Vertex {
    fn draw(&self, canvas: &mut dyn Canvas) {
        canvas.set(
            //todo this should accept a pt to ensure the scaling instead of an xy
            canvas.scalar().scale_v(self).into(),
            &Color {
                r: 255,
                g: 255,
                b: 255,
            },
        )
        // canvas.set(self.into(), Color { r: 0, g: 0, b: 255 })
    }
}

impl ops::Add<Vertex> for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Vertex) -> Self::Output {
        Vertex {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ops::Sub<Vertex> for Vertex {
    type Output = Vertex;

    fn sub(self, rhs: Vertex) -> Self::Output {
        Vertex {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vertex {
    type Output = Vertex;

    fn mul(self, rhs: f64) -> Self::Output {
        Vertex {
            x: (self.x * rhs),
            y: (self.y * rhs),
            z: (self.z * rhs),
        }
    }
}

impl ops::Div<f64> for Vertex {
    type Output = Vertex;

    fn div(self, rhs: f64) -> Self::Output {
        Vertex {
            x: (self.x / rhs),
            y: (self.y / rhs),
            z: (self.z / rhs),
        }
    }
}
impl ops::Add<f64> for Vertex {
    type Output = Vertex;

    fn add(self, rhs: f64) -> Self::Output {
        Vertex {
            x: (self.x + rhs),
            y: (self.y + rhs),
            z: (self.z + rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn vertex_must_be_normalized_test() {
        Vertex {
            x: 5.0,
            y: 5.0,
            z: 5.0,
        };
    }
    #[test]
    fn valid_vertex_test() {
        let v = Vertex {
            x: 0.05,
            y: 1.0,
            z: -0.3,
        };
        assert_eq!(
            v,
            Vertex {
                x: 0.05,
                y: 1.0,
                z: -0.3
            }
        );
    }
}
