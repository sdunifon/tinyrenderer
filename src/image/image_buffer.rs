use super::Color;
use super::Xy;
use std::ops::{Index, IndexMut};

//TODO maybe use bounding box here to also  specify the location.. then height and width can become methods
#[derive(Debug, Clone)]
pub struct ImageBuffer {
    height: u32,
    width: u32,
    pub(super) data: Vec<Color>,
}

impl Default for ImageBuffer {
    fn default() -> Self {
        Self {
            height: 500,
            width: 500,
            data: Vec::new(),
        }
    }
}
impl ImageBuffer {
    pub(crate) fn new(height: u32, width: u32) -> ImageBuffer {
        ImageBuffer {
            height,
            width,
            data: vec![Color::default(); (height * width) as usize],
        }
    }

    fn debug_bounds_check(&self, pt: &Xy) -> bool {
        let Xy(x, y) = pt;
        debug_assert!(
            *x < self.width as i32,
            "x is out of bounds: !(x:{} < width:{})",
            x,
            self.width
        );
        debug_assert!(
            *y < self.height as i32,
            "y is out of bounds: !(y:{} < height:{})",
            y,
            self.height
        );
        true
    }
    #[inline]
    fn x_y_to_index(&self, x: &i32, y: &i32) -> usize {
        (y * (self.width as i32) + x) as usize
    }
}

impl Index<&Xy> for ImageBuffer {
    type Output = Color;

    fn index(&self, index: &Xy) -> &Self::Output {
        //todo resize index to the relative position
        debug_assert!(self.debug_bounds_check(&index));
        let Xy(x, y) = index;

        &self.data[self.x_y_to_index(x, y)]
    }
}
impl<'a> IndexMut<&'a Xy> for ImageBuffer {
    fn index_mut(&mut self, index: &'a Xy) -> &mut Self::Output {
        debug_assert!(self.debug_bounds_check(&index));
        let Xy(x, y) = index;
        let index = self.x_y_to_index(x, y);
        &mut self.data[index]
    }
}

pub trait ToImageBuffer {
    fn to_image_buffer(self) -> ImageBuffer;
}
// impl DrawTool for ImageBuffer {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn image_buffer_index_test() {
        let buf = ImageBuffer::new(250, 250);
        buf[&Xy(0, 250)];
    }

    #[test]
    #[should_panic]
    fn image_buffer_index_overflow1_test() {
        let buf = ImageBuffer::new(250, 250);
        buf[&Xy(0, 251)];
    }

    #[test]
    #[should_panic]
    fn image_buffer_index_overflow2_test() {
        let buf = ImageBuffer::new(250, 250);
        buf[&Xy(251, 250)];
    }

    #[test]
    fn image_library_buffer_index_test() {
        let (width, height) = (100, 100);
        let mut image_buffer: image_lib::ImageBuffer<image_lib::Rgb<u8>, Vec<u8>> =
            image_lib::ImageBuffer::new(width, height);

        for (x, y, _pixel) in image_buffer.enumerate_pixels_mut() {
            assert!(y < height);
            assert!(x < width);
        }
        image_buffer[(width - 1, height - 1)];
    }

    #[test]
    #[should_panic]
    fn image_library_buffer_index2_test() {
        let (width, height) = (100, 100);
        let image_buffer: image_lib::ImageBuffer<image_lib::Rgb<u8>, Vec<u8>> =
            image_lib::ImageBuffer::new(width, height);
        image_buffer[(width, height)];
    }
}
