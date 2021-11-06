use super::image::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// pub type Vertices<const N: usize> = [Vertex; N];
pub type Vertices = Vec<Vertex>;

impl Vertex {}

impl<const H: usize, const W: usize> Drawable<H, W> for Vertex
where
    [u8; H * W]: Sized,
{
    fn draw(&self, canvas: &mut Image<H, W>) {
        canvas.set(
            Pt(self.x as usize, self.y as usize),
            Px { r: 0, g: 0, b: 255 },
        )
    }

    fn draw2(&self, canvas: &mut Image<H, W>) {
        todo!()
    }
}
