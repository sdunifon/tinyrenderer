use std::ops;

use super::image::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub type Vertices = Vec<Vertex>;

impl Vertex {
    pub fn new_resized(x: f64, y: f64, z: f64, height: usize, width: usize) -> Vertex {
        Self {
            x: ((x + 1.0) * (width as f64 / 2.0)) as u32,
            y: ((y + 1.0) * (height as f64 / 2.0)) as u32,
            z: z as u32,
        }
    }

    pub fn to_point(&self) -> Pt {
        Pt(self.x as usize, self.y as usize)
    }
}

impl<const H: usize, const W: usize> Drawable<H, W> for Vertex
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    fn draw(&self, canvas: &mut Image<H, W>) {
        canvas.set(
            Pt(self.x as usize, self.y as usize),
            Px { r: 0, g: 0, b: 255 },
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
            x: (self.x as f64 * rhs) as u32,
            y: (self.y as f64 * rhs) as u32,
            z: (self.z as f64 * rhs) as u32,
        }
    }
}
