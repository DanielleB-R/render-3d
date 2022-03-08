use crate::camera::Viewport;
use crate::canvas::TriangleCanvas;
use crate::scene::{Object, Scene, Triangle};
use glam::IVec2;
use image::RgbImage;

impl Triangle {
    pub fn render<T: TriangleCanvas>(&self, canvas: &mut T, projected: &[IVec2]) {
        canvas.draw_wireframe_triangle(
            projected[self.vertices[0]],
            projected[self.vertices[1]],
            projected[self.vertices[2]],
            self.color,
        );
    }
}

impl Object {
    pub fn render(&self, canvas: &mut RgbImage, viewport: &Viewport) {
        let projected: Vec<_> = self
            .vertices
            .iter()
            .map(|v| viewport.project_vertex(canvas, *v))
            .collect();

        for t in &self.triangles {
            t.render(canvas, &projected);
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
