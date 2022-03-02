use glam::Vec3;
use image::RgbImage;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
pub struct Viewport {
    width: f32,
    height: f32,
    distance: f32,
}

impl Viewport {
    pub fn direction_from_canvas(&self, canvas: &RgbImage, cx: i32, cy: i32) -> Vec3 {
        Vec3::new(
            cx as f32 * (self.width) / (canvas.width() as f32),
            cy as f32 * (self.height) / (canvas.height() as f32),
            self.distance,
        )
    }
}
