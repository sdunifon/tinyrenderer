pub struct Image<const H: usize, const W: usize, PixelType = Px<u8>>
where
    [u8; H * W]: Sized,
{
    height: usize,
    width: usize,
    data: [PixelType; H * W], // buffer : ImageBuffer<Rgb<u8>, Vec<Rgb<u8::Subpixel> >
}

#[derive(PartialEq, Debug, Default, Clone, Copy)]
pub struct Px<T = u8> {
    pub r: T,
    pub g: T,
    pub b: T,
}

pub struct Pt(pub usize, pub usize);

impl<const H: usize, const W: usize, PixelType> Image<H, W, PixelType>
where
    [u8; H * W]: Sized,
    PixelType: PartialEq + Copy + Default,
{
    pub fn new() -> Image<H, W> {
        Image {
            height: H,
            width: W,
            data: [Px::default(); H * W],
        }
    }

    pub fn render(&self, filename: &str) {
        let mut imgbuf = image::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let r = (0.3 * x as f32) as u8;
            let g = (0.3 * y as f32) as u8;
            *pixel = image::Rgb([r, g, 0]);
        }
        imgbuf.save(filename).unwrap();
    }

    #[inline]
    fn xy2a(x: usize, y: usize) -> usize {
        y * W + x
    }

    #[inline]
    pub fn get(&self, pt: Pt) -> PixelType {
        self.data[Self::xy2a(pt.0, pt.1)]
    }

    #[inline]
    pub fn set(&mut self, pt: Pt, p: PixelType) {
        self.data[Self::xy2a(pt.0, pt.1)] = p;
    }
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
