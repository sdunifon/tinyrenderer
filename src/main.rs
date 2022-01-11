use std::error::Error;
use structopt::StructOpt;

use tinyrenderer::{display_window, load_file, render::RenderOptions, Render};

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

#[cfg_attr(feature = "native_image_render", show_image::main)]
fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let mut render = Render::new(RenderOptions {
        wireframe: args.wireframe,
        height: args.height,
        width: args.width,
        ..Default::default()
    });

    render.load_file(&args.filename)?;
    render.update()?;

    #[cfg(feature = "native_image_render")]
    display_window(&render)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error;

    #[test]
    fn setup_render_test() -> Result<(), Box<dyn error::Error>> {
        load_file("assets/head.obj")?;
        Ok(())
    }
}
