use std::error::Error;
use tinyrenderer::image::traits::Drawable;
use tinyrenderer::{color::WHITE, display_window, Color};
use tinyrenderer::{Line, Render, Vertex};

#[cfg_attr(feature = "native_image_render", show_image::main)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut renderer = Render::default();
    let line = Line::from_vertices(&Vertex::new(0.1, 0.5, 0.), &Vertex::new(0.3, 0.7, 0.0));
    let line2 = Line {
        v1: Vertex::new(0.9, 0.3, 0.),
        v2: Vertex::new(0.8, 0.1, 0.0),
        color: Color {
            r: 0,
            g: 128,
            b: 92,
        },
    };
    line.draw(&mut renderer.image);
    line2.draw(&mut renderer.image);
    display_window(&renderer)
}
