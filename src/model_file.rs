use super::*;
use std::io::BufReader;

use std::fs::File;
use std::io::BufRead;

pub struct ModelFile<'a> {
    pub filename: &'a str,
}

impl<'a> ModelFile<'a> {
    fn read_iter<F: FnMut(&str)>(&self, mut func: F) {
        let file = File::open(self.filename).expect("file not found!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            func(&line.unwrap());
        }
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
                    height,
                    width,
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
                Regex::new(r"f (\d*)/\d*/\d* (\d*)/\d*/\d* (\d*)/\d*/\d*").unwrap();
        };

        let mut triangles: Triangles = vec![];

        self.read_iter(|line: &str| {
            match FACE_RE.captures(line) {
                Some(captures) => {
                    // println!("{:?}", captures);

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

    #[test]
    fn read_test() {
        let m = ModelFile {
            filename: "head.obj",
        };
        m.vertex_parse(500, 500);
    }
    #[test]
    fn vertex_parse_test() {
        let m = ModelFile {
            filename: "head.obj",
        };
        let vecs = m.vertex_parse(500, 500);
        assert_eq!(
            vecs[0],
            Vertex {
                x: 249,
                y: 66,
                z: 0,
            }
        );
        assert_eq!(
            vecs[17],
            Vertex {
                x: 415,
                y: 92,
                z: 0,
            }
        );
    }
    #[test]
    fn face_parse_test() {
        let m = ModelFile {
            filename: "head.obj",
        };

        let verts = m.vertex_parse(500, 500);
        let faces = m.face_parse(&verts);
        assert_eq!(faces.len(), 2492);
    }
}
