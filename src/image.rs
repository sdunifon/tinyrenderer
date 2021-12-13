pub mod color;
mod image_buffer;
mod pt;
mod scalar;
mod traits;
mod xy;

use super::*;
pub use image_buffer::ImageBuffer;
use pt::Pt;
pub use scalar::Scalar;
pub use traits::{Canvas, Drawable};
pub use xy::Xy;

use crate::image::scalar::Translator;
use core::fmt;
use scalar::Resizer;

pub struct Image {
    pub height: u32,
    pub width: u32,
    pub buffer: ImageBuffer, //TODO privatize me // buffer : ImageBuffer<Rgb<u8>, Vec<Rgb<u8::Subpixel> >
    scalar: Scalar,
    resizer: Box<Resizer>,
    translator: Box<Translator>,
}

impl Image {
    pub fn new(height: u32, width: u32) -> Image {
        Image {
            height,
            width,
            scalar: Scalar::Scale {
                x: height,
                y: width,
            },
            resizer: Box::new(Resizer::new(height, width)),
            translator: Box::new(Translator::new(height, width)),
            buffer: ImageBuffer::new(height, width),
        }
    }
    // image_lib::ImageBuffer<image_lib::Rgb::<u8>, Container>

    //TODO move to image_buffer
    pub fn render_to_buffer(&self) -> image_lib::ImageBuffer<image_lib::Rgb<u8>, Vec<u8>> {
        let mut image_buffer = image_lib::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let y = self.height - y;
            *pixel = image_lib::Rgb::<u8>(self.buffer[&Xy((x as i32), (y as i32))].to_color_ary())
        }
        image_buffer
    }

    pub fn render(&self, filename: &str) {
        let image_buffer = self.render_to_buffer();
        image_buffer.save(filename).unwrap();
    }

    #[inline]
    fn xy2i(&self, x: u32, y: u32) -> u32 {
        y * self.width + x
    }

    #[inline]
    fn pt2i(&self, pt: Xy) -> u32 {
        // dbg!(pt.1 * W + pt.0)
        pt.1 as u32 * self.width + pt.0 as u32
        // Self::xy2i(pt.0, pt.1)
    }

    // pub fn draw( drawer:( img ) -> () ){
    //     drawer(self);
    // }
}

impl Canvas for Image {
    #[inline]
    fn get(&self, pt: Xy) -> &Color {
        &self.buffer[&pt]
    }

    #[inline]
    fn set(&mut self, pt: Xy, color: &Color) {
        self.buffer[&pt] = color.clone();
    }

    fn draw(&mut self, d: &dyn Drawable) {
        d.draw(self as &mut dyn Canvas);
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn width(&self) -> u32 {
        self.width
    }
    fn scalar(&self) -> &Scalar {
        &self.scalar
    }

    fn resizer(&self) -> &Resizer {
        &self.resizer
    }

    fn translator(&self) -> &Translator {
        &self.translator
    }

    fn scale(&self, vertex: &Vertex) -> Xy {
        self.scalar.scale_v(vertex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::assert_file_creation;

    #[test]
    fn render_test() {
        assert_file_creation("test_render.tga", |filename: &str| {
            let img = Image::new(500, 500);
            img.render(filename);
        });
    }

    #[test]
    fn xy2a_test() {
        assert_eq!(Image::new(500, 500).xy2i(25, 25), 12525)
    }

    #[test]
    fn set_get_test() {
        let mut img = Image::new(500, 500);

        assert_eq!(*img.get(Xy(250, 250)), Color { r: 0, g: 0, b: 0 });
        img.set(Xy(250, 250), &Color { r: 0, g: 255, b: 0 });
        assert_eq!(*img.get(Xy(250, 250)), Color { r: 0, g: 255, b: 0 });
        assert_eq!(*img.get(Xy(251, 250)), Color { r: 0, g: 0, b: 0 });
        img.set(Xy(250, 250), &Color { r: 0, g: 1, b: 0 });
        assert_eq!(*img.get(Xy(250, 250)), Color { r: 0, g: 1, b: 0 });
    }
    // #[test]
    // fn index_conversion_test() {
    //     assert_eq!(Image::xy2i(68, 345), Image::pt2i(Xy(68, 345)));
    // }

    #[test]
    #[ignore]
    fn get_set_boundries_test() {
        // if we are able to get this test working we can remvoe all the + 1 to the image size for the
        // where boundry l and switch back from [u8; (H + 1) * (W + 1)]: Sized, to [u8,H * W]
        let image = Image::new(500, 500);
        assert_eq!(image.pt2i(Xy(500, 500)), 250000)
    }
}
