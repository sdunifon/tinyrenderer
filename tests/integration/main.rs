use crate::common::*;
use std::error;
use std::fs;
use std::path::Path;
use std::prelude::*;
use tinyrenderer::RenderError;
use tinyrenderer::RenderOptions;
use tinyrenderer::Drawable;
use tinyrenderer::Render;
use tinyrenderer::Triangle;
use tinyrenderer::Vertex;

mod common;
type TestResult = Result<(), Box<dyn error::Error>>;

#[test]
fn renderer_load_head_test() -> Result<(), Box<dyn error::Error>> {
    renderer_with_file("assets/head.obj")?;
    Ok(())
}

#[test]
fn renderer_load_airboat_test() -> Result<(), Box<dyn error::Error>> {
    renderer_with_file("assets/airboat.obj")?;
    Ok(())
}
#[test]
fn renderer_load_cessna_test() -> Result<(), Box<dyn error::Error>> {
    renderer_with_file("assets/cessna.obj")?;
    Ok(())
}
#[test]
fn renderer_load_torus_test() -> Result<(), Box<dyn error::Error>> {
    renderer_with_file("assets/torus.obj")?;
    Ok(())
}
#[test]
fn renderer_load_high_poly_torus_test() -> Result<(), Box<dyn error::Error>> {
    renderer_with_file("assets/torus_high_poly.obj")?;
    Ok(())
}

#[test]
fn render_wireframe() -> Result<(), Box<dyn error::Error>> {
    let mut render = Render::new(RenderOptions {
        wireframe: false,
        height: 1024,
        width: 1024,
        ..Default::default()
    });

    render.load_file("assets/cessna.obj")?;
    render.update_file_render()?;
    Ok(())
}
//TODO idea take a snapshot of a render png and compare it to future tests.
#[test]
fn render_high_res_test() -> TestResult {
    render_with_options(
        "assets/torus_high_poly.obj",
        RenderOptions {
            wireframe: true,
            height: 8000,
            width: 8000,
            ..Default::default()
        },
    );
    Ok(())
}
#[test]
fn render_triangle_test() -> TestResult {
    let mut renderer = Render::default();

    let triangle = Triangle::new([
        Vertex::new(0.3, 0.7, 0.2),
        Vertex::new(0.6, 0.9, 0.2),
        Vertex::new(0.2, -0.3, 0.4),
    ]);

    triangle.draw_on(&mut renderer.image);
    Ok(())
}

#[test]
fn image_magick_installed_test() {
    check_for_image_magick();
}

#[test]
fn make_image_test() {
    let filename = "lib_test_render.tga";
    if Path::new(filename).exists() {
        fs::remove_file(filename).unwrap();
    }
    test_image_1().unwrap().render_to_file(filename);
    assert!(Path::new(filename).exists(), "rendered image not found");
    fs::remove_file(filename).unwrap();
}
