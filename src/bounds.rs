use super::*;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct BoundingBox<T> {
    pub x_min: T,
    pub y_min: T,
    pub x_max: T,
    pub y_max: T,
}

impl<'a, T> BoundingBox<T>
where
    T: PartialEq + PartialOrd + Copy,
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
    fn draw(&self, canvas: &mut dyn Canvas) -> Result<(), RenderError> {
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
            line.draw(canvas);
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
    bounds::BoundingBox<T>: bounds::Boundable<i32>,
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
}
