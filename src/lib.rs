#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(destructuring_assignment)]
#![feature(str_split_as_str)]
#![feature(test)]

#[macro_use]
extern crate lazy_static;

mod face;
mod image;
mod line;
mod model_file;
mod vertex;

mod test_helper;

use face::{Face, Faces};
pub use image::*;
use line::Line;
use model_file::ModelFile;
use regex::Regex;
use vertex::{Vertex, Vertices};

pub fn make_image(filename: &str) {
    let mut i = Image::<500, 500>::new();

    i.draw(&Vertex {
        x: 50,
        y: 40,
        z: 40,
    });

    let file = ModelFile {
        filename: "head.obj",
    };

    let verticies = file.vertex_parse(500, 500);

    let faces = file.face_parse(&verticies);
    for face in &faces {
        i.draw(face)
    }
    for vertex in &verticies {
        i.draw(vertex)
    }
    i.render(filename);
}

#[cfg(test)]
mod tests {
    use crate::test_helper::assert_file_creation;

    use super::*;
    use std::fs;
    use std::path::Path;
    extern crate test;
    use test::Bencher;

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
    #[bench]
    fn bench_make_image(b: &mut Bencher) {
        assert_file_creation("test_render.tga", |filename: &str| {
            b.iter(|| make_image(filename));
        });
    }
}
