#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub const RED: Color = Color { r: 255, g: 0, b: 0 };
pub const GREEN: Color = Color { r: 0, g: 255, b: 0 };
pub const BLUE: Color = Color { r: 0, g: 0, b: 255 };
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};
pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
pub trait Colorful {
    // #[inline]
    fn base_color(&self) -> Color {
        WHITE
    }
    // #[inline]
    fn color(&self) -> Color {
        self.base_color()
    }
}

pub trait ToColorArray {
    fn to_color_ary(&self) -> [u8; 3];
}

impl ToColorArray for Color {
    fn to_color_ary(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

// fn to_color_ary(&self) {
//     self.color().to_color_ary()
// }
