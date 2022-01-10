use tinyrenderer::{render::RenderError, Render};

pub fn setup_renderer() -> Render {
    Render::default()
}
// fn setup_render(filename: &str) {
pub fn renderer_with_file(filename: &str) -> Result<Render, RenderError> {
    let mut render = Render::default();
    render.load_file(filename)?;
    render.update()?;
    Ok(render)
}
