use std::ops::Range;

use crate::{color, BoundingBox, Circle, Color, Drawable, ImageBuffer, ToImageBuffer, Xy};
#[derive(Debug,PartialEq)]
pub enum DrawCmd {
    Set(Xy),
    SetColor(Xy, Color),
    Line(Xy, Xy, Color),
    Circle(Xy, u32, Color),
    List(Vec<DrawCmd>),
    Clear(BoundingBox<i32>),
    // Fill(Box<dyn Drawable>),
    // Trace(Box<dyn Drawable>),
    CopyBuffer(Box<ImageBuffer>),
    // Outline(&'a ImageBuffer),
    // Function(&'a dyn Fn(i32) -> DrawCmd<'a>, Range<i32>),
}

impl Drawable for Vec<&dyn Drawable> {
    fn draw_on(&self, canvas: &mut dyn crate::Canvas) -> Result<(), crate::RenderError> {
        todo!()
    }

    // fn draw_on_passthrough(&self, canvas: &dyn crate::Canvas) -> Result<(), crate::RenderError> {
    //     unimplemented!();
    // }
}
pub trait ToDrawCommands {
    fn to_draw_commands(&self) -> Vec<DrawCmd>;
}

impl<'a> Drawable for Vec<DrawCmd> {
    fn draw_on(&self, canvas: &mut dyn crate::Canvas) -> Result<(), crate::RenderError> {
        for cmd in self {
            cmd.draw_on(canvas)?
        }
        Ok(())
    }
}

impl<'a> ToImageBuffer for Vec<DrawCmd> {
    fn to_image_buffer(self) -> ImageBuffer {
        todo!()
    }
}
// impl DrawCmd for DrawCmd::List {}

impl<'a> Drawable for DrawCmd {
    fn draw_on(&self, canvas: &mut dyn crate::Canvas) -> Result<(), crate::RenderError> {
        match self {
            DrawCmd::Set(xy) => canvas.set(*xy, &color::GREEN),
            DrawCmd::Line(_, _, _) => todo!(),
            DrawCmd::Circle(_, _, _) => todo!(),
            DrawCmd::List(_) => todo!(),
            DrawCmd::Clear(_) => todo!(),
            // DrawCmd::Fill(_) => todo!(),
            // DrawCmd::Trace(_) => todo!(),
            DrawCmd::CopyBuffer(_) => todo!(),
            DrawCmd::SetColor(_, _) => todo!(),
            // DrawCmd::Outline(_) => todo!(),
            // DrawCmd::Function(_, _) => todo!(),
        }
        Ok(())
    }
}
