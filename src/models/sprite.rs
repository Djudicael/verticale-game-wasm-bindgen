use wasm_bindgen::{Clamped, JsValue};
// use web_sys::console;
use web_sys::{CanvasRenderingContext2d, ImageData};

use super::player::Position;
use image::DynamicImage;

#[derive(Clone, Debug)]
pub struct Sprite {
    pub position: Position,
    pub image: ImageData,
}
impl Sprite {
    pub fn new(position: Position, image: &DynamicImage) -> Self {
        let rgba_image = image.to_rgba8();
        let image = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(rgba_image.as_raw()),
            image.width(),
            image.height(),
        )
        .unwrap();
        Self { position, image }
    }

    fn draw(&self, context: &mut CanvasRenderingContext2d) -> Result<(), JsValue> {
        context.put_image_data(&self.image, self.position.x, self.position.y)?;
        Ok(())
    }
    pub fn update(&mut self, context: &mut CanvasRenderingContext2d) -> Result<(), JsValue> {
        self.draw(context)?;
        Ok(())
    }
}
