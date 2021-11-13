#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(destructuring_assignment)]
#![feature(str_split_as_str)]
#![feature(test)]

#[macro_use]
extern crate lazy_static;

mod fillable;
mod image;
mod line;
mod model_file;
mod triangle;
pub mod utils;
mod vertex;

pub mod test_helper;

use fillable::Fillable;
pub use image::*;
use line::Line;
pub use model_file::ModelFile;
use regex::Regex;
use triangle::{Triangle, Triangles};
pub use utils::*;
use vertex::*;

const IMAGE_SIZE: usize = 500; //TOFIX: increasing this over 500 seems to overflow the stack

pub fn make_image() -> Image<IMAGE_SIZE, IMAGE_SIZE> {
    let mut image = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    image.draw(&Vertex {
        x: 50,
        y: 40,
        z: 40,
    });

    let file = ModelFile {
        filename: "head.obj",
    };

    let verticies = file.vertex_parse(IMAGE_SIZE, IMAGE_SIZE);

    let triangles = file.face_parse(&verticies);
    for triangle in &triangles {
        image.draw(triangle)
    }
    for vertex in &verticies {
        image.draw(vertex)
    }
    image
}

pub fn draw_triangle() -> Image<IMAGE_SIZE, IMAGE_SIZE> {
    let mut image = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    let t = Triangle {
        vertices: [
            Vertex {
                x: 100,
                y: 100,
                z: 0,
            },
            Vertex {
                x: 150,
                y: 200,
                z: 0,
            },
            Vertex {
                x: 200,
                y: 100,
                z: 0,
            },
        ],
    };
    image.draw(&t);
    t.fill::<IMAGE_SIZE, IMAGE_SIZE>(image, BLUE);
    image
}

// pub fn render(image: &mut Image, faces: &Faces, verticies: &Vertices) {}
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
        make_image().render(filename);
        assert!(Path::new(filename).exists(), "rendered image not found");
        fs::remove_file(filename).unwrap();
    }
    #[bench]
    fn bench_make_image(b: &mut Bencher) {
        assert_file_creation("test_render.tga", |filename: &str| {
            b.iter(|| make_image().render(filename));
        });
    }

    #[bench]
    fn bench_render_only(b: &mut Bencher) {
        const IMAGE_SIZE: usize = 500;
        let mut i = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

        let file = ModelFile {
            filename: "head.obj",
        };

        let verticies = file.vertex_parse(IMAGE_SIZE, IMAGE_SIZE);

        let faces = file.face_parse(&verticies);

        b.iter(|| {
            for face in &faces {
                i.draw(face)
            }
            for vertex in &verticies {
                i.draw(vertex)
            }
        });
        i.render(file.filename);
    }
}
