use super::{bounds::DetectInside, Boundable, BoundingBox, Xy};

struct Circle {
    radius: u32,
    center: Xy,
}

impl Circle {
    fn new(center: Xy, radius: u32) -> Self {
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

impl DetectInside for Circle {
    fn includes(&self, p: Xy) -> bool {
        self.center.distance_to(p) <= self.radius.into()
    }
}

#[cfg(test)]
mod tests {
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
        assert!(Circle::new(Xy(7, 8), 5).includes(Xy(9, 99)));
        assert!(!Circle::new(Xy(-3, 2), 10).includes(Xy(-3, 13)));
    }
}
