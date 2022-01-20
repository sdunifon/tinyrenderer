use crate::drawable::Drawable;
use crate::{color, Canvas, RenderError, Xy};

pub struct Point(pub Xy);

impl std::ops::Deref for Point {
    type Target = Xy;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drawable for Point {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<(), RenderError> {
        canvas.set(self.0, &color::WHITE);
        Ok(())
    }
}
