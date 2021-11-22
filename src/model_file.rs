use super::*;
use std::io::BufReader;

use std::fs::File;
use std::io::BufRead;

pub struct ModelFile {
    filename: String,
    pub verticies: Vertices, // create normalized verticies
    pub triangles: Triangles,
    file_vertex_range: Option<VertexRange>, //TODO get rid of this causes extra complexity
}
#[derive(Debug, PartialEq)]
pub struct VertexRange {
    min: f64,
    max: f64,
}
impl VertexRange {
    fn highest(&self) -> f64 {
        if self.min.abs() < self.max.abs() {
            self.max.abs()
        } else {
            self.min.abs()
        }
    }
}

impl ModelFile {
    pub fn open(filename: &str) -> ModelFile {
        ModelFile {
            filename: filename.to_string(),
            verticies: Vec::new(),
            triangles: Vec::new(),
            file_vertex_range: None,
        }
    }

    fn read_iter<F: FnMut(&str)>(&self, mut func: F) {
        // make this a trait so we can use files from the browser
        let file = File::open(&self.filename).expect("file not found!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            func(&line.unwrap());
        }
    }

    fn calculate_vertex_range(&mut self) {
        //TODO remove mutable/get rid of side effects
        debug_assert!(
            self.verticies.len() > 0,
            "Cannot calculate vertext range  because vertexes have not been parsed"
        );
        let mut range = VertexRange { min: 0., max: 0. };
        for &vertex in &self.verticies {
            let Vertex { x, y, z } = vertex;
            for dimension in [x, y, z] {
                if dimension > range.max {
                    range.max = dimension;
                }
                if dimension < range.min {
                    range.min = dimension;
                }
            }
        }
        self.file_vertex_range = Some(range);
    }

    pub fn load(&mut self) {
        self.verticies = self.vertex_parse();
        self.triangles = self.face_parse(&self.verticies);
        self.calculate_vertex_range();
    }

    pub fn vertex_parse(&self) -> Vertices {
        lazy_static! {
            static ref VERTEX_RE: Regex = Regex::new("v$").unwrap();
        };

        let mut verticies: Vertices = vec![];

        self.read_iter(|line: &str| {
            let mut line_split = line.split(' ');
            if VERTEX_RE.is_match(line_split.next().unwrap()) {
                // let v = Vertex::new_resized(
                // line_split.next().unwrap().parse::<f64>().unwrap(),
                // line_split.next().unwrap().parse::<f64>().unwrap(),
                // line_split.next().unwrap().parse::<f64>().unwrap(),
                // height as f64,
                // width as f64,
                // );

                let v = Vertex {
                    x: line_split.next().unwrap().parse::<f64>().unwrap(),
                    y: line_split.next().unwrap().parse::<f64>().unwrap(),
                    z: line_split.next().unwrap().parse::<f64>().unwrap(),
                };
                verticies.push(v.clone());
            }
        });
        verticies
    }

    pub fn normalize_verticies(verticies: Vertices, value_range: VertexRange) -> Vertices {
        let mut normalized_verticies = Vertices::new();
        let scale = value_range.highest();
        for vertex in verticies {
            normalized_verticies.push(vertex / scale);
        }
        normalized_verticies
    }

    pub fn face_parse(&self, verticies: &Vertices) -> Triangles {
        lazy_static! {
            static ref FACE_RE: Regex =
                //Regex::new(r"f (\d*)/\d*/\d* (\d*)/\d*/\d* (\d*)/\d*/\d*").unwrap();
                //Regex::new(r"f (\d*)/?[^\s]* (\d*)/?[^\s]* (\d*)/?[^\s]*").unwrap();
                Regex::new(r"f (\d*)[^\s]* (\d*)[^\s]* (\d*)[^\s]*").unwrap();
        };

        let mut triangles: Triangles = vec![];

        self.read_iter(|line: &str| {
            match FACE_RE.captures(line) {
                Some(captures) => {
                    println!("{:?}", captures);

                    let vertex_indices = [&captures[1], &captures[2], &captures[3]];
                    let vertex_indices: [usize; 3] =
                        vertex_indices.map(|vi_str| vi_str.parse().unwrap());
                    let triangle = Triangle::new(vertex_indices.map(|vi| verticies[vi - 1]));
                    triangles.push(triangle);
                }
                None => (), //println!("couldnt capture{}", line), // faces.push(v.clone());
            }
        });

        return triangles;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_file(filename: &str) {
        let m = ModelFile::open(filename);
        let verts = m.vertex_parse();
        assert!(verts.len() > 100);
        let triangles = m.face_parse(&verts);
        assert!(triangles.len() > 100);
    }

    #[test]
    fn read_test() {
        let m = ModelFile::open("assets/head.obj");
        m.vertex_parse();
    }
    #[test]
    fn vertex_parse_test() {
        let m = ModelFile::open("assets/head.obj");
        let vecs = m.vertex_parse();
        assert_eq!(
            vecs[0],
            Vertex {
                x: -0.000581696,
                y: -0.734665,
                z: -0.623267
            }
        );
        assert_eq!(
            vecs[17],
            Vertex {
                x: 0.66248,
                y: -0.631463,
                z: -0.244119
            }
        );
    }
    #[test]
    fn face_parse_test() {
        let m = ModelFile::open("assets/head.obj");

        let verts = m.vertex_parse();
        let faces = m.face_parse(&verts);
        assert_eq!(faces.len(), 2492);
    }

    #[test]
    fn min_max_range_detection_name() {
        let m = ModelFile::open("assets/head.obj");
        let verts = m.vertex_parse();
    }
    #[test]
    fn parse_head_obj() {
        test_file("assets/head.obj");
    }
    #[test]
    fn parse_airboat_obj() {
        test_file("assets/airboat.obj");
    }
    #[test]
    fn parse_torus_obj() {
        test_file("assets/torus.obj");
    }
    #[test]
    fn parse_cessna_obj() {
        test_file("assets/cessna.obj");
    }
    #[test]
    fn calculate_file_vertex_range_cessna_test() {
        let mut f = ModelFile::open("assets/cessna.obj");
        f.load();
        assert_eq!(
            VertexRange {
                min: -22.152081,
                max: 22.152081,
            },
            f.file_vertex_range.unwrap()
        )
    }
    #[test]
    fn calculate_file_vertex_range_head_test() {
        let mut f = ModelFile::open("assets/head.obj");
        f.load();
        assert_eq!(
            VertexRange { min: -1., max: 1. },
            f.file_vertex_range.unwrap()
        )
    }

    #[test]
    fn normalize_verticies_test() {
        let verts: Vertices = vec![
            Vertex {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            Vertex {
                x: 4.0,
                y: -10.0,
                z: 10.0,
            },
        ];
        let norm_verts = ModelFile::normalize_verticies(
            verts,
            VertexRange {
                min: -10.0,
                max: 10.0,
            },
        );
        assert_eq!(
            norm_verts[0],
            Vertex {
                x: 0.1,
                y: 0.2,
                z: 0.3
            }
        );
        assert_eq!(
            norm_verts[1],
            Vertex {
                x: 0.4,
                y: -1.0,
                z: 1.0
            }
        );
    }
    #[test]
    fn normalize_verticies_test_2() {
        let verts: Vertices = vec![
            Vertex {
                x: 1.0,
                y: -4.0,
                z: 2.0,
            },
            Vertex {
                x: 1.0,
                y: 2.0,
                z: -3.0,
            },
        ];
        let norm_verts = ModelFile::normalize_verticies(
            verts,
            VertexRange {
                min: -4.0,
                max: 2.0,
            },
        );
        assert_eq!(
            norm_verts[0],
            Vertex {
                x: 0.25,
                y: -1.0,
                z: 0.5
            }
        );
        assert_eq!(
            norm_verts[1],
            Vertex {
                x: 0.25,
                y: 0.5,
                z: -0.75
            }
        );
    }
}
