use std::fs;
use std::path::Path;

// pub fn assert_file_creation(filename: &str, file_creation: &dyn Fn(&str)) {
pub fn assert_file_creation<F: Fn(&str)>(filename: &str, f: F) {
    if Path::new(filename).exists() {
        fs::remove_file(filename).unwrap();
    }
    f(filename);
    //TODO should we render here so we don't need it in the closure?
    assert!(Path::new(filename).exists(), "rendered image not found");
    fs::remove_file(filename).unwrap();
}
