use super::image::Drawable;
use super::*;
use crate::model_file::ModelFileDrawer;
use std::path::Path;
use std::{error, fmt};

const RENDER_WIDTH: usize = 150;
const RENDER_HEIGHT: usize = 150;

pub struct Render {
    file: Option<ModelFile>,
    pub image: Image<RENDER_HEIGHT, RENDER_WIDTH>, //TODO privatize me
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
        self.file = Some(ModelFile::open_file(filepath));
        self.reload();
        Ok(())
    }

    pub fn load_from_string(&mut self, str: &str) {
        self.file = Some(ModelFile::from(str))
    }

    pub fn reload(&mut self) {
        self.file.as_mut().unwrap().load();
    }

    // fn draw(&self, drawer: &mut dyn Drawer<H, W>) {
    //     match self.file {
    //         None => {}
    //         Some(file) => self.model_file.draw(drawer),
    //     }
    // }
    pub fn file_data(&self) -> String {
        self.file.as_ref().unwrap().file_data.join("\n")
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
