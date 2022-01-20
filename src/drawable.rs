use crate::Canvas;

pub trait Drawable {
    fn draw(&self, drawer: &mut dyn Canvas);
}
