use std::{error::Error, path::PathBuf, process::Command};
use tinyrenderer::Canvas;
use tinyrenderer::{render::RenderError, Image, ModelFile, Render, RenderOptions, Vertex};

pub fn setup_renderer() -> Render {
    Render::default()
}
// fn setup_render(filename: &str) {
pub fn renderer_with_file(filename: &str) -> Result<Render, RenderError> {
    let mut render = Render::default();
    render.load_file(filename)?;
    render.update_file_render()?;
    Ok(render)
}

pub fn render_with_options(
    filename: &str,
    render_options: RenderOptions,
) -> Result<Render, RenderError> {
    let mut render = Render::new(render_options);
    render.load_file(filename)?;
    render.update_file_render()?;
    Ok(render)
}

//TODO redo tset_image
pub fn test_image_1() -> Result<Image, Box<dyn Error>> {
    let mut image = Image::new(800, 800);

    image.draw(&Vertex {
        x: 0.1,
        y: 0.4,
        z: 0.0,
    });

    let file = ModelFile::open_file("assets/head.obj")?;

    let verticies = file.vertex_parse();

    let triangles = file.face_parse(&verticies);
    for triangle in &triangles {
        image.draw(triangle)
    }
    for vertex in &verticies {
        image.draw(vertex)
    }
    // for triangle in &triangles {
    // todo!()
    // triangle.fill(&mut image)
    // }
    Ok(image)
}

pub fn multi_render_suite(filename: &str) {
    todo!()
}

pub fn check_for_image_magick() {
    let output = Command::new("compare")
        .output()
        .expect("Image Magick not installed. Please install Image Magick to run the integration tests. It is only needed for integration tests.");

    if !String::from_utf8_lossy(&output.stdout).contains("Magick") {
        panic!("'compare' script is not imagemagick")
    }
}

fn check_image_difference(filename1: PathBuf, filename2: PathBuf) -> Option<PathBuf> {
    todo!()
}
