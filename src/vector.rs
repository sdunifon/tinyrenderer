use num_traits::float::Float;
use std::ops::{self, Add, Mul};

#[derive(Debug, PartialEq, Clone, Copy)] // try removing copy here and the impl where statement of div.. and try to get it unit working without a copy
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3<T>
where
    T: Mul<Output = T> + Add<Output = T> + ops::Div<Output = T> + num_traits::float::Float,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Return the length / Magnitude / norm of the vector
    pub fn norm(&self) -> T {
        let Self { x, y, z } = *self;
        ((x * x) + (y * y) + (z * z)).sqrt()
    }

    pub fn unit(&self) -> Vector3<T> {
        // Vector3::new(1., 2., 3.) / 4.
        // let a = self / 4.; //self.norm()
        // let b = Vector3::new(1., 2., 3.) / 4.; //self.norm()
        // *self.clone()
        // *self.clone() / self.x //self.norm()
        let new_vec = self.clone();
        new_vec / self.norm()
    }

    fn cross(&self, rhs: &Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: (self.y * rhs.z) - (self.z * rhs.y),
            y: (self.z * rhs.x) - (self.x * rhs.z),
            z: (self.x * rhs.y) - (self.y * rhs.x),
        }
    }
}

// impl<T> ops::Div<T> for Vector3<T>
// where
//     T: ops::Div<T, Output = T>,
// {
//     type Output = Vector3<T>;

//     fn div(self, rhs: T) -> Self::Output {
//         Vector3 {
//             x: self.x / rhs,
//             y: self.y / rhs,
//             z: self.z / rhs,
//         }
//     }
// }

// impl<T> ops::Div<f64> for Vector3<T>
// where
//     T: ops::Div<f64, Output = T>,
// {
//     type Output = Vector3<T>;

//     fn div(self, rhs: f64) -> Self::Output {
//         Vector3 {
//             x: self.x / rhs,
//             y: self.y / rhs,
//             z: self.z / rhs,
//         }
//     }
// }
impl<T> Vector3<T> {}
impl<T> ops::Div<T> for Vector3<T>
where
    T: ops::Div<T, Output = T> + Copy, // do we need copy
{
    type Output = Vector3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use all_asserts::assert_near;
    use criterion::black_box;
    use test::Bencher;

    fn assert_vector_eq(v1: Vector3<f64>, v2: Vector3<f64>) {
        assert_near!(v1.x, v2.x, 0.001);
        assert_near!(v1.y, v2.y, 0.001);
        assert_near!(v1.z, v2.z, 0.001);
    }

    #[test]
    fn norm() {
        assert_near!(Vector3::new(1., 2., 3.).norm(), 3.741, 0.001);
        assert_near!(Vector3::new(-2., 2., 2.).norm(), 3.464, 0.001);
    }

    #[test]
    fn div_test() {
        assert_eq!(Vector3::new(4., 4., 4.) / 2., Vector3::new(2., 2., 2.));
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_name() {}
        }
        #[test]
        fn unit_test() {
            assert_vector_eq(
                Vector3::new(2., 2., 2.).unit(),
                Vector3::new(0.577, 0.577, 0.577),
            );
            assert_vector_eq(
                Vector3::new(3., 1., 0.).unit(),
                Vector3::new(0.9487, 0.3162, 0.0),
            );
            assert_vector_eq(
                Vector3::new(2., -5.7, 6.).unit(),
                Vector3::new(0.234, -0.669, 0.704),
            );
        }
    }
    /// short hand for creating a vector with x y z input
    macro_rules! v {
        ($a:expr,$b:expr,$c:expr) => {
            Vector3::new($a as f64, $b as f64, $c as f64)  //TODO take out automatic conversion to f64.. currently haveing some problems with erros sayingit could be f32
        };
    }

    #[test]
    fn macro_test() {
        assert_eq!(Vector3::new(2., 4., -5.), v!(2., 4., -5.))
    }

    #[test]
    fn cross_test() {
        assert_eq!(
            Vector3::new(1., 2., 0.).cross(&Vector3::new(3., 1., 0.)),
            v!(0., 0., -5.)
        );
        assert_vector_eq(
            v!(2.6, 4.0, -0.4).cross(&v!(-8., 0., 0.6)),
            v!(2.4, 1.64, 32.),
        )
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
