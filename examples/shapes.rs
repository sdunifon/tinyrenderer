use std::error::Error;
use tinyrenderer::image::traits::Drawable;
use tinyrenderer::{display_window, Circle, Triangle, Xy};
use tinyrenderer::{Render, Vertex};

#[cfg_attr(feature = "native_image_render", show_image::main)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut renderer = Render::default();

    let circle = Circle::new(Xy(150, 165), 55);

    let triangle = Triangle::new([
        Vertex::new(0.3, 0.7, 0.2),
        Vertex::new(0.6, 0.9, 0.2),
        Vertex::new(0.2, -0.3, 0.4),
    ]);
    // triangle.draw(&mut renderer.image);
    // circle.draw(&mut renderer.image);

    renderer.render_queue.push(Box::new(circle)); // todo make method to just accept circle without box
    renderer.render_queue.push(Box::new(triangle)); // todo make method to just accept circle without box

    for i in 1..15 {
        renderer.render_queue.push(Box::new(Circle::new(
            Xy(45 * i + 100, 25 * i + 100),
            4 * i as u32,
        )));
    }
    renderer.draw();

    // renderer.update_file_render()?;
    display_window(&renderer)
}
