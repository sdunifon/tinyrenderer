use native_display::display_window;
use std::error::Error;
use structopt::StructOpt;
use tinyrenderer::{render::RenderOptions, Render};

#[allow(dead_code)]
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "render-type", short = "r", default_value = "full")]
    render_type: String,
    #[structopt(long = "height", short = "h", default_value = "800")]
    height: u16,
    #[structopt(long = "width", short = "w", default_value = "800")]
    width: u16,
    #[structopt(long = "wireframe")]
    wireframe: bool,
    #[structopt(default_value = "./assets/cessna.obj")]
    filename: String,
}

#[show_image::main]
fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let mut render = Render::new(RenderOptions {
        wireframe: args.wireframe,
        height: args.height,
        width: args.width,
        ..Default::default()
    });

    render.load_file(&args.filename)?;
    render.update_file_render()?;

    display_window(&render)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error;
    use tinyrenderer::load_file;

    #[test]
    fn setup_render_test() -> Result<(), Box<dyn error::Error>> {
        load_file("assets/head.obj")?;
        Ok(())
    }
}
