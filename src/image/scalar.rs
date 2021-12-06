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
            Scalar::None => Pt::new(vertex.x as i32, vertex.y as i32, vertex, self),
        }
    }
}

pub struct Resizer(Box<dyn Fn(Vertex) -> Pt>);

impl Resizer {
    pub fn new(height: u32, width: u32) -> Resizer {
        let (height, width) = (400, 400);
        let func = |vertex: Vertex| -> Pt {
            let resized_vertex = vertex * (height.max(width) / 2) as f64;
            let x = resized_vertex.x.round() as i32;
            let y = resized_vertex.y.round() as i32;
            Pt::new(
                x,
                y,
                &vertex,
                &Scalar::Scale {
                    x: x as u32,
                    y: y as u32,
                },
            )
        };
        Resizer(Box::new(func))
    }
}
pub struct Translator(Box<dyn Fn(Pt) -> Pt>);
impl Translator {
    pub fn new(height: u32, width: u32) -> Translator {
        let translator = |pt: Pt| {
            pt.x += (width / 2) as i32;
            pt.y += (height / 2) as i32;
            pt
        };
        Translator(Box::new(translator))
    }
}
