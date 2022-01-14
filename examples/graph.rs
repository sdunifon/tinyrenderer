fn main() {
    // let mut f = Funiter(2);
    // let z: Funiter = f.take(20).collect();
    println!("hello world");
}

#[derive(Debug)]
struct Funiter(u64);

impl Funiter {}

impl Iterator for Funiter {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Funiter(self.0 * 2))
    }
}
