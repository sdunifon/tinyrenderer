use super::*;

use std::mem;
struct Line {
    p1: Pt,
    p2: Pt,
    color: Px,
}

impl Line {}

impl<const H: usize, const W: usize, PixelType> Drawable<H, W, PixelType> for Line
where
    [u8; H * W]: Sized,
{
    fn draw(&self, canvas: &Image<H, W, PixelType>)
    where
        [u8; H * W]: Sized,
    {
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
            if steep {
                canvas.set(Pt(y as usize, x as usize), Px { r: 255, g: 0, b: 0 })
            //if transposed (steep==true) de-transpose
            } else {
                canvas.set(Pt(x as usize, y as usize), Px { r: 255, g: 0, b: 0 })
            }
        }
    }
}
// void line(int x0, int y0, int x1, int y1, TGAImage &image, TGAColor color) {
//     bool steep = false;
//     if (std::abs(x0-x1)<std::abs(y0-y1)) {
//         std::swap(x0, y0);
//         std::swap(x1, y1);
//         steep = true;
//     }
//     if (x0>x1) {
//         std::swap(x0, x1);
//         std::swap(y0, y1);
//     }
//     int dx = x1-x0;
//     int dy = y1-y0;
//     int derror2 = std::abs(dy)*2;
//     int error2 = 0;
//     int y = y0;
//     for (int x=x0; x<=x1; x++) {
//         if (steep) {
//             image.set(y, x, color);
//         } else {
//             image.set(x, y, color);
//         }
//         error2 += derror2;
//         if (error2 > dx) {
//             y += (y1>y0?1:-1);
//             error2 -= dx*2;
//         }
//     }
// }
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
        let i = Image::<250, 250>::new();
        i.draw(l);
    }
}
