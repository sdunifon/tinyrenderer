pub mod color;
mod image_buffer;
mod traits;

use super::*;
pub use color::*;
pub use image_buffer::ImageBuffer;
pub use traits::{DrawTools, Drawable};

use core::fmt;
pub struct Image {
    pub height: u32,
    pub width: u32,
    pub buffer: ImageBuffer, //TODO privatize me // buffer : ImageBuffer<Rgb<u8>, Vec<Rgb<u8::Subpixel> >
}
#[derive(PartialEq, Default, Clone, Copy)]
pub struct Pt(pub usize, pub usize);
impl fmt::Display for Pt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pt({},{})", self.0, self.1)
    }
}

impl fmt::Debug for Pt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pt({},{})", self.0, self.1)
    }
}

//implement method on image that takes a normalized vertex and translattes to something else
struct Translator {
    height: usize,
    width: usize,
}
//vertex plus translator =
// impl From<&Vertex> for Pt {
impl Pt {
    pub(crate) fn pt_on_image(vertex: &Vertex, image: &Image) -> Self {
        let resized_vertex = *vertex * (image.height.max(image.width) / 2) as f64;
        let center_adjust_x: i32 = (image.width as i32) / 2;
        let center_adjust_y: i32 = (image.height as i32) / 2;
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

impl Image {
    pub fn new(height: u32, width: u32) -> Image {
        Image {
            height,
            width,
            buffer: ImageBuffer::new(height, width),
        }
    }
    // image_lib::ImageBuffer<image_lib::Rgb::<u8>, Container>
    pub fn render_to_buffer(&self) -> image_lib::ImageBuffer<image_lib::Rgb<u8>, Vec<u8>> {
        let mut image_buffer = image_lib::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let y = self.height - y;
            *pixel = image_lib::Rgb::<u8>(self.get(Pt(x as usize, y as usize)).to_color_ary())
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
    fn pt2i(&self, pt: Pt) -> u32 {
        // dbg!(pt.1 * W + pt.0)
        pt.1 as u32 * self.width + pt.0 as u32
        // Self::xy2i(pt.0, pt.1)
    }

    // pub fn draw( drawer:( img ) -> () ){
    //     drawer(self);
    // }
}

impl DrawTools for Image {
    #[inline]
    fn get(&self, pt: Pt) -> Color {
        self.buffer.data[self.pt2i(pt) as usize]
    }

    #[inline]
    fn set(&mut self, pt: Pt, p: Color) {
        // dbg!(pt.0, pt.1, pt);

        if pt.1 > self.height as usize {
            println!("debug me");
        }

        debug_assert!(
            pt.0 as u32 <= self.width,
            "x is grearter than width: ! pt.0: {} < W:{}",
            pt.0,
            self.width
        );
        debug_assert!(
            pt.1 as u32 <= self.height,
            "y is grearter than height ! pt.1: {} < H:{}",
            pt.1,
            self.height
        );
        // dbg!(pt);
        todo!();
        //put set on buffer to access it
        // self.buffer.data[self.pt2i(pt) as usize] = p;
    }

    fn draw(&mut self, d: &dyn Drawable) {
        d.draw(self as &mut dyn DrawTools);
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

        assert_eq!(img.get(Pt(250, 250)), Color { r: 0, g: 0, b: 0 });
        img.set(Pt(250, 250), Color { r: 0, g: 255, b: 0 });
        assert_eq!(img.get(Pt(250, 250)), Color { r: 0, g: 255, b: 0 });
        assert_eq!(img.get(Pt(251, 250)), Color { r: 0, g: 0, b: 0 });
        img.set(Pt(250, 250), Color { r: 0, g: 1, b: 0 });
        assert_eq!(img.get(Pt(250, 250)), Color { r: 0, g: 1, b: 0 });
    }
    // #[test]
    // fn index_conversion_test() {
    //     assert_eq!(Image::xy2i(68, 345), Image::pt2i(Pt(68, 345)));
    // }

    #[test]
    #[ignore]
    fn get_set_boundries_test() {
        // if we are able to get this test working we can remvoe all the + 1 to the image size for the
        // where boundry l and switch back from [u8; (H + 1) * (W + 1)]: Sized, to [u8,H * W]
        let image = Image::new(500, 500);
        assert_eq!(image.pt2i(Pt(500, 500)), 250000)
    }
}
