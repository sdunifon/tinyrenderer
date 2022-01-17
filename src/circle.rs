use super::{bounds::DetectInside, Boundable, BoundingBox, Xy};
use crate::color::{self, Color, Colorful};
use crate::{Drawable, Fillable};

pub struct Circle {
    radius: u32,
    center: Xy,
}

impl Circle {
    pub fn new(center: Xy, radius: u32) -> Self {
        Self { center, radius }
    }
}

impl Boundable<i32> for Circle {
    fn bounding_box(&self) -> BoundingBox<i32> {
        BoundingBox {
            x_min: self.center.0 - self.radius as i32,
            x_max: self.center.0 + self.radius as i32,
            y_min: self.center.1 - self.radius as i32,
            y_max: self.center.1 + self.radius as i32,
        }
    }
}
impl Colorful for Circle {
    fn base_color(&self) -> Color {
        color::GREEN
    }

    fn color(&self) -> Color {
        self.base_color()
    }
}

impl Fillable for Circle {}
impl Drawable for Circle {
    fn draw(&self, drawer: &mut dyn crate::Canvas) {
        //todo implement https://www.geeksforgeeks.org/bresenhams-circle-drawing-algorithm/
        //just filling for now
        self.fill(drawer)
    }
}
impl DetectInside for Circle {
    fn includes(&self, p: Xy) -> bool {
        self.center.distance_to(p) <= self.radius.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::WHITE, Canvas, Render};

    use super::*;

    #[test]
    fn bounding_test() {
        let c = Circle::new(Xy(5, 20), 25);
        assert_eq!(
            c.bounding_box(),
            BoundingBox {
                x_min: -20,
                x_max: 30,
                y_min: -5,
                y_max: 45,
            }
        );
    }
    #[test]
    fn inside_test() {
        assert!(Circle::new(Xy(7, 8), 5).includes(Xy(9, 9)));
        assert!(!Circle::new(Xy(-3, 2), 10).includes(Xy(-3, 13)));
    }
    #[test]
    fn fill_test() {
        let mut renderer = Render::default();
        let circle = Circle::new(Xy(1, 5), 5);
        circle.fill(&mut renderer.image);
        assert_eq!(renderer.image.get(Xy(2, 6)), &WHITE)
    }
}
