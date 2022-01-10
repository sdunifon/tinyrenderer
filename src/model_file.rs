use super::*;
use std::error::{self, Error};
use std::io::BufReader;

use crate::render::{RenderError, RenderOptions};
use std::fs::File;
use std::io::BufRead;

#[derive(Default)]
pub struct ModelFile {
    pub file_data: Vec<String>,
    pub vertices: Option<NormalizedVertices>, // create normalized verticies
    pub triangles: Triangles,
}

impl From<&str> for ModelFile {
    fn from(str: &str) -> Self {
        ModelFile {
            file_data: str.split('\n').map(|str| str.to_string()).collect(),
            ..Default::default()
        }
    }
}
impl From<File> for ModelFile {
    fn from(file: File) -> ModelFile {
        // File::open(filename).expect("file not found!");
        let file_data = BufReader::new(file).lines().map(|l| l.unwrap()).collect();
        ModelFile {
            file_data,
            ..Default::default()
        }
    }
}

impl ModelFile {
    pub fn open_file(filename: &str) -> Result<ModelFile, Box<dyn Error>> {
        match File::open(filename) {
            Ok(file) => Ok(Self::from(file)),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn read_iter<F: FnMut(&str)>(&self, mut func: F) {
        // make this a trait so we can use files from the browser

        for line in &self.file_data {
            func(&line);
        }
    }

    pub fn load(&mut self) {
        let original_vertices = self.vertex_parse();

        self.vertices = Some(NormalizedVertices::from(original_vertices));
        self.triangles = self.face_parse(&self.vertices.as_ref().unwrap());
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

pub struct ModelFileDrawer<'a> {
    pub options: &'a RenderOptions,
    pub model_file: &'a ModelFile,
}

impl<'a> Drawable for ModelFileDrawer<'a> {
    fn draw(&self, canvas: &mut dyn Canvas) {
        if self.options.wireframe {
            self.model_file.triangles.draw(canvas);
        } else {
            self.model_file.vertices.as_ref().unwrap().draw(canvas);
            self.model_file.triangles.fill(canvas);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn test_file(filename: &str) {
        let m = ModelFile::open_file(filename).unwrap();
        let verts = m.vertex_parse();
        assert!(verts.len() > 100);
        let triangles = m.face_parse(&verts);
        assert!(triangles.len() > 100);
    }

    #[test]
    fn read_test() {
        let m = ModelFile::open_file("assets/head.obj").unwrap();
        m.vertex_parse();
    }
    #[test]
    fn vertex_parse_test() {
        let m = ModelFile::open_file("assets/head.obj").unwrap();
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
        let m = ModelFile::open_file("assets/head.obj").unwrap();

        let verts = m.vertex_parse();
        let faces = m.face_parse(&verts);
        assert_eq!(faces.len(), 2492);
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
    fn handle_parse_error_test() {
        todo!()
    }
    #[test]
    fn handle_file_with_quads_instead_of_trigangles_error_test() {
        todo!()
    }
}
