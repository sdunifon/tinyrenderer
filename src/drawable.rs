use crate::RenderError;
use crate::canvas::Canvas;

pub trait Drawable {
    fn draw_on(&self, canvas: &mut dyn Canvas) -> Result<(), RenderError>;
}
