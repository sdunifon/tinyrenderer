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

impl<const H: usize, const W: usize> Fillable<H, W> for Triangle
where
    [u8; (H + 1) * (W + 1)]: Sized + Drawable<H, W>,
{
    fn vertices() -> [Vertex; 3] {
        todo!()
    }
}
