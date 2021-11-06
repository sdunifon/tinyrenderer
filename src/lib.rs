#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(destructuring_assignment)]
#![feature(str_split_as_str)]

#[macro_use]
extern crate lazy_static;

mod image;
mod line;
mod model_file;
mod vertex;

mod test_helper;

pub use image::*;
use model_file::*;
use vertex::Vertex;

pub fn make_image(filename: &str) {
    let mut i = Image::<500, 500>::new();
    i.set(Pt(50, 50), Px { r: 0, g: 255, b: 0 });

    i.draw(&Vertex {
        x: 50.0,
        y: 40.0,
        z: 40.0,
    });
    i.render(filename);
}

pub fn display_model_file(filename: &str) {
    let m = ModelFile { filename };
    m.display();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn make_image_test() {
        let filename = "lib_test_render.tga";
        if Path::new(filename).exists() {
            fs::remove_file(filename).unwrap();
        }
        make_image(filename);
        assert!(Path::new(filename).exists(), "rendered image not found");
        fs::remove_file(filename).unwrap();
    }
}
