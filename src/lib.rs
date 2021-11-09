#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(destructuring_assignment)]
#![feature(str_split_as_str)]

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
    let v1 = verticies[24].clone();
    let v2 = verticies[25].clone();
    let v3 = verticies[26].clone();

    let v11 = verticies[124].clone();
    let v12 = verticies[125].clone();
    let v13 = verticies[126].clone();
    for vertex in &verticies {
        i.draw(vertex)
    }

    // f 24/1/24 25/2/25 26/3/26

    // let l1 = Line::from_vertices(&v1, &v2);
    // let l2 = Line::from_vertices(&v1, &v3);
    // let l3 = Line::from_vertices(&v2, &v3);

    // i.draw(&l1);
    // i.draw(&l2);
    // i.draw(&l3);

    // let f = Face::new([v11, v12, v13]);
    // i.draw(&f);

    let faces = file.face_parse(&verticies);
    for face in &faces {
        i.draw(face)
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
