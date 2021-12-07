use super::scalar::{Resizer, Translator};
use super::ImageBuffer;
use super::Scalar;
use super::Vertex;
use super::{Color, Xy};
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

// pub trait DrawToolsB {
//     fn set(&mut self, point: Xy<H, W>, color: Color);
//     fn get(&self, point: Xy<H, W>) -> Color;
//     fn draw(&mut self, d: &dyn Drawable<H, W>);
//     fn draw_iter() {}
// }

#[derive(Default)]
pub struct DrawIter {
    index: usize,
    buffer: ImageBuffer,
}

impl Iterator for DrawIter {
    type Item = (Xy, Color);

    fn next(&mut self) -> Option<Self::Item> {
        return if self.index < self.len() {
            let color = self.buffer.data[self.index];
            self.index += 1;
            todo!("correct me");
            Some((Xy(0, 0), color))
        } else {
            None
        };
    }
}
impl ExactSizeIterator for DrawIter {}

struct DrawnPixelsIter(DrawIter);
impl Iterator for DrawnPixelsIter {
    type Item = (Xy, Color);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
impl DrawnPixelsIterator for DrawnPixelsIter {}

trait DrawnPixelsIterator: Iterator {}

trait IterateByDrawnPixels: DrawBuffer {
    fn iter_drawn_pixels(&self) -> DrawnPixelsIter {
        DrawnPixelsIter(self.draw_iter())
    }
}
pub trait Drawable {
    fn draw(&self, drawer: &mut dyn Canvas);
}

pub trait DrawBuffer: Canvas {
    //TODO figure out a way to just copy the entire source buffer to the destination
    fn draw_buffer(&self, destination: impl Canvas) {}

    fn draw_iter(&self) -> DrawIter;
}

// // TODO maybe an iter of fillable possibly?
