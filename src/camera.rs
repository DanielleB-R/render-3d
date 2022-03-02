use glam::DVec3;
use image::RgbImage;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
pub struct Viewport {
    width: f64,
    height: f64,
    distance: f64,
}

impl Viewport {
    pub fn direction_from_canvas(&self, canvas: &RgbImage, cx: i32, cy: i32) -> DVec3 {
        DVec3::new(
            cx as f64 * (self.width) / (canvas.width() as f64),
            cy as f64 * (self.height) / (canvas.height() as f64),
            self.distance,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
pub struct Camera {
    pub viewport: Viewport,
    pub position: DVec3,
}
