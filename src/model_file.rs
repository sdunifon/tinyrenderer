use std::io::BufReader;

use std::fs::File;
use std::io::BufRead;

use super::vertex::Vertex;
pub struct ModelFile<'a> {
    pub filename: &'a str,
}

impl<'a> ModelFile<'a> {
    fn read_iter(&self, func: fn(&str)) {
        let file = File::open(self.filename).expect("file not found!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            func(&line.unwrap());
        }
    }

    pub fn usplay(&self) {
        let mut split = "Mary had a little lamb".split(' ');
        assert_eq!(split.as_str(), "Mary had a little lamb");
        split.next();
        println!("{:?}", split.as_str());
        self.read_iter(|line| {
            let mut line_split = line.split(' ');
            if line_split.next().unwrap().chars().nth(0) == Some('v') {
                let v = Vertex {
                    x: line_split.next().unwrap().parse().unwrap(),
                    y: line_split.next().unwrap().parse().unwrap(),
                    z: line_split.next().unwrap().parse().unwrap(),
                };
                println!("{:?}", v);
            }
        })
    }

    pub fn display(&self) {
        // self.read_iter(|line| {
        //     let line_input:[&str;3] = line.split(' ').collect();
        //     if line_input[0].chars().nth(0) == Some('v') {
        //         let v = Vertex {
        //             x: line_input[0].parse().unwrap(),
        //             y: line_input[1].parse().unwrap(),
        //             z: line_input[2].parse().unwrap(),
        //         }
        //     println!("{?:}", v);
        //     }
        // })
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
        m.usplay();
    }
}
