use super::*;
use std::io::BufReader;

use std::fs::File;
use std::io::BufRead;

pub struct ModelFile {
    filename: String,
    pub verticies: Vertices,
    pub triangles: Triangles,
}

impl ModelFile {
    pub fn open(filename: &str) -> ModelFile {
        ModelFile {
            filename: filename.to_string(),
            verticies: Vec::new(),
            triangles: Vec::new(),
        }
    }

    fn read_iter<F: FnMut(&str)>(&self, mut func: F) {
        let file = File::open(&self.filename).expect("file not found!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            func(&line.unwrap());
        }
    }

    pub fn load(&mut self, height: usize, width: usize) {
        self.verticies = self.vertex_parse(height, width);
        self.triangles = self.face_parse(&self.verticies);
    }

    pub fn vertex_parse(&self, height: usize, width: usize) -> Vertices {
        lazy_static! {
            static ref VERTEX_RE: Regex = Regex::new("v$").unwrap();
        };

        let mut verticies: Vertices = vec![];

        self.read_iter(|line: &str| {
            let mut line_split = line.split(' ');
            if VERTEX_RE.is_match(line_split.next().unwrap()) {
                let v = Vertex::new_resized(
                    line_split.next().unwrap().parse::<f64>().unwrap(),
                    line_split.next().unwrap().parse::<f64>().unwrap(),
                    line_split.next().unwrap().parse::<f64>().unwrap(),
                    height as f64,
                    width as f64,
                );
                verticies.push(v.clone());
            }
        });
        // println!("--{:?}", verticies);
        verticies
    }

    pub fn face_parse(&self, verticies: &Vertices) -> Triangles {
        lazy_static! {
            static ref FACE_RE: Regex =
                //Regex::new(r"f (\d*)/\d*/\d* (\d*)/\d*/\d* (\d*)/\d*/\d*").unwrap();
                //Regex::new(r"f (\d*)/?[^\s]* (\d*)/?[^\s]* (\d*)/?[^\s]*").unwrap();
                Regex::new(r"f (\d*) (\d*) (\d*)").unwrap();
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
        let verts = m.vertex_parse(500, 500);
        assert!(verts.len() > 100);
        let triangles = m.face_parse(&verts);
        assert!(triangles.len() > 100);
    }

    #[test]
    fn read_test() {
        let m = ModelFile::open("assets/head.obj");
        m.vertex_parse(250, 250);
    }
    #[test]
    fn vertex_parse_test() {
        let m = ModelFile::open("assets/head.obj");
        let vecs = m.vertex_parse(500, 500);
        assert_eq!(
            vecs[0],
            Vertex {
                x: 250,
                y: 66,
                z: 94,
            }
        );
        assert_eq!(
            vecs[17],
            Vertex {
                x: 416,
                y: 92,
                z: 189,
            }
        );
    }
    #[test]
    fn face_parse_test() {
        let m = ModelFile::open("assets/head.obj");

        let verts = m.vertex_parse(500, 500);
        let faces = m.face_parse(&verts);
        assert_eq!(faces.len(), 2492);
    }

    #[test]
    fn min_max_range_detection_name() {
        let m = ModelFile::open("assets/head.obj");
        let verts = m.vertex_parse(500, 500);
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
}
