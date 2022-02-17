use super::*;

struct WebCanvas {
    ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

impl WebCanvas {
    fn render(&mut self, renderer: Render){


    }

    fn render_image_buffer(&mut self, image_buffer: &ImageBuffer){
        for x in 0..self.width as u32 {
            for y in 0..self.height as u32 {
                let color: Color;
                let color = image_buffer[&Xy(x as i32, y as i32)];
                // model.renderer.image.data[(y as usize) * model.renderer.width() + (x as usize)];

                self.set_pixel(Xy((self.width - x) as i32, (self.height - y) as i32), color);
            }
        }
    }
    fn set_pixel(&mut self,xy:Xy, color: Color){
       todo!()
    }

    fn clear(&mut self) {
        self.ctx.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }
}