use crate::camera::Viewport;
use crate::canvas::Canvas;
use crate::scene::{Object, Scene, Triangle};

impl Triangle {
    pub fn render(&self, canvas: &mut Canvas, viewport: &Viewport) {
        canvas.draw_filled_triangle(
            viewport.project_vertex(canvas, self.v0),
            viewport.project_vertex(canvas, self.v1),
            viewport.project_vertex(canvas, self.v2),
            self.color,
        );
    }
}

impl Object {
    pub fn render(&self, canvas: &mut Canvas, viewport: &Viewport) {
        for t in &self.triangles {
            t.render(canvas, viewport);
        }
    }
}

impl Scene {
    pub fn render(&self, canvas: &mut Canvas) {
        for object in &self.objects {
            object.render(canvas, &self.camera.viewport);
        }
    }
}
