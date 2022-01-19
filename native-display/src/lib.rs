use show_image::{create_window, event};
use tinyrenderer::Render;

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
