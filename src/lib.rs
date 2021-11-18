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
mod triangle;
pub mod utils;
mod vertex;

pub mod test_helper;

pub use bounds::{Boundable, BoundingBox};
use fillable::Fillable;
pub use image::*;
use line::Line;
pub use model_file::ModelFile;
use regex::Regex;
pub use triangle::{Triangle, Triangles};
pub use utils::*;
pub use vertex::{HasVerticies, Vertex, Vertices};

pub const IMAGE_SIZE: usize = 300; //TOFIX: increasing this over 500 seems to overflow the stack

pub fn make_image() -> Image<IMAGE_SIZE, IMAGE_SIZE> {
    let mut image = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    image.draw(&Vertex {
        x: 50,
        y: 40,
        z: 40,
    });

    let file = ModelFile::open("head.obj");

    let verticies = file.vertex_parse(IMAGE_SIZE, IMAGE_SIZE);

    let triangles = file.face_parse(&verticies);
    for triangle in &triangles {
        image.draw(triangle)
    }
    for vertex in &verticies {
        image.draw(vertex)
    }
    for triangle in &triangles {
        triangle.fill(&mut image, random_color())
    }
    image
}

pub fn draw_triangle(
    triangle: Triangle,
    fill: bool,
) -> Result<Image<IMAGE_SIZE, IMAGE_SIZE>, Box<dyn std::error::Error>> {
    let mut image = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    image.draw(&triangle);
    if fill {
        triangle.fill(&mut image, random_color());
    }
    Ok(image)
}

pub fn render_triangle() -> Image<IMAGE_SIZE, IMAGE_SIZE> {
    let triangle = Triangle::new([
        Vertex { x: 50, y: 50, z: 0 },
        Vertex {
            x: 75,
            y: 100,
            z: 0,
        },
        Vertex {
            x: 100,
            y: 50,
            z: 0,
        },
    ]);

    let mut image = Image::<IMAGE_SIZE, IMAGE_SIZE>::new();

    image.draw(&triangle);
    triangle.fill(&mut image, random_color());
    image
}

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
                x: 50,
                y: 100,
                z: 0,
            },
            Vertex {
                x: 75,
                y: 100,
                z: 0,
            },
            Vertex {
                x: 100,
                y: 50,
                z: 0,
            },
        ])
    }

    #[test]
    fn render_triangle_test() {
        render_triangle();
    }
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

    #[test]
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
    //         filename: "head.obj",
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
