use super::{Pt, Vertex, Xy};

#[derive(Debug, Clone, Copy)]
pub enum Scalar {
    Scale { x: u32, y: u32 },
    None,
}

impl Scalar {
    pub fn scale_v(&self, vertex: &Vertex) -> Xy {
        debug_assert!(
            vertex.x.abs() <= 1. && vertex.y.abs() <= 1.,
            "Vertex: {:?} has coordinates with length greater than one",
            vertex
        );

        match self {
            Scalar::Scale {
                x: width,
                y: height,
            } => {
                let x = Self::scale(vertex.x, *width as f64);
                let y = Self::scale(vertex.y, *height as f64);
                Xy(x, y)
            }
            Scalar::None => Xy(vertex.x.round() as i32, vertex.y.round() as i32),
        }
    }

    //    private
    #[inline]
    fn scale(n: f64, scale: f64) -> i32 {
        (((scale - 1.) * (n + 1.)) / 2.).round() as i32
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
            Xy(0, 99)
        )
    }

    #[test]
    fn scale_v_test1() {
        let s = Scalar::Scale { x: 300, y: 300 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: 0.5,
                y: -0.5,
                z: 0.5,
            }),
            Xy(224, 75)
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
            Xy(50, 50) //49.5 rounds to 50
        )
    }

    #[test]
    fn scale_v_test3() {
        let s = Scalar::Scale { x: 100, y: 100 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: 0.75,
                y: -0.75,
                z: 0.5
            }),
            Xy(87, 12)
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
            Xy(2, 224)
        );
    }

    #[test]
    fn scale_v_test5() {
        let s = Scalar::Scale { x: 101, y: 10 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: 0.1,
                y: 0.1,
                z: 0.5,
            }),
            Xy(55, 5)
        )
    }

    #[test]
    fn scale_v_test6() {
        let s = Scalar::Scale { x: 101, y: 101 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: 1.,
                y: 0.75,
                z: 0.5,
            }),
            Xy(100, 88)
        )
    }

    #[test]
    fn scale_v_test7() {
        let s = Scalar::Scale { x: 101, y: 101 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: 0.99,
                y: 0.987,
                z: 0.5,
            }),
            Xy(100, 99)
        );
    }

    #[test]
    fn scale_v_test8() {
        let s = Scalar::Scale { x: 10, y: 10 };
        assert_eq!(
            s.scale_v(&Vertex {
                x: 0.35,
                y: 0.74,
                z: 0.5,
            }),
            Xy(6, 8)
        );
        assert_eq!(
            s.scale_v(&Vertex {
                x: 1.,
                y: -1.,
                z: 0.5,
            }),
            Xy(9, 0)
        );
    }

    #[test]
    #[should_panic]
    fn scale_v_panic_test() {
        let s = Scalar::Scale { x: 100, y: 100 };
        s.scale_v(&Vertex {
            x: -1.1,
            y: 0.6,
            z: 0.5,
        });
    }

    #[test]
    #[should_panic]
    fn scale_v_panic_test1() {
        let s = Scalar::Scale { x: 100, y: 100 };
        s.scale_v(&Vertex {
            x: -1.0,
            y: 10.6,
            z: 0.5,
        });
    }

    #[test]
    #[should_panic]
    fn scale_v_panic_test2() {
        let s = Scalar::Scale { x: 100, y: 100 };
        s.scale_v(&Vertex {
            x: 0.4,
            y: 10.,
            z: 0.5,
        });
    }
}
