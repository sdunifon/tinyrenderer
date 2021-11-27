use super::image::Drawable;
use super::*;
use crate::model_file::ModelFileDrawer;
use std::path::Path;
use std::{error, fmt};

const RENDER_WIDTH: usize = 400;
const RENDER_HEIGHT: usize = 400;

pub struct Render {
    file: Option<ModelFile>,
    image: Image<RENDER_HEIGHT, RENDER_WIDTH>,
    options: RenderOptions,
}
pub struct RenderOptions {
    pub wireframe: bool,
}
type Result<T> = std::result::Result<T, RenderError>;

impl Default for Render {
    fn default() -> Self {
        Self {
            file: Default::default(),
            image: Image::<RENDER_HEIGHT, RENDER_WIDTH>::new(),
            options: RenderOptions { wireframe: true },
        }
    }
}
impl Render {
    // pub fn new(filepath: &str) -> Self {
    //     assert!(Path::new(filepath).exists(), "{} doesn't exist!", filepath);
    //     Render {
    //         filename: filepath.to_string(),
    //         image: Image::<RENDER_HEIGHT, RENDER_WIDTH>::new(),
    //     }
    // }

    pub fn load_file(&mut self, filepath: &str) -> Result<()> {
        self.file = Some(ModelFile::open(filepath));
        self.reload();
        Ok(())
    }

    pub fn reload(&mut self) {
        self.file.as_mut().unwrap().load();
    }

    pub fn update(&mut self) -> Result<()> {
        let model_file_drawer = ModelFileDrawer {
            options: &RenderOptions { wireframe: true },
            model_file: self.file.as_ref().unwrap(),
        };
        model_file_drawer.draw(&mut self.image);
        Ok(())
    }

    pub fn image_buffer(&self) -> image_lib::ImageBuffer<image_lib::Rgb<u8>, Vec<u8>> {
        self.image.render_to_buffer()
    }
    pub fn width(&self) -> usize {
        self.image.width
    }
    pub fn height(&self) -> usize {
        self.image.height
    }
}

#[derive(Debug, Clone, Default)]
pub struct RenderError(pub String);
impl error::Error for RenderError {}
impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Render failed: {:?}", self.0)
    }
}
