use crate::drawable::Drawable;
use crate::{color, RenderError, Xy};
use crate::canvas::Canvas;

pub struct Point(pub Xy);

impl std::ops::Deref for Point {
    type Target = Xy;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drawable for Point {
    fn draw_on(&self, canvas: &mut dyn Canvas) -> Result<(), RenderError> {
        canvas.set(self.0, &color::WHITE);
        Ok(())
    }
}
