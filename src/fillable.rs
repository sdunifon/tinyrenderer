use super::*;

// [u8; (H + 1) * (W + 1)]: Sized + Drawable<H, W>,
pub trait Fillable<const H: usize, const W: usize>: Colorful
where
    [u8; (H + 1) * (W + 1)]: Sized,
{
    fn fill(&self, image: &mut dyn Drawer<H, W>);
    //TODO --better fill technique --!!!! https://github.com/ssloy/tinyrenderer/wiki/Lesson-2:-Triangle-rasterization-and-back-face-culling
    // triangle(vec2 points[3]) {
    //     vec2 bbox[2] = find_bounding_box(points);
    //     for (each pixel in the bounding box) {
    //         if (inside(points, pixel)) {
    //             put_pixel(pixel);
    //         }
    //     }
    // }
    //
}
