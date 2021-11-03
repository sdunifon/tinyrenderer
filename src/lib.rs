#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(destructuring_assignment)]

mod image;
mod line;

mod test_helper;

use image::*;

pub fn make_image(filename: &str) {
    let mut i = Image::<500, 500>::new();
    i.set(Pt(50, 50), Px { r: 0, g: 255, b: 0 });
    i.render(filename);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn make_image_test() {
        let filename = "lib_test_render.tga";
        if Path::new(filename).exists() {
            fs::remove_file(filename).unwrap();
        }
        make_image(filename);
        assert!(Path::new(filename).exists(), "rendered image not found");
        fs::remove_file(filename).unwrap();
    }
}
