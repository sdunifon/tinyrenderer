use crate::color::Color;
use rand::random;
use std::ops::{Add, Mul};

#[macro_export]
macro_rules! swap_vars {
    ($e:expr, $a:ident, $b:ident) => {
        let ($a, $b) = if $e { ($b, $a) } else { ($a, $b) };
    };
}

pub fn random_color() -> Color {
    Color {
        r: random(),
        g: random(),
        b: random(),
    }
}

pub fn random_grayscale() -> Color {
    let gray_value = random();

    Color {
        r: gray_value,
        g: gray_value,
        b: gray_value,
    }
}

pub fn lerp<T>(a: T, b: T, percent: i32) -> T
where
    T: Add<T, Output = T>,
    T: Mul<i32, Output = T>,
    T: Into<i32>,
    T: From<i32>,
{
    // let range = [a..b];
    // let size = range.len();
    // range[]
    // range[(size as f64 * t).round() as usize]
    a + ((b.into() * percent) / 100).into()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn swap_vars() {
        let a = 1;
        let b = 2;

        swap_vars!(1 > 2, a, b);
        assert_eq!(a, 1);
        assert_eq!(b, 2);

        let c = 1;
        let d = 2;

        swap_vars!(3 > 2, c, d);
        assert_eq!(c, 2);
        assert_eq!(d, 1);
    }
    #[test]
    fn lerp_test() {
        assert_eq!(lerp(0, 10, 0), 0);
        assert_eq!(lerp(0, 10, 100), 10);
        assert_eq!(lerp(0, 5, 100), 5);
        assert_eq!(lerp(5, 10, 0), 5);
        assert_eq!(lerp(0, 10, 50), 5);
        assert_eq!(lerp(6, 11, 27), 8);
    }
}
