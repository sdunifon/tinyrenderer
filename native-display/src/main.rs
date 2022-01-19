use show_image::{create_window, event};
use std::error::Error;
use structopt::StructOpt;
use tinyrenderer::{render::RenderOptions, Render};

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

pub fn display_window(render: &Render) -> Result<(), Box<dyn std::error::Error>> {
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
    use super::*;
    use std::error;
    use tinyrenderer::load_file;

    #[test]
    fn setup_render_test() -> Result<(), Box<dyn error::Error>> {
        load_file("assets/head.obj")?;
        Ok(())
    }
}
