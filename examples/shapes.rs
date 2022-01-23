#![feature(box_syntax)]
use native_display::display_window;
use std::error::Error;
use std::ops::Add;
use tinyrenderer::Drawable;
use tinyrenderer::{Boundable, Circle, Color, Digit, Line, Point, Triangle, Xy};
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

    let one = Digit::Three;

    let bounding_box = circle.bounding_box();

    let bounding_box2 = Circle::new(Xy(125, 265), 105).bounding_box();

    let point = Point(Xy(50, 50));
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

    // line.draw(&mut renderer.image);
    // line2.draw(&mut renderer.image);
    // triangle.draw(&mut renderer.image);
    // circle.draw(&mut renderer.image);

    renderer.render_queue.push(box circle);
    renderer.render_queue.push(box bounding_box);
    renderer.render_queue.push(box bounding_box2);
    renderer.render_queue.push(box triangle); // todo make method to just accept circle without box
    renderer.render_queue.push(box point);
    renderer.render_queue.push(box one);
    renderer.render_queue.push(box line);
    renderer.render_queue.push(box line2);

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

// #[cfg(syntax_synta)]
// mod syntax_ideask{
//     fn syntax() {

//         "23453".draw_at(23,43).draw_on(canvas);
//         "23453".at(23,43).on(canvas);

//             DrawAt((45,45), 1000.into());

//             DrawAt((45,45), Triangle::new));
//
//             Digit(2).in(Number)
//
//         let a: Vec<Digit> = [2,3,4,5,6] // draw each one next to each other
//             Vec<Digit>::draw() ->
//             // a[0].draw_at(10,10)
//             // a[1].draw_at(20,10)
//             // a[2].draw_at(30,10)
//         a.draw_at(10,10).draw_on()
//     }

// }
