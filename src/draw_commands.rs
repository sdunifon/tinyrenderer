use crate::{Circle, Color, Drawable, Xy};

enum DrawCommands {
    Set(Xy, Color),
    Line(Xy, Xy, Color),
    Drawable(Box<dyn Drawable>),
    Circle(Xy, u32, Color),
}
