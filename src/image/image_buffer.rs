use super::Color;
use super::DrawTools;

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
}
// impl DrawTool for ImageBuffer {}
