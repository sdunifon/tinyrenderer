use std::ops;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq)] //TODO remove copy
pub struct Vertex {
    // note could have aditional data like color
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct ViewportPixel {
    // note could have a1ditional data like color
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

pub type Vertices = Vec<Vertex>;
// pub trait HasVerticies<const N: usize>
pub trait HasVerticies {
    // fn veuprtices(&self) -> [Vertex; N];
    fn vertices(&self) -> [Vertex; 3];

    fn vectors(&self) -> [Vector3<f64>; 3] {
        self.vertices()
            .map(|v| Vector3::<f64>::new(v.x as f64, v.y as f64, v.z as f64))
    }
}

pub trait HasNormal: HasVerticies {
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
}

impl<const H: usize, const W: usize> Drawable<H, W> for Vertex
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    fn draw(&self, canvas: &mut Image<H, W>) {
        canvas.set(
            Pt(self.x as usize, self.y as usize),
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
            x: (self.x as f64 * rhs),
            y: (self.y as f64 * rhs),
            z: (self.z as f64 * rhs),
        }
    }
}
