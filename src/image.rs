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

pub trait ToColorArray {
    fn to_a(&self) -> [u8; 3];
}

impl ToColorArray for Px {
    fn to_a(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

const RED: Px = Px { r: 255, g: 0, b: 0 };
const GREEN: Px = Px { r: 0, g: 255, b: 0 };
const BLUE: Px = Px {
    r: 255,
    g: 0,
    b: 255,
};
const WHITE: Px = Px {
    r: 255,
    g: 255,
    b: 255,
};
const BLACK: Px = Px { r: 0, g: 0, b: 0 };

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
            *pixel = image_lib::Rgb::<u8>(self.get(Pt(x as usize, y as usize)).to_a());
        }
        imgbuf.save(filename).unwrap();
    }

    #[inline]
    fn xy2a(x: usize, y: usize) -> usize {
        y * W + x
    }

    #[inline]
    pub fn get(&self, pt: Pt) -> Px {
        self.data[Self::xy2a(pt.0, pt.1)]
    }

    #[inline]
    pub fn set(&mut self, pt: Pt, p: Px) {
        self.data[Self::xy2a(pt.0, pt.1)] = p;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn render_test() {
        let filename = "test_render.tga";
        let img = Image::<500, 500>::new();
        if Path::new(filename).exists() {
            fs::remove_file(filename).unwrap();
        }
        img.render(filename);
        assert!(Path::new(filename).exists(), "rendered image not found");
        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn xy2a_test() {
        assert_eq!(Image::<500, 500>::xy2a(25, 25), 12525)
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
}
