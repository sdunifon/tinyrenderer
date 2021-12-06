use super::Image;
use super::{Pt, Vertex, Xy};
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub enum Scalar {
    Scale { x: u32, y: u32 },
    None,
}

impl Scalar {
    pub fn scaled_pt(&self, vertex: &Vertex) -> Pt {
        match self {
            Scalar::Scale {
                x: height,
                y: width,
            } => {
                let resized_vertex = *vertex * (height.max(width) / 2) as f64;
                let center_adjust_x: i32 = (*width as i32) / 2;
                let center_adjust_y: i32 = (*height as i32) / 2;
                let x = (resized_vertex.x.round() as i32 + center_adjust_x)
                    .try_into()
                    .unwrap();
                let y = (resized_vertex.y.round() as i32 + center_adjust_y)
                    .try_into()
                    .unwrap();
                Pt::new(x, y, vertex, self)
            }
            Scalar::None => Pt::new(vertex.x as u32, vertex.y as u32, vertex, self),
        }
    }
}
