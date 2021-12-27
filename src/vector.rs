use num_traits::float::Float;
use std::ops::{Add, Mul};

pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3<T>
where
    T: Mul<Output = T> + Add<Output = T> + num_traits::float::Float,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn norm(&self) -> T {
        let Self { x, y, z } = *self;
        ((x * x) + (y * y) + (z * z)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use criterion::black_box;
    use test::Bencher;
    #[test]
    fn norm() {
        assert_eq!(Vector3::new(1., 2., 3.).norm(), 3.74);
        assert_eq!(Vector3::new(-2., 2., 2.).norm(), 4.);
    }
    #[bench]
    fn power_bench(b: &mut Bencher) {
        let (x, y, z) = (1f64, 2f64, 3f64);
        b.iter(move || {
            for i in 1..100000 {
                black_box((x * x) + (y * y) + (z * z));
            }
        });
    }

    #[bench]
    fn power_bench2(b: &mut Bencher) {
        let (x, y, z) = (1f64, 2f64, 3f64);
        b.iter(|| {
            for i in 1..100000 {
                black_box(x.powf(2.) + y.powf(2.) + z.powf(2.));
            }
        })
    }
}

// impl<T> Vector3<T> {
//     pub fn new(data: [T; 3]) -> Self {
//         Self { data }
//     }

//     pub fn unit(&self) {}

//     pub fn norm(&self) -> T {

//     }
// }
