use crate::canvas::Canvas;
use crate::RenderError;

pub trait Drawable {
    fn draw_on(&self, canvas: &mut dyn Canvas) -> Result<(), RenderError>;
    fn draw_on_passthrough(&self, canvas: &dyn Canvas) -> Result<(), RenderError> {
        unimplemented!();
    }
}
