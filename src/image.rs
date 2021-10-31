struct Image<const H: usize, const W: usize>
where
    [u8; H * W]: Sized,
{
    height: usize,
    width: usize,
    data: [u8; H * W], // buffer : ImageBuffer<Rgb<u8>, Vec<Rgb<u8::Subpixel> >
}

impl<const H: usize, const W: usize> Image<H, W>
where
    [u8; H * W]: Sized,
{
    fn new() -> Image<H, W> {
        Image {
            height: H,
            width: W,
            data: [0; H * W],
        }
    }

    fn render(&self, filename: &str) {
        let mut imgbuf = image::ImageBuffer::new(self.width as u32, self.height as u32);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let r = (0.3 * x as f32) as u8;
            let g = (0.3 * y as f32) as u8;
            *pixel = image::Rgb([r, g, 0]);
        }
        imgbuf.save(filename).unwrap();
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
        let img = Image::<1000, 1000>::new();
        if (Path::new(filename).exists()) {
            fs::remove_file(filename).unwrap();
        }
        img.render(filename);
        assert!(Path::new(filename).exists(), "rendered image not found");
    }
}
