use super::Color;
use super::DrawTools;
use crate::Pt;
use std::ops::{Index, IndexMut};

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
    fn len(&self) -> u32 {
        self.height * self.width
    }

    fn debug_bounds_check(&self, pt: &Pt) -> bool {
        let Pt(x, y) = pt;
        debug_assert!(
            x < &self.width,
            "x is out of bounds: !(x:{} < self.width{})",
            x,
            self.width
        );
        debug_assert!(
            y < &self.height,
            "y is out of bounds: !(x:{} < self.width{})",
            y,
            self.height
        );
        true
    }
    #[inline]
    fn x_y_to_index(&self, x: &u32, y: &u32) -> usize {
        (y * self.width + x) as usize
    }
}

impl Index<&Pt> for ImageBuffer {
    type Output = Color;

    fn index(&self, index: &Pt) -> &Self::Output {
        debug_assert!(self.debug_bounds_check(&index));
        let Pt(x, y) = index;

        &self.data[self.x_y_to_index(x, y)]
    }
}
impl<'a> IndexMut<&'a Pt> for ImageBuffer {
    fn index_mut(&mut self, index: &'a Pt) -> &mut Self::Output {
        debug_assert!(self.debug_bounds_check(&index));
        let Pt(x, y) = index;
        let index = self.x_y_to_index(x, y);
        &mut self.data[index]
    }
}

// impl DrawTool for ImageBuffer {}
