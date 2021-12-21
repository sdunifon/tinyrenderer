trait Lerp {
    type Output;
    fn lerp(&self, rhs: Self, t: f64) -> Self::Output;
}

#[inline]
fn lerp(a: f64, b: f64, t: f64) -> f64 {
    ((b - a) * t) + a
}

impl Lerp for f64 {
    type Output = f64;

    fn lerp(&self, rhs: Self, t: f64) -> Self::Output {
        lerp(*self, rhs, t)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_fn_test() {
        assert_eq!(lerp(3.0, 6.0, 0.5), 4.5);
    }
    #[test]
    fn lerp_trait_test() {
        assert_eq!(4.0.lerp(8.0, 0.25), 5.0)
    }
}
