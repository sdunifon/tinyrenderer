use super::ImageBuffer;
use super::{Color, Pt};
pub trait DrawTools<const H: usize, const W: usize>
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    fn set(&mut self, point: Pt<H, W>, color: Color);
    fn get(&self, point: Pt<H, W>) -> Color;
    fn draw(&mut self, d: &dyn Drawable<H, W>);
    fn draw_iter(&self) {}
}

// pub trait DrawToolsB {
//     fn set(&mut self, point: Pt<H, W>, color: Color);
//     fn get(&self, point: Pt<H, W>) -> Color;
//     fn draw(&mut self, d: &dyn Drawable<H, W>);
//     fn draw_iter() {}
// }

#[derive(Default)]
struct DrawIter {
    index: usize,
    buffer: ImageBuffer,
}

impl Iterator for DrawIter {
    // type Item = Pt<200, 200>;

    type Item = Color;
    fn next(&mut self) -> Option<Self::Item> {
        return if self.index < self.len() {
            let item = self.buffer.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        };
    }
}
impl ExactSizeIterator for DrawIter {}

struct DrawnPixelsIter(DrawIter);
impl Iterator for DrawnPixelsIter {
    // type Item = Pt<200, 200>;
    type Item = Color;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
impl DrawnPixelsIterator for DrawnPixelsIter {}

trait DrawnPixelsIterator: Iterator {}

// trait IterateByDrawnPixels: DrawBuffer {
//     fn iter_drawn_pixels(&self) -> DrawnPixelsIter {
//         DrawnPixelsIter(self.draw_iter())
//     }
// }
pub trait Drawable<const H: usize, const W: usize>
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    fn draw(&self, drawer: &mut dyn DrawTools<H, W>);
    // fn draw2(&self, image: &mut Image<H, W>);
}
//
// pub trait DrawBuffer: DrawToolsB {
//     //TODO figure out a way to just copy the entire source buffer to the destination
//     fn draw_buffer(&self, destination: impl DrawTools) {}
//
//     fn draw_iter(&self) {}
// }
//
// // TODO maybe an iter of fillable possibly?
