use super::image::Drawable;
use super::*;
use std::path::Path;

const RENDER_WIDTH: usize = 400;
const RENDER_HEIGHT: usize = 400;

pub struct Render {
    file: Option<ModelFile>,
    image: Image<RENDER_HEIGHT, RENDER_WIDTH>,
}

impl Default for Render {
    fn default() -> Self {
        Self {
            file: Default::default(),
            image: Image::<RENDER_HEIGHT, RENDER_WIDTH>::new(),
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

    pub fn load_file(&mut self, filepath: &str) {
        assert!(Path::new(filepath).exists(), "{} doesn't exist!", filepath);
        self.file = Some(ModelFile::open(filepath));
        self.file
            .as_mut()
            .unwrap()
            .load(RENDER_WIDTH, RENDER_HEIGHT);
    }

    pub fn update(&mut self) {
        for vertex in &self.file.as_ref().unwrap().verticies {
            self.image.draw(vertex);
        }
        for triangle in &self.file.as_ref().unwrap().triangles {
            triangle.fill(&mut self.image)
        }
    }
    pub fn width(&self) -> usize {
        self.image.width
    }
    pub fn height(&self) -> usize {
        self.image.height
    }
}
