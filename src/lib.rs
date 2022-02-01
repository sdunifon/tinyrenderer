#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(str_split_as_str)]
#![feature(test)]
#![feature(generic_arg_infer)]
// #![feature(trace_macros)]

#[macro_use]
extern crate lazy_static;

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
mod canvas;
pub mod circle;
pub mod color;
mod digit;
mod draw_at;
mod draw_cmd;
mod drawable;
mod image_buffer;
pub mod point;
pub mod test_helper;
mod xy;

pub use bounds::{Boundable, BoundingBox, DetectInside};
pub use canvas::Canvas;
pub use circle::Circle;
pub use draw_cmd::ToDrawCommands;
pub use color::{Color, Colorful};
pub use digit::Digit;
pub use draw_at::DrawAt;
pub use draw_at::DrawBoundable;
pub use draw_cmd::DrawCmd;
pub use drawable::Drawable;
pub use fillable::Fillable;
pub use image::*;
pub use image_buffer::{ImageBuffer, ToImageBuffer};
pub use line::{Line, Line2d};
pub use model_file::ModelFile;
pub use point::Point;
pub use regex::Regex;
pub use render::RenderError;
pub use render::{Render, RenderOptions};
pub use rendering_traits::*;
pub use triangle::{Triangle, Triangles};
pub use utils::*;
pub use vector::Vector3;
pub use vertex::{HasTriangleVertices, NormalizedVertices, Vertex, Vertices};
pub use xy::Xy;

pub fn load_file(filename: &str) -> Result<Render, RenderError> {
    let mut render = Render::default();
    render.load_file(filename).or({
        let mut path = project_root::get_project_root().unwrap();
        path.push(filename);
        render.load_file(
            path.to_str()
                .ok_or(RenderError("invalid path".to_string()))?,
        )
    })?;

    render.update_file_render()?;
    Ok(render)
}
