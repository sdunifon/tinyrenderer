use crate::{Canvas, RenderError};

pub trait Drawable {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<(), RenderError>;
}
