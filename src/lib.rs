#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(destructuring_assignment)]
#![feature(str_split_as_str)]
#![feature(test)]
#![feature(generic_arg_infer)]
// #![feature(trace_macros)]

extern crate nalgebra as na;

#[macro_use]
extern crate lazy_static;

mod bounds;
pub mod fillable;
mod image;
mod line;
mod math;
mod model_file;
pub mod render;
mod rendering_traits;
mod triangle;
pub mod utils;
mod vector;
mod vertex;

pub mod test_helper;

// use std::{
//     mem,
//     thread::{self, Builder},
// };

pub use bounds::{Boundable, BoundingBox};
use fillable::Fillable;
pub use image::color::*;
use image::*;
pub use image::{ImageBuffer, Xy};
use line::Line;
pub use model_file::ModelFile;
use na::Vector3;
use regex::Regex;
pub use render::Render;
use rendering_traits::*;
pub use triangle::{Triangle, Triangles};
pub use utils::*;

// pub use vector::Vector3;
pub use vertex::{HasTriangleVerticies, NormalizedVertices, Vertex, Vertices};
pub const IMAGE_HEIGHT: u32 = 1024; //TOFIX: increasing this over 500 seems to overflow the stack
pub const IMAGE_WIDTH: u32 = 1024; //TOFIX: increasing this over 500 seems to overflow the stack

pub fn make_image() -> Image {
    let mut image = Image::new(IMAGE_HEIGHT, IMAGE_WIDTH);

    image.draw(&Vertex {
        x: 50.,
        y: 40.,
        z: 40.,
    });

    let file = ModelFile::open_file("assets/head.obj");

    let verticies = file.vertex_parse();

    let triangles = file.face_parse(&verticies);
    for triangle in &triangles {
        image.draw(triangle)
    }
    for vertex in &verticies {
        image.draw(vertex)
    }
    for triangle in &triangles {
        triangle.fill(&mut image)
    }
    image
}

pub fn draw_triangle(triangle: Triangle, fill: bool) -> Result<Image, Box<dyn std::error::Error>> {
    let mut image = Image::new(IMAGE_HEIGHT, IMAGE_WIDTH);

    image.draw(&triangle);
    if fill {
        triangle.fill(&mut image);
    }
    Ok(image)
}

pub fn render_triangle() -> Image {
    let triangle = Triangle::new([
        Vertex {
            x: 50.,
            y: 50.,
            z: 0.,
        },
        Vertex {
            x: 75.,
            y: 100.,
            z: 0.,
        },
        Vertex {
            x: 100.,
            y: 50.,
            z: 0.,
        },
    ]);

    let mut image = Image::new(IMAGE_HEIGHT, IMAGE_WIDTH);

    image.draw(&triangle);
    triangle.fill(&mut image);
    image
}

// fn large_stack_thread<'a>(f: &'a fn() -> ()) {
//     let handler = thread::Builder::new()
//         .stack_size(200 * 1024 * 1024)
//         .spawn(|| {
//             let y: [u64; 10000000] = [1; 10000000];
//             println!("the arrays allocated {} bytes", mem::size_of_val(&y));
//             f();
//         })
//         .expect("can't spawn thread");

//     handler.join().expect("something's wrong with the thread");
// }
// pub fn render(image: &mut Image, faces: &Faces, verticies: &Vertices) {}
#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use std::path::Path;
    extern crate test;

    fn triangle() -> Triangle {
        Triangle::new([
            Vertex {
                x: 50.,
                y: 100.,
                z: 0.,
            },
            Vertex {
                x: 75.,
                y: 100.,
                z: 0.,
            },
            Vertex {
                x: 100.,
                y: 50.,
                z: 0.,
            },
        ])
    }

    #[test]
    #[ignore]
    fn render_triangle_test() {
        render_triangle();
    }
    #[test]
    #[ignore]
    fn make_image_test() {
        let filename = "lib_test_render.tga";
        if Path::new(filename).exists() {
            fs::remove_file(filename).unwrap();
        }
        make_image().render(filename);
        assert!(Path::new(filename).exists(), "rendered image not found");
        fs::remove_file(filename).unwrap();
    }

    #[test]
    #[ignore]
    fn draw_triangle_test() {
        assert!(draw_triangle(triangle(), false).is_ok());
        assert!(draw_triangle(triangle(), true).is_ok());
    }
    // #[bench]
    // fn bench_make_image(b: &mut Bencher) {
    //     assert_file_creation("test_render.tga", |filename: &str| {
    //         b.iter(|| make_image().render(filename));
    //     });
    // }

    // #[bench]
    // fn bench_render_only(b: &mut Bencher) {
    //     const IMAGE_SIZE: usize = 500;
    //     let mut i = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    //     let file = ModelFile {
    //         filename: "assets/head.obj",
    //     };

    //     let verticies = file.vertex_parse(IMAGE_SIZE, IMAGE_SIZE);

    //     let faces = file.face_parse(&verticies);

    //     b.iter(|| {
    //         for face in &faces {
    //             i.draw(face)
    //         }
    //         for vertex in &verticies {
    //             i.draw(vertex)
    //         }
    //     });
    //     i.render(file.filename);
    // }
}
