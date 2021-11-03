use std::io::BufReader;

use std::fs::File;
use std::io::BufRead;

pub struct ModelFile<'a> {
    filename: &'a str,
}

impl<'a> ModelFile<'a> {
    fn read_iter(&self, func: fn(&str)) {
        let file = File::open(self.filename).expect("file not found!");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            func(&line.unwrap());
        }
    }

    pub fn display(&self) {
        self.read_iter(|line| {
            println!("{}", line);
        })
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
        m.display();
    }
}
