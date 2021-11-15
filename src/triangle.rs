use na::{Vector2, Vector3};

use crate::vertex::HasNormal;

use super::*;
pub struct Triangle {
    vertices: [Vertex; 3],
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
        let v1 = dbg!(vector_array[0]);
        let normal = v1.cross(dbg!(&vector_array[1]));
        normal
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

#[cfg(test)]
mod tests {
    use na::vector;

    use super::*;

    fn triangles() -> Vec<Triangle> {
        let m = ModelFile::open("head.obj");

        let verts = m.vertex_parse(500, 500);
        let triangles = m.face_parse(&verts);
        triangles
    }
    #[test]
    fn normal_test() {
        let triangles = triangles();
        Vector3::<f64>::new(6 as f64, 5 as f64, 4 as f64);
        let a = vector!(2.0, 3.0, 4.0);
        assert_eq!(triangles[300].normal(), a);
    }
}
