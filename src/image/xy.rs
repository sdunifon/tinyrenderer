use crate::{Image, Vertex};
use std::{
    fmt,
    ops::{Add, Div, Sub},
};

#[derive(PartialEq, Default, Clone, Copy, PartialOrd)]
pub struct Xy(pub i32, pub i32);

impl fmt::Display for Xy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Xy({},{})", self.0, self.1)
    }
}

impl fmt::Debug for Xy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Xy({},{})", self.0, self.1)
    }
}

// impl From<&Vertex> for Xy {
impl Xy {
    pub(crate) fn pt_on_image(vertex: &Vertex, image: &Image) -> Self {
        let resized_vertex = *vertex * (image.height.max(image.width) / 2) as f64;
        let center_adjust_x: i32 = (image.width as i32) / 2;
        let center_adjust_y: i32 = (image.height as i32) / 2;
        Xy(
            (resized_vertex.x.round() as i32 + center_adjust_x)
                .try_into()
                .unwrap(),
            (resized_vertex.y.round() as i32 + center_adjust_y)
                .try_into()
                .unwrap(),
        )
    }

    pub fn distance_to(&self, p: Xy) -> f64 {
        (((p.0 - self.0).pow(2) + (p.1 - self.1).pow(2)) as f64).sqrt()
    }

    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
}

impl Add for Xy {
    type Output = Xy;

    fn add(self, rhs: Self) -> Self::Output {
        Xy(self.0 + rhs.0, self.1 + rhs.0)
    }
}
impl Sub for Xy {
    type Output = Xy;

    fn sub(self, rhs: Self) -> Self::Output {
        Xy(self.0 - rhs.0, self.1 - rhs.0)
    }
}

impl Div<Xy> for Xy {
    type Output = Xy;

    fn div(self, rhs: Self) -> Self::Output {
        Xy(self.0 / rhs.0, self.1 / rhs.0)
    }
}
impl<T> Div<T> for Xy
where
    T: Into<i32>,
{
    type Output = Xy;

    fn div(self, rhs: T) -> Self::Output {
        let rhs_i32: i32 = rhs.into();
        Xy(self.0 / rhs_i32, self.1 / rhs_i32)
    }
}
#[cfg(test)]
mod tests {
    use all_asserts::assert_near;

    use super::*;

    #[test]
    fn distance_to_test() {
        assert_near!(Xy(2, 5).distance_to(Xy(13, 7)), 11.180, 0.01);
        assert_near!(Xy(-6, 3).distance_to(Xy(-2, -4)), 8.06, 0.01);
    }
}
