pub mod color;
mod image_buffer;
mod pt;
pub(crate) mod scalar;
pub mod traits;
mod xy;

use super::*;
pub use crate::canvas::Canvas;
use color::*;
pub use image_buffer::ImageBuffer;
use pt::Pt;
pub use scalar::Scalar;
pub use xy::Xy;

pub use crate::drawable::Drawable;
use crate::image::scalar::Translator;
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
    //TODO use custom buffer instead of image packages
    pub fn render_to_buffer(&self) -> image_lib::ImageBuffer<image_lib::Rgb<u8>, Vec<u8>> {
        let mut image_buffer = image_lib::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let y = self.height - y - 1;
            *pixel = image_lib::Rgb::<u8>(self.buffer[&Xy(x as i32, y as i32)].to_color_ary())
        }
        image_buffer
    }

    pub fn render_to_file(&self, filename: &str) {
        let image_buffer = self.render_to_buffer();
        image_buffer.save(filename).unwrap();
    }

    fn resizer(&self) -> &Resizer {
        &self.resizer
    }

    fn translator(&self) -> &Translator {
        &self.translator
    }
}

impl Canvas for Image {
    #[inline]
    fn get(&self, pt: Xy) -> &Color {
        debug_assert!(
            pt.0 < self.width as i32 && pt.0 >= 0 as i32,
            "image: image.get x is out of bounds pt.0:{} self.width:{}",
            pt.0,
            self.width
        );
        debug_assert!(
            pt.1 < self.height as i32 && pt.1 >= 0 as i32,
            "image: image.get y is out of bounds pt.0:{} self.height:{}",
            pt.1,
            self.height
        );

        &self.buffer[&pt]
    }

    #[inline]
    fn set(&mut self, pt: Xy, color: &Color) {
        debug_assert!(
            pt.0 < self.width as i32 && pt.0 >= 0 as i32,
            "image: image.set x is out of bounds pt.0:{} self.width:{}",
            pt.0,
            self.width
        );
        debug_assert!(
            pt.1 < self.height as i32 && pt.1 >= 0 as i32,
            "image: image.set y is out of bounds pt.0:{} self.height:{}",
            pt.1,
            self.height
        );

        self.buffer[&pt] = color.clone();
    }

    fn draw(&mut self, drawable: &dyn Drawable) {
        drawable.draw_on(self as &mut dyn Canvas);
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

    ///Scale a normalized vertex to fit into the height and widht of the image
    fn scale(&self, vertex: &Vertex) -> Xy {
        self.scalar.scale_v(vertex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::tests::assert_file_creation;
    use assert_panic::assert_panic;

    #[test]
    fn render_test() {
        assert_file_creation("test_render.tga", |filename: &str| {
            let img = Image::new(500, 500);
            img.render_to_file(filename);
        });
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

    #[test]
    fn set_out_of_bounds_test() {
        assert_panic!(Image::new(100, 100).set(Xy(100, 0), &WHITE), String, starts with "image: image.set");
        assert_panic!(Image::new(100, 100).set(Xy(0, 100), &WHITE), String, starts with "image: image.set");
        assert_panic!(Image::new(100, 100).set(Xy(0, 101), &WHITE), String, starts with "image: image.set");
        assert_panic!(Image::new(100, 100).set(Xy(100, 100), &WHITE), String, starts with "image: image.set");
        assert_panic!(Image::new(500, 500).set(Xy(0, 500), &WHITE), String, starts with "image: image.set");
        assert_panic!(Image::new(500, 500).set(Xy(500, 0), &WHITE), String, starts with "image: image.set");
        assert_panic!(Image::new(500, 500).set(Xy(-1, 250), &WHITE), String, starts with "image: image.set");
        Image::new(500, 500).set(Xy(499, 499), &WHITE);
        Image::new(500, 500).set(Xy(0, 0), &WHITE);
    }
    #[test]
    fn get_out_of_bounds_test() {
        assert_panic!(let _i = Image::new(100, 100).get(Xy(100, 0)), String, starts with "image: image.get");
        assert_panic!(let _i = Image::new(100, 100).get(Xy(0, 100)), String, starts with "image: image.get");
        assert_panic!(let _i = Image::new(100, 100).get(Xy(0, 101)), String, starts with "image: image.get");
        assert_panic!(let _i = Image::new(100, 100).get(Xy(100, 100)), String, starts with "image: image.get");
        assert_panic!(let _i = Image::new(500, 500).get(Xy(0,500)), String, starts with "image: image.get");
        assert_panic!(let _i = Image::new(500, 500).get(Xy(500, 0)), String, starts with "image: image.get");
        assert_panic!(let _i = Image::new(500, 500).get(Xy(-1, 250)), String, starts with "image: image.get");
        Image::new(500, 500).get(Xy(499, 499));
        Image::new(500, 500).get(Xy(0, 0));
    }
    #[test]
    fn set_out_of_bounds_test2() {
        assert_panic!(Image::new(100, 100).set(Xy(100, 0), &WHITE), String, starts with "image:");
    }
}
