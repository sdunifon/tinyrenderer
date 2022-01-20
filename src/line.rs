use super::*;

use crate::drawable::Drawable;
use std::mem;

pub struct Line {
    pub v1: Vertex,
    pub v2: Vertex,
    pub color: Color,
}

pub struct Line2d {
    pub p1: Xy,
    pub p2: Xy,
}

impl Line2d {
    /// Take a 3d line and a canvas to caluclate the size and return a Line2d
    fn from_line(line: &Line, canvas: &mut dyn Canvas) -> Self {
        // let (Xy(mut x1, mut y1), Xy(mut x2, mut y2)) =
        //     (canvas.scale(&self.v1), canvas.scale(&self.v2));
        Line2d {
            p1: canvas.scale(&line.v1),
            p2: canvas.scale(&line.v2),
        }
    }
}

impl Line {
    pub fn from_vertices(v1: &Vertex, v2: &Vertex) -> Line {
        Line {
            v1: v1.clone(),
            v2: v2.clone(),
            color: color::WHITE,
        }
    }
}
// From<(&line::Line, &mut dyn traits::Canvas)>`
// impl From<(&Line, &mut dyn Canvas)> for Line2d {
//     fn from(input: (&Line, &mut dyn Canvas)) -> Self {
//         let (line, canvas) = input;
//         // let (Xy(mut x1, mut y1), Xy(mut x2, mut y2)) =
//         //     (canvas.scale(&self.v1), canvas.scale(&self.v2));
//         Line2d {
//             p1: canvas.scale(&line.v1),
//             p2: canvas.scale(&line.v2),
//         }
//     }
// }
impl Colorful for Line2d {
    fn base_color(&self) -> Color {
        color::GREEN
    }
}

impl Colorful for Line {
    fn color(&self) -> Color {
        self.color
    }
}
impl Drawable for Line {
    fn draw(&self, canvas: &mut dyn Canvas) {
        let line_2d: Line2d = Line2d::from_line(self, canvas);
        line_2d.draw(canvas);
    }
}

impl Drawable for Line2d {
    fn draw(&self, canvas: &mut dyn Canvas) {
        let Line2d {
            p1: Xy(mut x1, mut y1),
            p2: Xy(mut x2, mut y2),
        } = self;

        let mut steep = false;
        if (x1 as i32 - x2 as i32).abs() < (y1 as i32 - y2 as i32).abs() {
            // if the line is steep, we transpose the image
            mem::swap(&mut x1, &mut y1);
            mem::swap(&mut x2, &mut y2);
            steep = true;
        }

        if x1 > x2 {
            // make it left-to-right
            mem::swap(&mut x1, &mut x2);
            mem::swap(&mut y1, &mut y2);
        }

        let dx: i32 = x2 as i32 - x1 as i32;
        let dy: i32 = y2 as i32 - y1 as i32;
        let derror: f64 = (dy as f64 / dx as f64).abs();
        let mut error: f64 = 0.0;
        let mut y: i32 = y1 as i32;

        for x in x1..=x2 {
            if steep {
                canvas.set(Xy(y, x), &self.color());
            } else {
                canvas.set(Xy(x, y), &self.color());
            }

            error += derror;
            if error > 0.5 {
                y += if y2 > y1 { 1 } else { -1 };
                error -= 1.;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_helper::tests::assert_file_creation;

    #[test]
    fn line_draw_test() {
        assert_file_creation("line_draw_test.tga", |filename: &str| {
            let l = Line {
                v1: Vertex {
                    x: 0.10,
                    y: 0.10,
                    z: 0.10,
                },
                v2: Vertex {
                    x: 0.20,
                    y: 0.20,
                    z: 0.20,
                },
                color: Color { r: 255, g: 0, b: 0 },
            };
            let mut i = Image::new(500, 500);
            i.draw(&l);
            i.render_to_file(filename);
        })
    }

    #[test]
    fn multiple_line_draw_test() {
        assert_file_creation("multiple_line_draw_test.tga", |filename: &str| {
            let l0 = Line {
                v1: Vertex::new(0.1, 0.1, 0.1),
                v2: Vertex::new(0.2, 0.20, 0.20),
                color: Color { r: 255, g: 0, b: 0 },
            };
            let l1 = Line {
                v1: Vertex::new(0.5, 0.5, 0.5),
                v2: Vertex::new(0.70, 0.07, 0.05),
                color: Color { r: 0, g: 255, b: 0 },
            };
            let l2 = Line {
                v1: Vertex::new(0.20, 0.20, 0.),
                v2: Vertex::new(1., 0.2, 0.),
                color: Color { r: 0, g: 0, b: 255 },
            };
            let mut i = Image::new(250, 250);
            i.draw(&l0);
            i.draw(&l1);
            i.draw(&l2);
            i.render_to_file(filename);
        })
    }
}
