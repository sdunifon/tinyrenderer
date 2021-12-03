use super::*;

// [u8; (H + 1) * (W + 1)]: Sized + Drawable<H, W>,
pub trait Fillable: Colorful {
    fn fill(&self, image: &mut dyn DrawTools);
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
