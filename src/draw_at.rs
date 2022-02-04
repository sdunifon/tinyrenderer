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

fn change_at_test() {
    let vc: Vec<DrawCmd> = vec![
        DrawCmd::Set(Xy(1, 2)),
        DrawCmd::Set(Xy(2, 2)),
        DrawCmd::Set(Xy(3, 2)),
    ];
    let pts = draw_set_points!([(3,2),(4,5),(2,2)]);
    // change_at(vc)
}

fn change_at(xy: Xy, cmds: Vec<DrawCmd>) -> impl Fn(&dyn Canvas) {
    cmds.iter().map(|cmd| match cmd {
        DrawCmd::Set(_) => {}
        DrawCmd::SetColor(_, _) => {}
        DrawCmd::Line(_, _, _) => {}
        DrawCmd::Circle(_, _, _) => {}
        DrawCmd::List(_) => {}
        DrawCmd::Clear(_) => {}
        DrawCmd::Fill(_) => {}
        DrawCmd::Trace(_) => {}
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

macro_rules! draw_set_points{
        ([$($a:expr),+])  => {
           {
               let v: Vec<DrawCmd>;
               $(draw_set_points!($a));+;
               v
           }
        }
        (($a:literal,$b:literal)) => {
            v.push(DrawCmd::Set(Xy($b,$a)));
        }
    }


#[cfg(test)]
#[feature(trace_macros)]
mod test {
    use crate::{DrawCmd, Xy};

    macro_rules! new_func {
       ($name:ident) => {

           fn $name(var: i32) -> i32{
               println!(" function {} -> {}",stringify!($name), var);
              var
           }
       }

    }

    macro_rules! print_result{

        ($e:expr) =>{
            println!("{} = {}",stringify!($e),$e)
        }
    }

    macro_rules! draw_set_points{
        ([$($a:expr),+])  => {
           {
               let v: Vec<DrawCmd>;
               $(draw_set_points!($a));+;
               v
           }
        }
        (($a:literal,$b:literal)) => {
            v.push(DrawCmd::Set(Xy($b,$a)));
        }
    }
    fn change_at_test() {
        let vc: Vec<DrawCmd> = vec![
            DrawCmd::Set(Xy(1, 2)),
            DrawCmd::Set(Xy(2, 2)),
            DrawCmd::Set(Xy(3, 2)),
        ];
        // let pts = draw_set_points!([(3,2),(4,5),(2,2)]);
        let pts = draw_set_points!([(2,2)]);
        // change_at(vc)
    }


    #[test]
    fn new_func_test(){

        new_func!(my_func);

        new_func!(other_func);
        my_func(3);
        other_func(2);
        print_result!(45 * 7 + 3);
        print_result!({
            let z = 4;
            let g = 3;
            z + g - 5
        } * {
            let z = 4;
            let q = 4;
            z * q
        })
    }


    #[test]
    fn nested_draw_cmds() {
        let vc: Vec<DrawCmd> = vec![
            DrawCmd::Set(Xy(1, 2)),
            DrawCmd::Set(Xy(2, 2)),
            DrawCmd::Set(Xy(3, 2)),
        ];
    }
}

