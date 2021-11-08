use core::fmt;

// use image_lib::Primitive;

pub struct Image<const H: usize, const W: usize>
where
    [u8; H * W]: Sized,
{
    height: usize,
    width: usize,
    data: [Px; H * W], // buffer : ImageBuffer<Rgb<u8>, Vec<Rgb<u8::Subpixel> >
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
    [u8; H * W]: Sized,
{
    pub fn new() -> Image<H, W> {
        Image {
            height: H,
            width: W,
            data: [Px::default(); H * W],
        }
    }

    pub fn render(&self, filename: &str) {
        let mut imgbuf = image_lib::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            // let [r, g, b] = self.get(Pt(x as usize, y as usize)).to_a();
            // let p: [u8; 3] = image_lib::Rgb([r, g, b]);
            // *pixel = p; //image_lib::Rgb([r, g, b]);
            // println!("--");
            // let adjusted_x: usize = ((x) * (W as u32 / 2)).try_into().unwrap();
            // let adjusted_y: usize = ((y) * (H as u32 / 2)).try_into().unwrap();
            // dbg!((x, adjusted_x));
            // dbg!((y, adjusted_y));
            // let pt = self.point_in_bounds(Pt(adjusted_x, adjusted_y));
            // match pt {
            //     Ok(pt) => *pixel = image_lib::Rgb::<u8>(self.get(pt).to_a()),
            //     Err(e) => println!("Render Error: {}", e),
            // }
            *pixel = image_lib::Rgb::<u8>(self.get(Pt(x as usize, y as usize)).to_a())
        }
        imgbuf.save(filename).unwrap();
    }

    #[inline]
    fn xy2i(x: usize, y: usize) -> usize {
        y * W + x
    }

    #[inline]
    fn pt2i(pt: Pt) -> usize {
        pt.1 * W + pt.0
    }

    #[inline]
    pub fn get(&self, pt: Pt) -> Px {
        self.data[Self::pt2i(pt)]
    }

    #[inline]
    pub fn set(&mut self, pt: Pt, p: Px) {
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
    [u8; H * W]: Sized,
{
    fn draw(&self, image: &mut Image<H, W>);
    fn draw2(&self, image: &mut Image<H, W>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helper::assert_file_creation;
    use std::fs;
    use std::path::Path;

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
        assert_eq!(Image::xy2i(68, 345), Image::pt2i(Pt(68, 345)));
    }
}
