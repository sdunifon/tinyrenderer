use std::cmp::{max, Ordering};
use std::ops;
use std::ops::{Deref, Range};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)] //TODO remove copy
pub struct Vertex {
    // note could have aditional data like color
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
#[derive(Debug, PartialEq)]
pub struct VertexRange {
    min: f64,
    max: f64,
}
impl VertexRange {
    pub fn magnitude(&self) -> f64 {
        if self.min.abs() < self.max.abs() {
            self.max.abs()
        } else {
            self.min.abs()
        }
    }
}
impl Default for VertexRange {
    fn default() -> Self {
        VertexRange {
            min: f64::MIN,
            max: f64::MAX,
        }
    }
}

pub struct ViewportPixel {
    // note could have a1ditional data like color
    pub x: u16,
    pub y: u16,
    pub z: u16,
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

struct NormalizedVertices {
    scale: f64,
    vertices: Vertices,
}

impl NormalizedVertices {
    fn calculate_vertex_range(vertices: &Vertices) -> VertexRange {
        debug_assert!(
            vertices.len() > 0,
            "Cannot calculate vertext range  because vertexes have not been parsed"
        );
        let mut range = VertexRange::default();

        for vertex in vertices {
            let Vertex { x, y, z } = vertex;
            for dimension in [x, y, z] {
                if dimension > &range.max {
                    range.min = *dimension;
                }
                if dimension < &range.min {
                    range.max = *dimension;
                }
            }
        }
        range
    }
    pub fn normalize_vertices(vertices: &Vertices, scale: f64) -> Vertices {
        let mut normalized_verticies = Vertices::new();
        for vertex in vertices {
            // let normalized_vertex = ((*vertex / scale) + 1.0) * 300.0;
            let normalized_vertex = Self::normalize(vertex, scale);

            normalized_verticies.push(normalized_vertex);
        }
        normalized_verticies
    }

    #[inline]
    fn normalize(v1: &Vertex, scale: f64) -> Vertex {
        let vertex = *v1 / scale;
        vertex
    }
}

impl From<Vertices> for NormalizedVertices {
    fn from(vertices: Vertices) -> NormalizedVertices {
        let vertex_range = NormalizedVertices::calculate_vertex_range(&vertices);
        let scale = vertex_range.magnitude();
        let normalized_vertices = NormalizedVertices::normalize_vertices(&vertices, scale);
        NormalizedVertices {
            vertices: normalized_vertices,
            scale: vertex_range.magnitude(),
        }
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
    pub fn new_resized(x: f64, y: f64, z: f64, height: f64, width: f64) -> Vertex {
        let avg_resize = (height + width) / 2.0;
        Self {
            x: ((x + 1.0) * (width / 2.0)).round() as f64,
            y: ((y + 1.0) * (height / 2.0)).round() as f64,
            z: ((z + 1.0) * (avg_resize / 2.0)).round() as f64, //not sure if that should be resized
        }
    }

    pub fn to_point(&self) -> Pt {
        Pt(self.x as usize, self.y as usize)
    }

    pub fn normalize(&self, scale: f64) -> Vertex {
        todo!()
    }
}
impl<const H: usize, const W: usize> Drawable<H, W> for Vertices
where
[u8; (H + 1) * (W + 1)]: Sized,
{
    fn draw(&self, image: &mut dyn Drawer<H,W>) {
      self.iter().for_each(|v| v.draw(image))
    }
}

impl<const H: usize, const W: usize> Drawable<H, W> for Vertex
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    fn draw(&self, canvas: &mut dyn Drawer<H,W>) {
        let resized_vertex = *self * (H.max(W)/2) as f64; //TODO move this to set maybe?
        let center_adjust_x: i32 = (W as i32)/2;
        let center_adjust_y: i32 = (H as i32)/2;
        let canavas_point = Pt((resized_vertex.x.round() as i32  + center_adjust_x).try_into().unwrap(), (resized_vertex.y.round() as i32 + center_adjust_y).try_into().unwrap());
        canvas.set(
            canavas_point,
            Color { r: 0, g: 0, b: 255 },
        )
    }

    // fn draw2(&self, _canvas: &mut Image<H, W>) {
    //     todo!()
    // }
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
    fn vertex_range_magnitude_test() {
        let vr = VertexRange { min: -3., max: 9. };
        assert_eq!(9., vr.magnitude());
    }
    #[test]
    fn normalize_verticies_test() {
        let verts: Vertices = vec![
            Vertex {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            Vertex {
                x: 4.0,
                y: -10.0,
                z: 10.0,
            },
        ];
        let norm_verts = NormalizedVertices::normalize_vertices(&verts, 10.);
        assert_eq!(
            norm_verts[0],
            Vertex {
                x: 0.1,
                y: 0.2,
                z: 0.3
            }
        );
        assert_eq!(
            norm_verts[1],
            Vertex {
                x: 0.4,
                y: -1.0,
                z: 1.0
            }
        );
    }
    #[test]
    fn normalize_verticies_test_2() {
        let verts: Vertices = vec![
            Vertex {
                x: 1.0,
                y: -4.0,
                z: 2.0,
            },
            Vertex {
                x: 1.0,
                y: 2.0,
                z: -3.0,
            },
        ];
        let norm_verts = NormalizedVertices::normalize_vertices(&verts, 4.);

        assert_eq!(
            norm_verts[0],
            Vertex {
                x: 0.25,
                y: -1.0,
                z: 0.5
            }
        );
        assert_eq!(
            norm_verts[1],
            Vertex {
                x: 0.25,
                y: 0.5,
                z: -0.75
            }
        );
    }
}
