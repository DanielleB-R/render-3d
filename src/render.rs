use crate::camera::Viewport;
use crate::canvas::TriangleCanvas;
use crate::scene::{Object, Scene, Triangle};
use glam::{DMat4, IVec2};
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
    pub fn render(&self, canvas: &mut RgbImage, transform_matrix: &DMat4, viewport: &Viewport) {
        let projected: Vec<_> = self
            .vertices
            .iter()
            .map(|v| transform_matrix.transform_point3(*v))
            .map(|v| viewport.project_vertex(canvas, v))
            .collect();

        for t in &self.triangles {
            t.render(canvas, &projected);
        }
    }
}

impl Scene {
    pub fn render(&self, canvas: &mut RgbImage, viewport: &Viewport) {
        let camera_matrix = self.camera.transform.inverse();

        for object in &self.objects {
            let transform_matrix = camera_matrix * object.transform;
            object.render(canvas, &transform_matrix, viewport);
        }
    }
}
