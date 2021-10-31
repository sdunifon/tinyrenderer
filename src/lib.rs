#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use crate::image::Image;
use crate::image::Pt;
use crate::image::Px;

mod image;
pub fn make_image(filename: &str) {
    let mut i = Image::<500, 500>::new();
    i.set(Pt(250, 250), Px { r: 0, g: 0, b: 255 });
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
        let img = Image::<200, 200>::new();
        if Path::new(filename).exists() {
            fs::remove_file(filename).unwrap();
        }
        img.render(filename);
        assert!(Path::new(filename).exists(), "rendered image not found");
        fs::remove_file(filename).unwrap();
    }
}
