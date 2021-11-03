use super::*;

use std::mem;
struct Line {
    p1: Pt,
    p2: Pt,
    color: Px,
}

impl Line {}

impl<const H: usize, const W: usize> Drawable<H, W> for Line
where
    [u8; H * W]: Sized,
{
    fn draw(&self, canvas: &mut Image<H, W>) {
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

        let dx: i64 = x1 as i64 - x0 as i64;
        let dy: i64 = y1 as i64 - y0 as i64;
        let mut derror: f64 = (dy as f64 / dx as f64).abs();
        let mut error: f64 = 0.0;
        let mut y: i64 = y0 as i64;

        for x in x0..=x1 {
            if steep {
                canvas.set(Pt(y as usize, x as usize), self.color);
            } else {
                canvas.set(Pt(x as usize, y as usize), self.color);
            }

            error += derror;
            if error > 0.5 {
                y += if y1 > y0 { 1 } else { -1 };
                error -= 1.;
            }
        }
    }
    fn draw2(&self, canvas: &mut Image<H, W>) {
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
        let dx: i64 = x1 as i64 - x0 as i64;
        let dy: i64 = y1 as i64 - y0 as i64;
        let derror2: i64 = (dy).abs() * 2;
        let mut error2: i64 = 0;
        let mut y: i64 = y0 as i64;

        for x in x0..=x1 {
            if steep {
                canvas.set(Pt(y as usize, x as usize), self.color);
            } else {
                canvas.set(Pt(x as usize, y as usize), self.color);
            }
            error2 += derror2;
            if error2 > dx {
                y += if y1 > y0 { 1 } else { -1 };
                error2 -= dx * 2;
            }
        }
    }
}

mod tests {
    use super::*;

    use crate::test_helper::assert_file_creation;

    #[test]
    fn line_draw_test() {
        assert_file_creation("line_draw_test.tga", |filename: &str| {
            let l = Line {
                p1: Pt(10, 10),
                p2: Pt(20, 20),
                color: Px { r: 255, g: 0, b: 0 },
            };
            let mut i = Image::<250, 250>::new();
            i.draw(&l);
            i.render(filename);
        })
    }

    #[test]
    fn multiple_line_draw_test() {
        assert_file_creation("multiple_line_draw_test.tga", |filename: &str| {
            let l0 = Line {
                p1: Pt(10, 10),
                p2: Pt(20, 20),
                color: Px { r: 255, g: 0, b: 0 },
            };
            let l1 = Line {
                p1: Pt(5, 5),
                p2: Pt(70, 7),
                color: Px { r: 0, g: 255, b: 0 },
            };
            let l2 = Line {
                p1: Pt(20, 20),
                p2: Pt(150, 2),
                color: Px { r: 0, g: 0, b: 255 },
            };
            let mut i = Image::<250, 250>::new();
            i.draw(&l0);
            i.draw(&l1);
            i.draw(&l2);
            i.render(filename);
        })
    }
}
