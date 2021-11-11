use core::fmt;

pub struct Image<const H: usize, const W: usize>
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    height: usize,
    width: usize,
    data: [Px; (H + 1) * (W + 1)], // buffer : ImageBuffer<Rgb<u8>, Vec<Rgb<u8::Subpixel> >
}

#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct Px {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct Pt(pub usize, pub usize);
impl fmt::Display for Pt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pt({},{})", self.0, self.1)
    }
}
pub trait ToColorArray {
    fn to_a(&self) -> [u8; 3];
}

impl ToColorArray for Px {
    fn to_a(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

pub const RED: Px = Px { r: 255, g: 0, b: 0 };
pub const GREEN: Px = Px { r: 0, g: 255, b: 0 };
pub const BLUE: Px = Px {
    r: 255,
    g: 0,
    b: 255,
};
pub const WHITE: Px = Px {
    r: 255,
    g: 255,
    b: 255,
};
pub const BLACK: Px = Px { r: 0, g: 0, b: 0 };

#[derive(Debug)]
pub struct PointOutOfBoundsError(Pt, usize, usize, usize);
impl fmt::Display for PointOutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Point: {}  -> index[{}] is outside the image bounds of {}x{}:  ",
            self.0, self.1, self.2, self.3
        )
    }
}

impl<const H: usize, const W: usize> Image<H, W>
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    pub fn new() -> Image<H, W> {
        Image {
            height: H,
            width: W,
            data: [Px::default(); (H + 1) * (W + 1)],
        }
    }
    // image_lib::ImageBuffer<image_lib::Rgb::<u8>, Container>
    pub fn render_to_buffer(&self) -> image_lib::ImageBuffer<image_lib::Rgb<u8>, Vec<u8>> {
        let mut image_buffer = image_lib::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
            let y = H - y as usize;
            *pixel = image_lib::Rgb::<u8>(self.get(Pt(x as usize, y as usize)).to_a())
        }
        image_buffer
    }

    pub fn render(&self, filename: &str) {
        let image_buffer = self.render_to_buffer();
        image_buffer.save(filename).unwrap();
    }

    #[inline]
    fn xy2i(x: usize, y: usize) -> usize {
        y * W + x
    }

    #[inline]
    fn pt2i(pt: Pt) -> usize {
        // dbg!(pt.1 * W + pt.0)
        pt.1 * W + pt.0
    }

    #[inline]
    pub fn get(&self, pt: Pt) -> Px {
        self.data[Self::pt2i(pt)]
    }

    #[inline]
    pub fn set(&mut self, pt: Pt, p: Px) {
        // dbg!(pt.0, pt.1, pt);
        self.data[Self::pt2i(pt)] = p;
    }

    pub fn point_in_bounds(&self, pt: Pt) -> Result<Pt, PointOutOfBoundsError> {
        // if pt.0 > W || pt.1 > H || Self::pt2i(pt) > self.data.len() {
        if pt.0 > W || pt.1 > H {
            return Err(PointOutOfBoundsError(pt, Self::pt2i(pt), H, W));
        }
        Ok(pt)
    }
    // pub fn draw( drawer:( img ) -> () ){
    //     drawer(self);
    // }

    pub fn draw(&mut self, d: &dyn Drawable<H, W>) {
        d.draw(self)
    }
}

pub trait Drawable<const H: usize, const W: usize>
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    fn draw(&self, image: &mut Image<H, W>);
    fn draw2(&self, image: &mut Image<H, W>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::assert_file_creation;

    #[test]
    fn render_test() {
        assert_file_creation("test_render.tga", |filename: &str| {
            let img = Image::<500, 500>::new();
            img.render(filename);
        });
    }

    #[test]
    fn xy2a_test() {
        assert_eq!(Image::<500, 500>::xy2i(25, 25), 12525)
    }

    #[test]
    fn set_get_test() {
        let mut img = Image::<500, 500>::new();

        assert_eq!(img.get(Pt(250, 250)), Px { r: 0, g: 0, b: 0 });
        img.set(Pt(250, 250), Px { r: 0, g: 255, b: 0 });
        assert_eq!(img.get(Pt(250, 250)), Px { r: 0, g: 255, b: 0 });
        assert_eq!(img.get(Pt(251, 250)), Px { r: 0, g: 0, b: 0 });
        img.set(Pt(250, 250), Px { r: 0, g: 1, b: 0 });
        assert_eq!(img.get(Pt(250, 250)), Px { r: 0, g: 1, b: 0 });
    }
    #[test]
    fn index_conversion_test() {
        assert_eq!(
            Image::<500, 500>::xy2i(68, 345),
            Image::<500, 500>::pt2i(Pt(68, 345))
        );
    }

    #[test]
    #[ignore]
    fn get_set_boundries_test() {
        // if we are able to get this test working we can remvoe all the + 1 to the image size for the
        // where boundry l and switch back from [u8; (H + 1) * (W + 1)]: Sized, to [u8,H * W]
        assert_eq!(Image::<500, 500>::pt2i(Pt(500, 500)), 250000)
    }
}
