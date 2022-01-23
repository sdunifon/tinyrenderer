use crate::{Boundable, Canvas, Drawable, Xy};

pub struct DrawAt(Xy, dyn DrawBoundable<i32>);

trait DrawBoundable<T>: Drawable + Boundable<T> {}
impl DrawAt {
    fn translate_to(&self, point: Xy) -> Xy {
        self.0 + point
    }
    fn drawable(&self) -> &dyn DrawBoundable<i32> {
        &self.1
    }
}

impl Boundable<i32> for DrawAt {
    fn bounding_box(&self) -> crate::BoundingBox<i32> {
        self.drawable().bounding_box()
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
impl DrawAt {}
