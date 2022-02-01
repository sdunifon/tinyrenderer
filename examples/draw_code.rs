#![feature(box_syntax)]
use native_display::display_window;
use std::error::Error;
use std::ops::Add;
use tinyrenderer::{Boundable, Circle, Color, Digit, DrawBoundable, Line, Point, Triangle, Xy};
use tinyrenderer::{DrawAt, Drawable};
use tinyrenderer::{Render, Vertex};

#[cfg_attr(feature = "native_image_render", show_image::main)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut renderer = Render::default();

    let circle = Circle::new(Xy(150, 165), 55);

    renderer.render_queue.push(box circle);

    // circle.draw_at(|setter| setter.set())
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

// let xys = [Xy(1,2),Xy(2,2),Xy(3,3)]
// let f1 = (xy)-> xy {

//     xy + Xy(50,50)
// }
