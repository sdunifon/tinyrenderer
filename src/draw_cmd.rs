use std::ops::{Add, Range};

use crate::{color, BoundingBox, Circle, Color, Drawable, ImageBuffer, ToImageBuffer, Xy};
#[derive(Debug,PartialEq,Clone)]
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

impl Add<Xy> for &DrawCmd {
    type Output = DrawCmd;
    fn add(self, other: Xy) -> Self::Output {
        match self {
            DrawCmd::Set(xy) => DrawCmd::Set(*xy + other),
            DrawCmd::SetColor(xy, color) => DrawCmd::SetColor(*xy + other, *color),
            DrawCmd::Line(xy, xy2, color) => DrawCmd::Line(*xy + other, *xy2 + other,  *color),
            DrawCmd::Circle(xy, radius, color) => DrawCmd::Circle(*xy + other, *radius, *color),
            DrawCmd::List(list) => DrawCmd::List(list.into_iter().map(|cmd| cmd + other).collect()),
            DrawCmd::Clear(bbox) => DrawCmd::Clear(*bbox + other),
            DrawCmd::CopyBuffer(buffer) => todo!( "need to copy the buffer")
        }
    }
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

#[macro_export]
macro_rules! draw_set_cmd {
    ($(($a:literal, $b:literal)),+) => {
        {
            let mut v: Vec<DrawCmd> = Vec::new();
            $(
                v.push(DrawCmd::Set(Xy($a,$b)));
            )+
            v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xy_add() {
        let cmd = DrawCmd::Set(Xy(1, 2));
        let xy = Xy(3, 4);
        let cmd3 = &cmd + xy;
        assert_eq!(cmd3, DrawCmd::Set(Xy(4, 6)));
    }

}