use crate::drawable::DrawInstructions;
use crate::{Boundable, Canvas, DrawCmd, Drawable, RenderError, Xy};

pub struct DrawAt(pub Xy, pub Box<dyn DrawBoundable<i32>>);

pub trait DrawBoundable<T>: Drawable + Boundable<T> {}
impl<'a> DrawAt {
    fn translate_to(&self, point: Xy) -> Xy {
        self.0 + point
    }
    fn drawable(&self) -> &Box<dyn DrawBoundable<i32>> {
        &self.1
    }
}

impl<'a> Boundable<i32> for DrawAt {
    fn bounding_box(&self) -> crate::BoundingBox<i32> {
        self.drawable().bounding_box()
    }
}
impl<'a> Drawable for DrawAt {
    fn draw_on(self: &DrawAt, canvas: &mut dyn Canvas) -> Result<(), crate::RenderError> {
        // self.drawable().draw_on_passthrough(self)?;
        todo!();
        Ok(())
    }
}
// impl<'a> DrawInstructions for DrawAt {
impl<'a> DrawAt {
    // fn draw_code(&'a self) -> Box<dyn Fn(&'a mut dyn Canvas) -> Result<(), RenderError>> {
    //     let drawable = self.drawable();
    //     Box::new(move |canvas: &mut dyn Canvas| -> Result<(), RenderError> {
    //         drawable.draw_on(canvas)
    //     })
    // }

    fn draw_code(&'a self) -> impl Fn(&'a mut dyn Canvas) {
        let drawable = self.drawable();
        move |canvas: &mut dyn Canvas| {
            drawable.draw_on(canvas);
        }
    }
}
// should this keep a reference to the image canvas.. that would allow us to stack multiple

impl Canvas for DrawAt {
    fn set(&mut self, point: Xy, color: &crate::Color) {
        todo!()
    }

    fn get(&self, point: Xy) -> &crate::Color {
        todo!()
    }

    fn draw(&mut self, drawable: &dyn Drawable) {
        todo!()
    }

    fn height(&self) -> u32 {
        todo!()
    }

    fn width(&self) -> u32 {
        todo!()
    }

    fn scalar(&self) -> &crate::Scalar {
        todo!()
    }

    fn scale(&self, vertex: &crate::Vertex) -> Xy {
        todo!()
    }
}



fn change_at(xy: Xy, cmds: Vec<DrawCmd>) -> impl Fn(&dyn Canvas) {
    cmds.iter().map(|cmd| match cmd {
        DrawCmd::Set(_) => {}
        DrawCmd::SetColor(_, _) => {}
        DrawCmd::Line(_, _, _) => {}
        DrawCmd::Circle(_, _, _) => {}
        DrawCmd::List(_) => {}
        DrawCmd::Clear(_) => {}
        // DrawCmd::Fill(_) => {}
        // DrawCmd::Trace(_) => {}
        DrawCmd::CopyBuffer(_) => {}
    });
    |canvas| println!(" in closure ")
}

// type ddfour Fn(DrawCmd) -> DrawCmd
// type ddfive Fn(Fn(DrawCmd)) -> Fn(DrawCmd)
// fn change_at(f: Fn(dyn Canvas))-> Fn(dyn Canvs)

//     fn draw_at(&self, draw_position: Xy) -> Box<dyn Fn(dyn Fn(Xy)) -> Xy> {
//         todo!()
//         // Ok(self.draw_on(|xy| xy + draw_position))
//     }



#[cfg(test)]
mod tests {
    use crate::{DrawCmd, Xy};

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

    #[test]
    fn draw_set_cmd_macro_test() {
        let cmds: Vec<DrawCmd> = vec![
            DrawCmd::Set(Xy(1, 2)),
            DrawCmd::Set(Xy(3, 4)),
            DrawCmd::Set(Xy(5, 6)),
        ];
        let macro_cmds = draw_set_cmd!((1,2),(3,4),(5,6));
        assert_eq!(cmds[0],macro_cmds[0]);
        assert_eq!(cmds[1],macro_cmds[1]);
        assert_eq!(cmds[2],macro_cmds[2]);
    }

}

