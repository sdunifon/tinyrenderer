use super::Scalar;
use crate::{Vertex, Xy};
use std::ops::Deref;

struct Pt {
    x: u32,
    y: u32,
    vertex: Vertex,
    scalar: Scalar,
}

impl From<&Pt> for Xy {
    fn from(pixel: &Pt) -> Self {
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
