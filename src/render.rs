use crate::canvas::TriangleCanvas;
use crate::scene::Triangle;
use glam::IVec2;

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
