use common::render_with_options;
use common::renderer_with_file;
use std::error;
use std::prelude::*;
use tinyrenderer::render::RenderOptions;
use tinyrenderer::Render;

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
    render.update()?;
    Ok(())
}

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
fn render_triangle_test() {
    todo!();
    // let renderer = Renderer::default();

    // let triangle = Triangle::default();
    // rendenderer_add_drawable(triangle)
}
