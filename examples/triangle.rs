fn main() {
    #[cfg(not(feature = "native_image_render"))]
    panic!("must have native_image_render feature enabled");

    println!("hello triangle")
}
