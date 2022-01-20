use crate::{Color, Drawable, Scalar, Vertex, Xy};
use crate::image::scalar::{Resizer, Translator};

pub trait Canvas {
    fn set(&mut self, point: Xy, color: &Color);
    fn get(&self, point: Xy) -> &Color;
    fn draw(&mut self, d: &dyn Drawable);
    fn height(&self) -> u32;
    fn width(&self) -> u32;
    fn scalar(&self) -> &Scalar;
    fn scale(&self, vertex: &Vertex) -> Xy;
    fn resizer(&self) -> &Resizer;
    fn translator(&self) -> &Translator;
}
