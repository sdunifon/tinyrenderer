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

    i.draw(&Vertex {
        x: 50.0,
        y: 40.0,
        z: 40.0,
    });

    let file = ModelFile {
        filename: "head.obj",
    };

    let verticies = file.vertex_parse();

    for vertex in verticies {
        i.draw(&vertex)
    }
    i.render(filename);
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
