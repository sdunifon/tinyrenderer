pub mod color;
mod image_buffer;
mod traits;

use super::*;
pub use color::*;
pub use image_buffer::ImageBuffer;
pub use traits::{DrawTools, Drawable};

use core::fmt;
pub struct Image<const H: usize, const W: usize>
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    pub height: usize,
    pub width: usize,
    pub data: [Color; (H + 1) * (W + 1)], //TODO privatize me // buffer : ImageBuffer<Rgb<u8>, Vec<Rgb<u8::Subpixel> >
}
#[derive(PartialEq, Default, Clone, Copy)]
pub struct Pt<const H: usize, const W: usize>(pub usize, pub usize);
impl<const H: usize, const W: usize> fmt::Display for Pt<H, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pt<{},{}>({},{})", H, W, self.0, self.1)
    }
}

impl<const H: usize, const W: usize> fmt::Debug for Pt<H, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pt<{},{}>({},{})", H, W, self.0, self.1)
    }
}

impl<const H: usize, const W: usize> From<&Vertex> for Pt<H, W> {
    fn from(vertex: &Vertex) -> Self {
        let resized_vertex = *vertex * (H.max(W) / 2) as f64;
        let center_adjust_x: i32 = (W as i32) / 2;
        let center_adjust_y: i32 = (H as i32) / 2;
        Pt(
            (resized_vertex.x.round() as i32 + center_adjust_x)
                .try_into()
                .unwrap(),
            (resized_vertex.y.round() as i32 + center_adjust_y)
                .try_into()
                .unwrap(),
        )
    }
}

#[derive(Debug)]
pub struct PointOutOfBoundsError<const H: usize, const W: usize>(Pt<H, W>, usize, usize, usize);
impl<const H: usize, const W: usize> fmt::Display for PointOutOfBoundsError<H, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Point: {}  -> index[{}] is outside the image bounds of {}x{}:  ",
            self.0, self.1, self.2, self.3
        )
    }
}

impl<const H: usize, const W: usize> Image<H, W>
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    pub fn new() -> Image<H, W> {
        Image {
            height: H,
            width: W,
            data: [Color::default(); (H + 1) * (W + 1)],
        }
    }
    // image_lib::ImageBuffer<image_lib::Rgb::<u8>, Container>
    pub fn render_to_buffer(&self) -> image_lib::ImageBuffer<image_lib::Rgb<u8>, Vec<u8>> {
        let mut image_buffer = image_lib::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let y = H - y as usize;
            *pixel = image_lib::Rgb::<u8>(self.get(Pt(x as usize, y as usize)).to_color_ary())
        }
        image_buffer
    }

    pub fn render(&self, filename: &str) {
        let image_buffer = self.render_to_buffer();
        image_buffer.save(filename).unwrap();
    }

    #[inline]
    fn xy2i(x: usize, y: usize) -> usize {
        y * W + x
    }

    #[inline]
    fn pt2i(pt: Pt<H, W>) -> usize {
        // dbg!(pt.1 * W + pt.0)
        pt.1 * W + pt.0
        // Self::xy2i(pt.0, pt.1)
    }

    pub fn point_in_bounds(&self, pt: Pt<H, W>) -> Result<Pt<H, W>, PointOutOfBoundsError<H, W>> {
        // if pt.0 > W || pt.1 > H || Self::pt2i(pt) > self.data.len() {
        if pt.0 > W || pt.1 > H {
            return Err(PointOutOfBoundsError(pt, Self::pt2i(pt), H, W));
        }
        Ok(pt)
    }
    // pub fn draw( drawer:( img ) -> () ){
    //     drawer(self);
    // }
}

impl<const H: usize, const W: usize> DrawTools<H, W> for Image<H, W>
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    #[inline]
    fn get(&self, pt: Pt<H, W>) -> Color {
        self.data[Self::pt2i(pt)]
    }

    #[inline]
    fn set(&mut self, pt: Pt<H, W>, p: Color) {
        // dbg!(pt.0, pt.1, pt);

        if pt.1 > H {
            println!("debug me");
        }

        debug_assert!(
            pt.0 <= W,
            "x is grearter than width: ! pt.0: {} < W:{}",
            pt.0,
            W
        );
        debug_assert!(
            pt.1 <= H,
            "y is grearter than height ! pt.1: {} < H:{}",
            pt.1,
            H
        );
        // dbg!(pt);
        self.data[Self::pt2i(pt)] = p;
    }

    fn draw(&mut self, d: &dyn Drawable<H, W>) {
        d.draw(self as &mut dyn DrawTools<H, W>);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::assert_file_creation;

    #[test]
    fn render_test() {
        assert_file_creation("test_render.tga", |filename: &str| {
            let img = Image::<500, 500>::new();
            img.render(filename);
        });
    }

    #[test]
    fn xy2a_test() {
        assert_eq!(Image::<500, 500>::xy2i(25, 25), 12525)
    }

    #[test]
    fn set_get_test() {
        let mut img = Image::<500, 500>::new();

        assert_eq!(img.get(Pt(250, 250)), Color { r: 0, g: 0, b: 0 });
        img.set(Pt(250, 250), Color { r: 0, g: 255, b: 0 });
        assert_eq!(img.get(Pt(250, 250)), Color { r: 0, g: 255, b: 0 });
        assert_eq!(img.get(Pt(251, 250)), Color { r: 0, g: 0, b: 0 });
        img.set(Pt(250, 250), Color { r: 0, g: 1, b: 0 });
        assert_eq!(img.get(Pt(250, 250)), Color { r: 0, g: 1, b: 0 });
    }
    #[test]
    fn index_conversion_test() {
        assert_eq!(
            Image::<500, 500>::xy2i(68, 345),
            Image::<500, 500>::pt2i(Pt(68, 345))
        );
    }

    #[test]
    #[ignore]
    fn get_set_boundries_test() {
        // if we are able to get this test working we can remvoe all the + 1 to the image size for the
        // where boundry l and switch back from [u8; (H + 1) * (W + 1)]: Sized, to [u8,H * W]
        assert_eq!(Image::<500, 500>::pt2i(Pt(500, 500)), 250000)
    }
}
