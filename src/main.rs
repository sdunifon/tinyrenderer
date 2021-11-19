use std::thread;

// fn main() {
//     make_image("render.tga");
// }
use show_image::{create_window, event};
use tinyrenderer::*;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const NEED_LARGE_STACK: bool = true;
    if NEED_LARGE_STACK {
        image_render_on_large_stack_thread();
    } else {
        image_render()?;
    }
    Ok(())
}

fn image_render_on_large_stack_thread() {
    let child = thread::Builder::new()
        .stack_size(32 * 1024 * 1024 * 1000)
        .spawn(move || {
            image_render().unwrap();
        })
        .unwrap();
    child.join().expect("child threaad failed");
}

fn image_render() -> Result<(), Box<dyn std::error::Error>> {
    let image: Image<IMAGE_SIZE, IMAGE_SIZE>;
    if true {
        image = make_image();
    } else {
        image = render_triangle();
    }

    let image_buffer = image.render_to_buffer();
    image.render("render.tga");
    // let image = ImageView::new(ImageInfo::rgb8(1920, 1080), pixel_data);

    // // Create a window with default options and display the image.
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
