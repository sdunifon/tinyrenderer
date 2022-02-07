use std::ops::Add;

use super::*;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct BoundingBox<T> {
    pub x_min: T,
    pub y_min: T,
    pub x_max: T,
    pub y_max: T,
}

impl<'a> BoundingBox<i32> {
    fn from_two_points(upper_left: Xy, lower_right: Xy) -> BoundingBox<i32> {
        debug_assert!(upper_left.x() <= lower_right.y());
        debug_assert!(upper_left.y() <= lower_right.y());

        BoundingBox {
            x_min: upper_left.x(),
            y_min: upper_left.y(),
            x_max: lower_right.x(),
            y_max: lower_right.y(),
        }
    }
}
impl<'a, T> BoundingBox<T>
where
    T: PartialEq + PartialOrd + Copy + Into<i32> + std::ops::Sub<Output = T> + std::fmt::Debug,
    u32: TryFrom<T>,
{
    fn new(x_min: T, y_min: T, x_max: T, y_max: T) -> BoundingBox<T> {
        debug_assert!(x_min <= x_max);
        debug_assert!(y_min <= y_max);
        BoundingBox {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }

    pub fn iter(&'a self) -> BoundingIterator<'a, T> {
        BoundingIterator {
            index: Xy(0, 0),
            bounding_box: self,
        }
    }

    pub fn upper_left(&self) -> Xy {
        Xy(self.x_min.into(), self.y_min.into())
    }

    pub fn lower_right(&self) -> Xy {
        Xy(self.x_max.into(), self.y_max.into())
    }

    pub fn center(&self) -> Xy {
        (self.upper_left() - self.lower_right()) / 2
    }

    pub fn width(&self) -> u32 {
       match  (self.x_max - self.x_min).try_into() {
            Ok(width) => width,
            Err(_) => panic!("BoundingBox::width: x_max - x_min is negative and can't fit in a u32")
        }
    }

    pub fn height(&self) -> u32 {
       match  (self.y_max - self.y_min).try_into() {
            Ok(height) => height,
            Err(_) => panic!("BoundingBox::height: y_max - y_min is negative and can't fit in a u32")
        }
    }

    pub fn create_image_buffer(&self) -> ImageBuffer {
        ImageBuffer::new(self.height(), self.width())
    }

}

impl<T> Add<Xy> for BoundingBox<T>
where
    T: Copy + PartialOrd + Add<Output = T> + From<i32> + std::ops::Sub<Output = T> + std::fmt::Debug,
    i32: From<T>,
    u32: TryFrom<T>,
{
    type Output = BoundingBox<T>;

    fn add(self, rhs_xy: Xy) -> Self::Output {
        // BoundingBox::from_two_points(self.upper_left() + rhs, self.lower_right() + rhs)
        BoundingBox::new(
            self.x_min + rhs_xy.x().into(),
            self.y_min + rhs_xy.y().into(),
            self.x_max + rhs_xy.x().into(),
            self.y_max + rhs_xy.y().into(),
        )
    }
}

pub struct BoundingIterator<'a, T> {
    index: Xy,
    bounding_box: &'a BoundingBox<T>,
}

///Bounding Iterator
/// maps an internal index starting in the top-left corner at 0,0 and is mapp to a point relative to the bounding box's location
/// increments in the x direction until the end of bounding box then moves down a y value and repeats
/// until it hits y_max and x_max on the bottom right corner
impl<'a> Iterator for BoundingIterator<'a, i32> {
    type Item = Xy;

    fn next(&mut self) -> Option<Self::Item> {
        let &BoundingBox {
            x_max,
            x_min,
            y_max,
            y_min,
        } = self.bounding_box;

        let relative_x_max = x_max - x_min;
        let relative_y_max = y_max - y_min;

        let incremented_index = match self.index {
            Xy(x, y) if x == relative_x_max && y == relative_y_max => None,
            Xy(x, y) if x == relative_x_max => Some(Xy(0, y + 1)),
            Xy(x, y) if x < relative_x_max => Some(Xy(x + 1, y)),
            _ => unreachable!(),
        };

        incremented_index.map(|Xy(x, y)| {
            self.index = Xy(x, y);
            Xy(x + x_min, y + y_min)
        })
    }
}

impl<T> Drawable for BoundingBox<T>
where
    T: Into<i32> + Copy,
{
    //todo get this to work for the generic case
    fn draw_on(&self, canvas: &mut dyn Canvas) -> Result<(), RenderError> {
        let Self {
            x_min,
            x_max,
            y_min,
            y_max,
        } = *self;

        let lines = [
            Line2d {
                p1: Xy(x_min.into(), y_min.into()),
                p2: Xy(x_max.into(), y_min.into()),
            },
            Line2d {
                p1: Xy(x_min.into(), y_max.into()),
                p2: Xy(x_max.into(), y_max.into()),
            },
            Line2d {
                p1: Xy(x_min.into(), y_min.into()),
                p2: Xy(x_min.into(), y_max.into()),
            },
            Line2d {
                p1: Xy(x_max.into(), y_min.into()),
                p2: Xy(x_max.into(), y_max.into()),
            },
        ];
        for line in lines {
            line.draw_on(canvas);
        }
        // self.fill() //TODO;  need to convert to i32 try to get the below From<BoundingBox<T>> for BoundingBox<i32> trait
        Ok(())
    }
}

//TODO uncomment this needed for fill
// impl<f64> From<BoundingBox<f64>> for BoundingBox<i32> {
//     fn from(bb: BoundingBox<f64>) -> Self {
//         BoundingBox::<i32> {
//             x_min: bb.x_min.into(),
//             y_min: bb.y_min.into(),
//             x_max: bb.x_max.into(),
//             y_max: bb.y_max.into(),
//         }
//     }
// }
impl<T> Colorful for BoundingBox<T> {
    fn base_color(&self) -> Color {
        color::BLUE
    }

    fn color(&self) -> Color {
        self.base_color()
    }
}

impl<T> DetectInside for BoundingBox<T>
where
    T: Into<i32> + Copy,
{
    fn includes(&self, p: Xy) -> bool {
        let Self {
            x_min,
            x_max,
            y_min,
            y_max,
        } = *self;
        let Xy(x, y) = p;

        x <= x_max.into() && y <= y_max.into() && x >= x_min.into() && y >= y_min.into()
    }
}

impl<T> Boundable<T> for BoundingBox<T>
where
    T: Copy,
{
    fn bounding_box(&self) -> BoundingBox<T> {
        self.to_owned()
    }
}

impl<T> Fillable for BoundingBox<T>
where
    i32: From<T>,
    T: Copy,
    bounding_box::BoundingBox<T>: bounding_box::Boundable<i32>,
{
    fn fill(&self, image: &mut dyn Canvas) {
        for Xy(x, y) in self.bounding_box().iter() {
            let p = Xy(x, y);
            if self.includes(Xy(x, y)) {
                image.set(p, &color::WHITE);
            }
        }
    }
}

pub trait Boundable<T> {
    fn bounding_box(&self) -> BoundingBox<T>;
}

pub trait DetectInside {
    fn includes(&self, p: Xy) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounding_box_test() {
        let t = Triangle::new([
            Vertex {
                x: 0.,
                y: 10.,
                z: 30.,
            },
            Vertex {
                x: 100.,
                y: -50.,
                z: 30.,
            },
            Vertex {
                x: 0.,
                y: 25.,
                z: 30.,
            },
        ]);
        assert_eq!(
            t.bounding_box(),
            BoundingBox {
                x_min: 0.,
                x_max: 100.,
                y_min: -50.,
                y_max: 25.
            }
        );
    }
    #[test]
    fn bound_iterator_index_test() {
        let b = BoundingBox {
            x_min: -4,
            x_max: 5,
            y_min: -4,
            y_max: 5,
        };
        let mut b_iter = b.iter();
        assert_eq!(b_iter.index, Xy(0, 0));
        assert_eq!(b_iter.next().unwrap(), Xy(-3, -4));
        assert_eq!(b_iter.index, Xy(1, 0));
        for b in 1..10 {
            dbg!(b);
            dbg!(b_iter.index);
            b_iter.next();
        }
        assert_eq!(b_iter.index, Xy(0, 1));
        assert_eq!(b_iter.next().unwrap(), Xy(-3, -3));
        assert_eq!(b_iter.index, Xy(1, 1));
        // incremented 12 so far so start there
        for i in 12..=98 {
            dbg!(i);
            dbg!(b_iter.index);
            dbg!(b_iter.next());
        }
        assert_eq!(b_iter.next().unwrap(), Xy(5, 5));
        assert_eq!(b_iter.index, Xy(9, 9));
        assert_eq!(b_iter.next(), None);
        assert_eq!(b_iter.next(), None);
    }

    #[test]
    fn test_from_two_points() {
        let b = BoundingBox::from_two_points(Xy(1, 2), Xy(3, 4));
        assert_eq!(b.x_min, 1);
        assert_eq!(b.x_max, 2);
        assert_eq!(b.y_min, 2);
        assert_eq!(b.y_max, 4);
    }

    #[test]
    fn test_upper_left_lower_right() {
        let b = BoundingBox::new(2, 2, 4, 4);
        assert_eq!(b.upper_left(), Xy(1, 2));
        assert_eq!(b.lower_right(), Xy(1, 2));
    }
    #[test]
    fn test_add() {
        let b = BoundingBox {
            x_min: 2,
            x_max: 4,
            y_min: 2,
            y_max: 4,
        };

        assert_eq!(
            b + Xy(1, 2),
            BoundingBox {
                x_min: 3,
                x_max: 5,
                y_min: 4,
                y_max: 6
            }
        );
    }

    #[test]
    fn test_image_buffer_height(){
        let b = BoundingBox::new(0, 25, 100, 100);
        assert_eq!(b.height(), 75);
    }

    #[test]
    fn test_image_buffer_width() {
        let b = BoundingBox::new(10, 25, 100, 100);
        assert_eq!(b.width(), 90);
    }

    // --float tests TODO bounding won't work with floats until we make something to convert f64->i32
    // #[test]
    // #[ignore]
    // fn test_add_with_float() {
    //     BoundingBox::new(2.5, 2.5, 4.5, 4.5); //get rid of this if we decide no to support floats in BoundingBox
    // }
    // #[test]
    // #[ignore]
    // fn test_add_with_float() {
    //     let bf = BoundingBox::new(2.5, 2.5, 4.5, 4.5); //get rid of this if we decide no to support floats in BoundingBox
    //     assert_eq!(bf + Xy(1, 2), BoundingBox::new(3.5, 3.5, 6.5, 6.5));
    // }
    // -- end float tests
}
