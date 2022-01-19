use std::error::Error;

use show_image::{create_window, event};
use tinyrenderer::{Canvas, Image, ModelFile, Render, Vertex};

pub const IMAGE_HEIGHT: u32 = 1024;
pub const IMAGE_WIDTH: u32 = 1024;

// #[show_image::main] //not working here needs main
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

pub fn make_image() -> Result<Image, Box<dyn Error>> {
    let mut image = Image::new(IMAGE_HEIGHT, IMAGE_WIDTH);

    image.draw(&Vertex {
        x: 50.,
        y: 40.,
        z: 40.,
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
    for triangle in &triangles {
        // triangle.fill(&mut image)
    }
    Ok(image)
}
