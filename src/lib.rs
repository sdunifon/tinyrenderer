#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(str_split_as_str)]
#![feature(test)]
#![feature(generic_arg_infer)]
// #![feature(trace_macros)]

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "native_image_render")]
use show_image::{create_window, event};

mod bounds;
pub mod fillable;
pub mod image;
mod line;
mod math;
mod model_file;
pub mod render;
mod rendering_traits;
mod triangle;
pub mod utils;
mod vector;
mod vertex;

mod camera;
pub mod circle;
mod digit;
pub mod point;
pub mod test_helper;

use std::error::Error;

pub use bounds::{Boundable, BoundingBox, DetectInside};
pub use circle::Circle;
pub use digit::Digit;
pub use fillable::Fillable;
pub use image::color::{self, Color, Colorful};
pub use image::*;
pub use image::{ImageBuffer, Xy};
pub use line::{Line, Line2d};
pub use model_file::ModelFile;
pub use point::Point;
use regex::Regex;
use render::RenderError;
pub use render::{Render, RenderOptions};
pub use rendering_traits::*;
pub use triangle::{Triangle, Triangles};
pub use utils::*;
pub use vector::Vector3;

pub use vertex::{HasTriangleVertices, NormalizedVertices, Vertex, Vertices};

pub fn load_file(filename: &str) -> Result<Render, RenderError> {
    let mut render = Render::default();
    render.load_file(filename)?;
    render.update_file_render()?;
    Ok(render)
}

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
        make_image().unwrap().render(filename);
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
