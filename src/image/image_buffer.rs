use super::Color;
use super::DrawTools;

pub struct ImageBuffer {
    height: usize,
    width: usize,
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

// impl DrawTool for ImageBuffer {}
