use super::image::*;

use std::mem;
struct Line {
    p1: Pt,
    p2: Pt,
    color: Px,
}

impl Line {
    fn draw(&self, image: Image<H, W>) {
        let (Pt(mut x0, mut y0), Pt(mut x1, mut y1)) = (self.p1, self.p2);

        let mut steep = false;
        if (x0 as i32 - x1 as i32).abs() < (y0 as i32 - y1 as i32).abs() {
            // if the line is steep, we transpose the image
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
            steep = true;
        }

        if x0 > x1 {
            // make it left-to-right
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }

        for x in x0..=x1 {
            let t: f64 = (x as f64 - x0 as f64) / (x1 as f64 - x0 as f64);
            let y = y0 as f64 * (1.0 - t) + y1 as f64 * t;
            if steep {}
        }
    }
}
// 'bool steep = false;
//     if (std::abs(x0-x1)<std::abs(y0-y1)) { // if the line is steep, we transpose the image
//         std::swap(x0, y0);
//         std::swap(x1, y1);
//         steep = true;
//     }
//     if (x0>x1) { // make it left−to−right
//         std::swap(x0, x1);
//         std::swap(y0, y1);
//     }
//     for (int x=x0; x<=x1; x++) {
//         float t = (x-x0)/(float)(x1-x0);
//         int y = y0*(1.-t) + y1*t;
//         if (steep) {
//             image.set(y, x, color); // if transposed, de−transpose
//         } else {
//             image.set(x, y, color);
//         }
//     }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_draw_test() {
        let l = Line {
            p1: Pt(10, 10),
            p2: Pt(20, 20),
            color: Px { r: 255, g: 0, b: 0 },
        };
        l.draw(Image::<250, 250>::new());
    }
}
