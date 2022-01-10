use std::error::Error;
use structopt::StructOpt;

use tinyrenderer::render::{Render, RenderError};

#[cfg(feature = "native_image_render")]
use show_image::{create_window, event};

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
    let render = setup_render(&args.filename);

    #[cfg(feature = "native_image_render")]
    display_window(&render.unwrap())?;

    Ok(())
}

// fn setup_render(filename: &str) {
fn setup_render(filename: &str) -> Result<Render, RenderError> {
    let mut render = Render::default();
    render.load_file(filename)?;
    render.update()?;
    Ok(render)
}

#[cfg(feature = "native_image_render")]
fn display_window(render: &Render) -> Result<(), Box<dyn std::error::Error>> {
    let image_buffer = render.image.render_to_buffer();

    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image_buffer)?;

    for event in window.event_channel()? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            println!("{:#?}", event);
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error;

    use super::*;

    #[test]
    fn setup_render_test() -> Result<(), Box<dyn error::Error>> {
        setup_render("assets/head.obj")?;
        Ok(())
    }
}
