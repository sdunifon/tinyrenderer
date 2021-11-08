use super::*;
pub struct Face {
    vertices: [Vertex; 3],
}

impl Face {
    pub fn new(v1: Vertex, v2: Vertex, v3: Vertex) -> Face {
        Face {
            vertices: [v1, v2, v3],
        }
    }
    pub fn lines(&self) -> [Line; 3] {
        [
            Line::from_vertices(&self.vertices[0], &self.vertices[1]),
            Line::from_vertices(&self.vertices[0], &self.vertices[2]),
            Line::from_vertices(&self.vertices[1], &self.vertices[2]),
        ]
    }
}

impl<const H: usize, const W: usize> Drawable<H, W> for Face
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    fn draw(&self, image: &mut Image<H, W>) {
        for line in self.lines() {
            image.draw(&line)
        }
    }

    fn draw2(&self, image: &mut Image<H, W>) {
        todo!()
    }
}
