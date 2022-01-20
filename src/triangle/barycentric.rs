use crate::drawable::Drawable;
use crate::{Canvas, Point, RenderError, Triangle, Xy};

///P = A + w1(B - A) + w2(C - A)
///Px = Ax + w1(Bx - Ax) + w2(Cx - Ax)
///Py = Ay + w1(By - Ay) + w2(Cy -Ay)

///w2 = (Py - Ay - w1(By - Ay))/(Cy-Ay)

///Px = Ax + w1(Bx-Ax) + (Py-Ay-w1(By-Ay)/(Cy-Ay))(Cx-Ax)

pub struct BaryCoord<T>(T, T, T);
pub struct BaryCoordOfTri<T>(BaryCoord<T>, Triangle);

impl<T> BaryCoord<T> {
    const A_Coord: BaryCoord<f64> = BaryCoord(1., 0., 0.);
    const B_Coord: BaryCoord<f64> = BaryCoord(0., 1., 0.);
    const C_Coord: BaryCoord<f64> = BaryCoord(0., 0., 1.);
}
impl<T> BaryCoordOfTri<T> {
    fn from_xy(xy: Xy, triangle: Triangle) {}

    fn to_xy(&self) -> Xy {
        todo!()
    }

    fn is_inside_triangle() -> bool {
        true
    }
}

impl<T> Drawable for BaryCoordOfTri<T> {
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<(), RenderError> {
        Point(self.to_xy()).draw(canvas);
        Ok(())
    }
}
#[cfg(test)]
mod test {
    use crate::Vertex;

    use super::*;
    #[test]
    fn bary_new_test() {
        let t = Triangle {
            vertices: [
                Vertex::new(0.1, 0.1, 0.0),
                Vertex::new(0.2, 0.5, 0.0),
                Vertex::new(0.3, 0.1, 0.0),
            ],
        };

        let xy = Xy(1, 1);
    }
}
