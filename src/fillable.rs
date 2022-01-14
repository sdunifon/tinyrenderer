use super::*;

// [u8; (H + 1) * (W + 1)]: Sized + Drawable<H, W>,
pub trait Fillable: Colorful + DetectInside + Boundable<i32> {
    fn fill(&self, image: &mut dyn Canvas) {
        for (x, y) in self.bounding_box().iter() {
            let p = Xy(x, y);
            if self.includes(Xy(x, y)) {
                image.set(p, &WHITE);
            }
        }
    }
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

fn fill_bounds() {
    //bounds: BoundingBox, canvas: &dyn Drawable) -> () {
    todo!();
    // for (x, y) in bounds {}
}
