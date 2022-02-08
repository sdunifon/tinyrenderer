use super::*;
use crate::drawable::Drawable;
use crate::model_file::ModelFileDrawer;
use std::{error, fmt};

impl Default for RenderOptions {
    fn default() -> Self {
        RenderOptions {
            wireframe: true,
            height: 800,
            width: 800,
        }
    }
}

pub struct Render {
    file: Option<ModelFile>,
    pub render_queue: Vec<Box<dyn Drawable>>,
    pub image: Image, //TODO privatize me
    options: RenderOptions,
}
#[derive(Debug)]
pub struct RenderOptions {
    pub wireframe: bool,
    pub height: u16,
    pub width: u16,
}

type Result<T> = std::result::Result<T, RenderError>;

impl Default for Render {
    fn default() -> Self {
        let render_options = RenderOptions::default();
        Self {
            file: Default::default(),
            image: Image::new(render_options.height.into(), render_options.width.into()),
            options: RenderOptions::default(),
            render_queue: Vec::new(),
        }
    }
}
impl Render {
    //TODO add file name
    pub fn new(render_options: RenderOptions) -> Self {
        Self {
            image: Image::new(render_options.height.into(), render_options.width.into()),
            options: render_options,
            ..Default::default()
        }
    }

    pub fn load_file(&mut self, filepath: &str) -> Result<()> {
        self.file = Some(ModelFile::open_file(filepath)?);
        self.reload()?;
        Ok(())
    }

    pub fn load_from_string(&mut self, str: &str) {
        self.file = Some(ModelFile::from(str))
    }

    pub fn reload(&mut self) -> Result<()> {
        match self.file {
            Some(ref mut file) => {
                file.load();
                Ok(())
            }
            None => Err(RenderError("No file to reload!".to_string())),
        }
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

    pub fn update_file_render(&mut self) -> Result<()> {
        let model_file_drawer = ModelFileDrawer {
            options: &self.options,
            model_file: self.file.as_ref().unwrap(),
        };
        model_file_drawer.draw_on(&mut self.image);
        Ok(())
    }

    // pub fn image_buffer(&self) -> image_lib::ImageBuffer<image_lib::Rgb<u8>, Vec<u8>> {
    pub fn image_buffer(&self) -> ImageBuffer {
        // self.image.render_to_buffer()
        self.image.buffer.clone() //TODO remove clone
    }
    pub fn width(&self) -> u32 {
        self.image.width
    }
    pub fn height(&self) -> u32 {
        self.image.height
    }

    pub fn draw(&mut self) {
        for drawable in self.render_queue.iter() {
            drawable.draw_on(&mut self.image);
        }
    }
}

pub trait HasRenderQueue {
    fn queue_push(&mut self, d: Box<dyn Drawable>);
}

impl HasRenderQueue for Render {
    // pub fn queue_push(&mut self, d: Box<dyn Drawable>) {
    fn queue_push(&mut self, d: Box<dyn Drawable>) {
        // Vec<Box<dyn Drawable>>
        self.render_queue.push(d);
    }
}

#[derive(Debug, Clone, Default)]
pub struct RenderError(pub String);
impl error::Error for RenderError {}
impl From<Box<dyn std::error::Error>> for RenderError {
    fn from(std_error: Box<dyn std::error::Error>) -> Self {
        RenderError(std_error.to_string())
    }
}
impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Render failed: {:?}", self.0)
    }
}
