use crate::canvas::Canvas;
use crate::{Color, DrawAt, DrawCmd, RenderError, Xy};

pub trait Drawable {
    fn draw_on(&self, canvas: &mut dyn Canvas) -> Result<(), RenderError>;
    fn draw_on_passthrough(&self, canvas: &dyn Canvas) -> Result<(), RenderError> {
        unimplemented!();
    }
}

pub trait DrawInstructions {
    // fn draw_code(&self) -> Box<dyn Fn(&mut dyn Canvas) -> Result<(), RenderError>>;
    fn draw_code(&self) -> Box<dyn Fn(&mut dyn PxSet) -> Result<(), RenderError>> {
        let xy = Xy(3, 4);

        todo!();
        // |setter: &dyn PxSet| {
        //     let xy = Xy(0, 0);
        //     setter.px_set(xy + Xy(3, 4));
        //     let a = 4;
        // }
    }
}
//sketch
impl DrawInstructions for DrawAt {
    fn draw_code(&self) -> Box<dyn Fn(&mut dyn PxSet) -> Result<(), RenderError>> {
        todo!();
        // let xy = self.0;

        // Box::new(|setter: Box<dyn PxSet>| {
        //     let xy = Xy(0, 0);
        //     setter.px_set(xy + Xy(3, 4));
        //     let a = 4;
        // })
    }
}

//sketch
pub trait PxSet {
    fn px_set(&self, px: Xy) {}
    fn color_px_setter(&self, color: Color) -> impl dyn Fn(Xy) -> Result<(), RenderError> {
        |xy| self.px_set_color(xy, color)
    }
    fn px_set_color(&self, px: Xy, color: Color);

    fn set(&self, xy: Xy) -> Xy {
        Xy(5, 5)
    }

    fn draw_commands(&self) -> Vec<DrawCmd> {
        let commands: Vec<DrawCmd> = Vec::new();
    }
}

//sketch
impl<F> PxSet for F
where
    F: Drawable,
{
    fn px_set(&self, px: Xy) {}

    // fn color_px_setter(&self, color: Color) -> impl Fn(Xy) -> Result<(), RenderError>> {
    //     |xy| self.px_set_color(xy, color)
    // }

    fn set(&self, xy: Xy) -> Xy {
        let a = 5;
        todo!();
    }

    fn draw_commands(&self) -> Vec<DrawCmd> {
        let commands: Vec<DrawCmd> = Vec::new();
    }

    fn px_set_color(&self, px: Xy, color: Color) {
        todo!()
    }
}
