use crate::{BoundingBox, Circle, Color, Drawable, ImageBuffer, Xy};

pub enum DrawCmd<'a> {
    Set(Xy, Color),
    Line(Xy, Xy, Color),
    Circle(Xy, u32, Color),
    List(Vec<DrawCmd<'a>>),
    Clear(BoundingBox<i32>),
    Fill(Box<dyn Drawable>),
    Trace(Box<dyn Drawable>),
    CopyBuffer(&'a ImageBuffer),
}

impl Drawable for Vec<&dyn Drawable> {
    fn draw_on(&self, canvas: &mut dyn crate::Canvas) -> Result<(), crate::RenderError> {
        todo!()
    }

    fn draw_on_passthrough(&self, canvas: &dyn crate::Canvas) -> Result<(), crate::RenderError> {
        unimplemented!();
    }
}

impl Drawable for Vec<DrawCmd> {
    fn draw_on(&self, canvas: &mut dyn crate::Canvas) -> Result<(), crate::RenderError> {
        todo!()
    }
}
impl DrawCmd for DrawCmd::List {}

impl Drawable for DrawCmd {
    fn draw_on(&self, canvas: &mut dyn crate::Canvas) -> Result<(), crate::RenderError> {
        match self {
            DrawCmd::Set(_, _) => todo!(),
            DrawCmd::Line(_, _, _) => todo!(),
            DrawCmd::Circle(_, _, _) => todo!(),
            DrawCmd::List(_) => todo!(),
            DrawCmd::Clear(_) => todo!(),
            DrawCmd::Fill(_) => todo!(),
            DrawCmd::Trace(_) => todo!(),
            DrawCmd::CopyBuffer(_) => todo!(),
        }
    }
}
