use super::Scalar;
use crate::{Vertex, Xy};
use std::ops::Deref;

struct Px {
    x: u32,
    y: u32,
    vertex: Vertex,
    scalar: Scalar,
}

impl From<&Px> for Xy {
    fn from(pixel: &Px) -> Self {
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
