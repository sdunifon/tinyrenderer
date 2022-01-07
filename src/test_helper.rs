#[cfg(test)]
pub mod tests {
    use crate::vector::Vector3;
    use all_asserts::assert_near;
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

    pub fn assert_vector_eq(v1: Vector3<f64>, v2: Vector3<f64>) {
        assert_near!(v1.x, v2.x, 0.001);
        assert_near!(v1.y, v2.y, 0.001);
        assert_near!(v1.z, v2.z, 0.001);
    }

    /// short hand for creating a vector with x y z input
    macro_rules! v {
        ($a:expr,$b:expr,$c:expr) => {
            Vector3::new($a as f64, $b as f64, $c as f64)  //TODO take out automatic conversion to f64.. currently haveing some problems with erros sayingit could be f32
        };
    }
    pub(crate) use v;
}
