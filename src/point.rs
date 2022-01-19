use crate::{color, Drawable, Xy};

pub struct Point(pub Xy);

impl std::ops::Deref for Point {
    type Target = Xy;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drawable for Point {
    fn draw(&self, drawer: &mut dyn crate::Canvas) {
        drawer.set(self.0, &color::WHITE);
    }
}
