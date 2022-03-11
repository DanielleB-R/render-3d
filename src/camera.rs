use glam::{DVec3, IVec2};
use image::RgbImage;
use serde::Deserialize;

use crate::canvas::Canvas;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Viewport {
    width: f64,
    height: f64,
    pub distance: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            distance: 1.0,
        }
    }
}

impl Viewport {
    pub fn direction_from_canvas(&self, canvas: &RgbImage, cx: i32, cy: i32) -> DVec3 {
        DVec3::new(
            cx as f64 * (self.width) / (canvas.width() as f64),
            cy as f64 * (self.height) / (canvas.height() as f64),
            self.distance,
        )
    }

    pub fn canvas_from_viewport(&self, canvas: &Canvas, x: f64, y: f64) -> IVec2 {
        IVec2::new(
            (x * (canvas.width() as f64) / (self.width)).floor() as i32,
            (y * (canvas.height() as f64) / (self.height)).floor() as i32,
        )
    }

    pub fn project_vertex(&self, canvas: &Canvas, vertex: DVec3) -> IVec2 {
        self.canvas_from_viewport(
            canvas,
            vertex[0] * self.distance / vertex[2],
            vertex[1] * self.distance / vertex[2],
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
pub struct Camera {
    pub viewport: Viewport,
    pub position: DVec3,
}
