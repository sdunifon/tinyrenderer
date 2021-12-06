use super::Scalar;
use crate::{Vertex, Xy};
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct Pt {
    pub x: i32,
    pub y: i32,
    vertex: Vertex,
    scalar: Scalar,
}

impl Pt {
    //TODO this should not be pub.. should only be able to create pts through a scalar to ensure that it has been scaled to the correct dimensions
    pub fn new(x: i32, y: i32, vertex: &Vertex, scalar: &Scalar) -> Pt {
        Pt {
            x,
            y,
            vertex: vertex.clone(),
            scalar: scalar.clone(),
        }
    }
}
impl From<Pt> for Xy {
    fn from(pixel: Pt) -> Self {
        Xy(pixel.x, pixel.y)
    }
}

// impl Deref for Px {
//     type Target = Xy;
//
//     fn deref(&self) -> &Self::Target {
//         let &ret = &Xy::from(self);
//         ret
//     }
// }
