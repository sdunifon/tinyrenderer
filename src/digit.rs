use crate::canvas::Canvas;
use crate::drawable::Drawable;
use crate::{Boundable, BoundingBox, Colorful, DrawBoundable, RenderError, ToImageBuffer, Xy};

pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Decimal,
    RightParen,
    LeftParen,
    Negative,
    Comma,
}

impl Digit {
    fn px_data(&self) -> [&'static str; 8] {
        match self {
            Digit::Zero => self.zero_px(),
            Digit::One => self.one_px(),
            Digit::Two => self.two_px(),
            Digit::Three => self.three_px(),
            Digit::Four => self.four_px(),
            Digit::Five => self.five_px(),
            Digit::Six => self.six_px(),
            Digit::Seven => self.seven_px(),
            Digit::Eight => self.eight_px(),
            Digit::Nine => self.nine_px(),
            Digit::Decimal => self.decimal_px(),
            Digit::RightParen => todo!(),
            Digit::LeftParen => todo!(),
            Digit::Negative => todo!(),
            Digit::Comma => todo!(),
        }
    }

    #[rustfmt::skip]
    const fn zero_px(&self) -> [&'static str; 8] {
        [
            "00111110",
            "01000010",
            "01000010",
            "01000010",
            "01000010",
            "01000010",
            "01000010",
            "00111100",
        ]
    }

    #[rustfmt::skip]
    const fn one_px(&self) -> [&'static str; 8] {
        [
            "00011000",
            "00101000",
            "01001000",
            "00001000",
            "00001000",
            "00001000",
            "00001000",
            "00111110",
        ]
    }

    #[rustfmt::skip]
    const fn two_px(&self) -> [&'static str; 8] {
        [
            "00111100",
            "01000010",
            "00000010",
            "00000100",
            "00001000",
            "00010000",
            "00100000",
            "01111110",
        ]
    }

    const fn blah() -> [[u8; 8]; 8] {
        let a: [[u8; 8]; 8] = [
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ];
        a
    }
    #[rustfmt::skip]
    const fn three_px(&self) -> [&'static str;8]{
        [
            "00111100",
            "01000010",
            "00000010",
            "00011100",
            "00000010",
            "00000010",
            "01000010",
            "00111100",
        ]
    }
    #[rustfmt::skip]
    const fn four_px(&self) -> [&'static str;8]{
        [
            "01000100",
            "01000100",
            "01000100",
            "01000100",
            "01111110",
            "00000100",
            "00000100",
            "00000100",
        ]
    }
    #[rustfmt::skip]
    const fn five_px(&self) -> [&'static str; 8] {
        [
            "00111110",
            "01000000",
            "01000000",
            "01111110",
            "00000001",
            "00000001",
            "00000001",
            "01111110",
        ]
    }
    #[rustfmt::skip]
    const fn six_px(&self) -> [&'static str; 8] {
        [
            "00111110",
            "10000000",
            "10000000",
            "10000000",
            "11111111",
            "10000001",
            "10000001",
            "01111110",
        ]
    }
    #[rustfmt::skip]
    const fn seven_px(&self) -> [&'static str; 8] {
        [
            "00111100",
            "00000010",
            "00000010",
            "00000100",
            "00001000",
            "00010000",
            "00010000",
            "00000000",
        ]
    }
    #[rustfmt::skip]
    const fn eight_px(&self) -> [&'static str; 8] {
        [
            "00111110",
            "10000001",
            "10000001",
            "10000001",
            "11111111",
            "10000001",
            "10000001",
            "01111110",
        ]
    }
    #[rustfmt::skip]
    const fn nine_px(&self) -> [&'static str; 8] {
        [
            "00111110",
            "10000001",
            "10000001",
            "01111111",
            "00000001",
            "00000011",
            "00111100",
            "00000000",
        ]
    }
    #[rustfmt::skip]
    const fn decimal_px(&self) -> [&'static str; 8] {
        [
            "00000000",
            "00000000",
            "00000000",
            "00000000",
            "00000000",
            "00111000",
            "00111000",
            "000l1000",
        ]
    }
    #[rustfmt::skip]
    const fn left_paren(&self) -> [&'static str; 8] {
        [
            "00000111",
            "00111000",
            "00100000",
            "01000000",
            "01000000",
            "00100000",
            "00111000",
            "00000111",
        ]
    }

    #[rustfmt::skip]
    const fn right_paren_px(&self) -> [&'static str; 8] {
        [
            "11100000",
            "00011100",
            "00000100",
            "00000010",
            "00000010",
            "00000100",
            "00011100",
            "11100000",
        ]
    }
    #[rustfmt::skip]
    const fn comma_px(&self) -> [&'static str; 8] {
        [
            "00000000",
            "00000000",
            "00000000",
            "00000000",
            "00000000",
            "00000000",
            "00110000",
            "00010000",
        ]
    }
}
impl Colorful for Digit {
    fn base_color(&self) -> crate::Color {
        crate::color::BLUE
    }

    fn color(&self) -> crate::Color {
        self.base_color()
    }
}
impl Drawable for Digit {
    fn draw_on(&self, canvas: &mut dyn Canvas) -> Result<(), RenderError> {
        let (x, y) = (25i32, 25i32);
        // self.px_data().iter().enumerate().map(|(row_vec, row_num)| {
        for (row_num, row_vec) in self.px_data().iter().rev().enumerate() {
            // row_vec.iter().enumerate().map(|(px, col_num)| {
            for (col_num, px) in row_vec.chars().enumerate() {
                if px == '1' {
                    let p = Xy(x + col_num as i32, y + row_num as i32);
                    canvas.set(p, &self.color());
                }
            }
        }
        Ok(())
    }
}

impl ToImageBuffer for Digit {
    fn to_image_buffer(self) -> crate::ImageBuffer {
        todo!()
    }
}

impl<T> Boundable<T> for Digit
where
    T: From<i32>,
{
    fn bounding_box(&self) -> crate::BoundingBox<T> {
        BoundingBox {
            x_min: 0.into(),
            y_min: 0.into(),
            x_max: 8.into(),
            y_max: 8.into(),
        }
    }
}

impl<T: From<i32>> DrawBoundable<T> for Digit {}

impl TryFrom<char> for Digit {
    type Error = RenderError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '1' => Ok(Digit::One),
            '2' => Ok(Digit::Two),
            '3' => Ok(Digit::Three),
            '4' => Ok(Digit::Four),
            '5' => Ok(Digit::Five),
            '6' => Ok(Digit::Six),
            '7' => Ok(Digit::Seven),
            '8' => Ok(Digit::Eight),
            '9' => Ok(Digit::Nine),
            '.' => Ok(Digit::Decimal),
            '(' => Ok(Digit::RightParen),
            ')' => Ok(Digit::LeftParen),
             _ => Err(RenderError("Digit parse error! Character {c} is not a displayable digit character. Use only characters 0-9.()".to_string())),
        }
    }
}

impl<T: Numeric + ToString> Drawable for T {
    fn draw_on(&self, canvas: &mut dyn Canvas) -> Result<(), RenderError> {
        let string = self.to_string();

        let digits = string
            .chars()
            .map(|c| Digit::try_from(c))
            .collect::<Result<Vec<Digit>, RenderError>>()?;
        Ok(())
    }
}

//display x,y points
pub trait Numeric {}

macro_rules! impl_trait_for {
    ($type_name:ty) => {
        //first
        impl Numeric for $type_name {}
    };
    ($type_name1:ty,$($type_name2:ty),* $(,)*) => {
        impl Numeric for $type_name1 {}
        $(impl_trait_for!($type_name2);)*
    };
}
impl_trait_for!(f64, f32, i64, i32, i16, i8, isize, u64, u32, u16, u8, usize);
// impl Numeric for f64 {}
// impl Numeric for f32 {}
// impl Numeric for i64 {}
// impl Numeric for i32 {}
// impl Numeric for i16 {}
// impl Numeric for i8 {}
// impl Numeric for isize {}
// impl Numeric for u64 {}
// impl Numeric for u32 {}
// impl Numeric for u16 {}
// impl Numeric for u8 {}
// impl Numeric for usize {}

#[cfg(test)]
mod tests {
    #[test]
    fn impl_trait_for_test() {
        pub trait Numeric {}
        impl_trait_for!(u8);
    }

    #[test]
    fn impl_trait_for_test2() {
        pub trait Numeric {}
        impl_trait_for!(u8, u16);
    }

    #[test]
    fn impl_trait_for_test3() {
        pub trait Numeric {}
        impl_trait_for!(u8, u16,);
    }

    #[test]
    fn impl_trait_for_test4() {
        pub trait Numeric {}
        impl_trait_for!(u8, u16, u32, i32);
    }
}
