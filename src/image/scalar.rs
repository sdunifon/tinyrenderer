use super::Image;
use super::{Vertex, Xy};

#[derive(Debug, Clone, Copy)]
pub enum Scalar {
    Scale { x: u32, y: u32 },
    None,
}

impl Scalar {
    fn scale(vertex: &Vertex) -> Xy {
        Xy(9, 9)
    }
    fn scaled_pt(vertex: &Vertex, image: &Image) -> Xy {
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
