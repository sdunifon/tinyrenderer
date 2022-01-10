use std::error::Error;
use structopt::StructOpt;

use tinyrenderer::{display_window, load_file};

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "render-type", short = "r", default_value = "full")]
    render_type: String,
    #[structopt(default_value = "./assets/cessna.obj")]
    filename: String,
}
#[cfg_attr(feature = "native_image_render", show_image::main)]
fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let render = load_file(&args.filename);

    #[cfg(feature = "native_image_render")]
    display_window(&render.unwrap())?;

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
