use tinyrenderer::{render::RenderError, Render, RenderOptions};

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

pub fn multi_render_suite(filename: &str) {
    todo!()
}
