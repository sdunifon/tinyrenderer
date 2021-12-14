use super::Image;
use super::{Pt, Vertex, Xy};
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub enum Scalar {
    Scale { x: u32, y: u32 },
    None,
}

impl Scalar {
    pub fn scale_v(&self, vertex: &Vertex) -> Xy {
        match self {
            Scalar::Scale {
                x: height,
                y: width,
            } => {
                let resized_vertex = *vertex * (height.max(width) / 2) as f64;
                let center_adjust_x: i32 = (*width as i32) / 2;
                let center_adjust_y: i32 = (*height as i32) / 2;
                let x = resized_vertex.x.round() as i32 + center_adjust_x;
                let y = resized_vertex.y.round() as i32 + center_adjust_y;

                Xy(x, y)
            }
            Scalar::None => Xy(vertex.x.round() as i32, vertex.y.round() as i32),
        }
    }
}

pub struct Resizer(pub Box<dyn Fn(&Vertex) -> Pt>);

impl Resizer {
    pub fn new(height: u32, width: u32) -> Resizer {
        let func = move |vertex: &Vertex| -> Pt {
            let resized_vertex = *vertex * (height.max(width) / 2) as f64;
            let x = resized_vertex.x.round() as i32;
            let y = resized_vertex.y.round() as i32;
            Pt::new(
                x,
                y,
                &vertex,
                &Scalar::Scale {
                    x: x as u32,
                    y: y as u32,
                },
            )
        };
        Resizer(Box::new(func))
    }
}
pub struct Translator(pub Box<dyn Fn(&Pt) -> Pt>);
impl Translator {
    pub fn new(height: u32, width: u32) -> Translator {
        let translator = move |pt: &Pt| {
            let mut new_pt = pt.clone();
            new_pt.x += (width / 2) as i32;
            new_pt.y += (height / 2) as i32;
            new_pt
        };
        Translator(Box::new(translator))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_v_test() {
        let s = Scalar::Scale { x: 100, y: 100 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: -1.,
                y: 1.,
                z: 0.5,
            }),
            Xy(0, 100)
        )
    }

    #[test]
    fn scale_v_test2() {
        let s = Scalar::Scale { x: 100, y: 100 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: 0.,
                y: 0.,
                z: 0.5
            }),
            Xy(0, 0)
        )
    }

    #[test]
    fn scale_v_test3() {
        let s = Scalar::Scale { x: 100, y: 100 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: 0.75,
                y: 0.,
                z: 0.5
            }),
            Xy(0, 0)
        )
    }

    #[test]
    fn scale_v_test4() {
        let s = Scalar::Scale { x: 20, y: 300 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: -0.75,
                y: 0.5,
                z: 0.5,
            }),
            Xy(-59, 225)
        );
    }

    #[test]
    fn scale_v_test5() {
        let s = Scalar::Scale { x: 100, y: 10 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: 0.1,
                y: 0.1,
                z: 0.5,
            }),
            Xy(75, 8)
        )
    }

    #[test]
    fn scale_v_test6() {
        let s = Scalar::Scale { x: 101, y: 101 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: 1.,
                y: 0.,
                z: 0.5,
            }),
            Xy(101, 8)
        )
    }

    #[test]
    fn scale_v_test7() {
        let s = Scalar::Scale { x: 101, y: 101 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: -1.,
                y: 0.75,
                z: 0.5,
            }),
            Xy(-101, 75)
        );
    }

    #[test]
    #[should_panic]
    fn scale_v_panic_test() {
        let s = Scalar::Scale { x: 100, y: 100 };
        s.scale_v(&Vertex {
            x: -1.1,
            y: 10.,
            z: 0.5,
        });
    }
}
