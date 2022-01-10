use common::renderer_with_file;
use std::error;
use std::prelude::*;
use tinyrenderer::Render;

mod common;

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
fn render_triangle_test() {
    todo!()
    let renderer = Renderer::default();

    let triangle = Triangle::default();
    rendenderer_add_drawable(triangle)
}
