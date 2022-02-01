use crate::image::scalar::{Resizer, Translator};
use crate::{Color, Drawable, Scalar, Vertex, Xy};

pub trait Canvas {
    fn set(&mut self, point: Xy, color: &Color); // TODO color shouldn't be a reverence
    fn get(&self, point: Xy) -> &Color;
    fn draw(&mut self, drawable: &dyn Drawable); // possibly re naem to rasterize
    fn height(&self) -> u32;
    fn width(&self) -> u32;
    fn scalar(&self) -> &Scalar; // should these be here
    fn scale(&self, vertex: &Vertex) -> Xy; //should these be in canvas or another trait?
}
//TODO a canvas could be liks some sort of fx layer also.. a middle step in between the drawables and canvas that applies effects like blur
//resizer and translater could also be "layers"
