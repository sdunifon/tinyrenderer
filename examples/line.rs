use tinyrenderer::display_window;
use tinyrenderer::{Line, Render, Vertex};

#[cfg_attr(feature = "native_image_render", show_image::main)]
fn main() {
    let mut renderer = Render::default();
    let line = Line::from_vertices(&Vertex::new(0.1, 0.5, 0.), &Vertex::new(0.3, 0.7, 0.0));
    line.draw(&mut renderer.image);
    display_window(&renderer);
}
