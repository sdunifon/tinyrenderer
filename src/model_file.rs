use std::io::BufReader;

use std::fs::File;
use std::io::BufRead;

use lazy_static::lazy_static;
use regex::Regex;

use super::vertex::Vertex;
use super::vertex::Vertices;

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

        let mult = 250.0;
        let mut verticies: Vertices = vec![];

        self.read_iter(|line: &str| {
            let mut line_split = line.split(' ');
            if VERTEX_RE.is_match(line_split.next().unwrap()) {
                // let v = Vertex {
                //     x: line_split.next().unwrap().parse::<f64>().unwrap(),
                //     y: line_split.next().unwrap().parse::<f64>().unwrap(),
                //     z: line_split.next().unwrap().parse::<f64>().unwrap(),
                // };
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
}
