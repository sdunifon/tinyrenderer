
a
use std::error::Error;
use tinyrenderer::image::traits::Drawable;
use tinyrenderer::{display_window, Circle};
use tinyrenderer::{Render, Vertex};

#[cfg_attr(feature = "native_image_render", show_image::main)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut renderer = Render::default();

    let circle = Circle::new(Xy(10,15),25));

    triangle.draw(&mut renderer.image);
    // renderer.update_file_render()?;
    display_window(&renderer)
}
