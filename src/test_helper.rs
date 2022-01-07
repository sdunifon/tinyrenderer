#[cfg(test)]
pub mod tests {
    use std::fs;
    use std::path::Path;
    // pub fn assert_file_creation(filename: &str, file_creation: &dyn Fn(&str)) {
    pub fn assert_file_creation<F: FnMut(&str)>(filename: &str, mut f: F) {
        if Path::new(filename).exists() {
            fs::remove_file(filename).unwrap();
        }
        f(filename);
        //TODO should we render here so we don't need it in the closure?
        assert!(Path::new(filename).exists(), "rendered image not found");
        fs::remove_file(filename).unwrap();
    }

    /// short hand for creating a vector with x y z input
    macro_rules! v {
        ($a:expr,$b:expr,$c:expr) => {
            Vector3::new($a as f64, $b as f64, $c as f64)  //TODO take out automatic conversion to f64.. currently haveing some problems with erros sayingit could be f32
        };
    }
    pub(crate) use v;
}
