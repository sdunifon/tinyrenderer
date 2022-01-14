#![allow(dead_code)] //todo turn back on once more code is established
#![allow(unused_variables)]
use super::*;

#[derive(Default, Debug, PartialEq)]
pub struct BoundingBox<T> {
    pub x_min: T,
    pub y_min: T,
    pub x_max: T,
    pub y_max: T,
}

impl<T> BoundingBox<T>
where
    T: PartialEq + PartialOrd,
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
    fn iter() -> BoundingIterator<T> {
        BoundingIterator { index: Xy() }
    }
}

struct BoundingIterator<T> {
    index: Xy,
    bounding_box: &BoundingBox<T>,
}

impl Iterator for BoundingIterator<i32> {
    type Item = Xy;

    fn next(&mut self) -> Option<Self::Item> {
        let BoundingBox {
            x_max,
            x_min,
            y_max,
            y_min,
        } = self.bounding_box;
        match self.index {
            Xy(x, y) if x == x_max && y == y_max => None,
            Xy(x, y) if x == x_max => Some(Xy(0, y + 1)),
            Xy(x, y) if x < x_max => Some(Xy(x + 1, y)),
            _ => unreachable!(),
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
    fn bound_iterator_test() {
        assert!(true)
    }
}
