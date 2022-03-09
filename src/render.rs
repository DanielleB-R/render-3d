use crate::camera::Viewport;
use crate::canvas::TriangleCanvas;
use crate::scene::{Object, Scene, Triangle};
use image::RgbImage;

impl Triangle {
    pub fn render(&self, canvas: &mut RgbImage, viewport: &Viewport) {
        canvas.draw_wireframe_triangle(
            viewport.project_vertex(canvas, self.v0),
            viewport.project_vertex(canvas, self.v1),
            viewport.project_vertex(canvas, self.v2),
            self.color,
        );
    }
}

impl Object {
    pub fn render(&self, canvas: &mut RgbImage, viewport: &Viewport) {
        for t in &self.triangles {
            t.render(canvas, viewport);
        }
    }
}

impl Scene {
    pub fn render(&self, canvas: &mut RgbImage) {
        for object in &self.objects {
            object.render(canvas, &self.camera.viewport);
        }
    }
}
