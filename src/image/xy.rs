use crate::{Image, Vertex};
use std::fmt;

#[derive(PartialEq, Default, Clone, Copy)]
pub struct Xy(pub u32, pub u32);

impl fmt::Display for Xy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Xy({},{})", self.0, self.1)
    }
}

impl fmt::Debug for Xy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Xy({},{})", self.0, self.1)
    }
}

// impl From<&Vertex> for Xy {
impl Xy {
    pub(crate) fn pt_on_image(vertex: &Vertex, image: &Image) -> Self {
        let resized_vertex = *vertex * (image.height.max(image.width) / 2) as f64;
        let center_adjust_x: i32 = (image.width as i32) / 2;
        let center_adjust_y: i32 = (image.height as i32) / 2;
        Xy(
            (resized_vertex.x.round() as i32 + center_adjust_x)
                .try_into()
                .unwrap(),
            (resized_vertex.y.round() as i32 + center_adjust_y)
                .try_into()
                .unwrap(),
        )
    }
}
