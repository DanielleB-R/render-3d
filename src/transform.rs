use crate::scene::{Object, Scene};
use glam::DMat4;

impl Object {
    pub fn transform(&self, camera_matrix: DMat4) -> Self {
        let transform_matrix = camera_matrix * self.transform;
        Self {
            vertices: self
                .vertices
                .iter()
                .map(|v| transform_matrix.transform_point3(*v))
                .collect(),
            triangles: self.triangles.clone(),
            transform: self.transform,
            bounding_center: transform_matrix.transform_point3(self.bounding_center),
            bounding_radius: self.bounding_radius,
        }
    }
}

impl Scene {
    pub fn transform(&self) -> Self {
        let camera_matrix = self.camera.transform.inverse();
        Self {
            objects: self
                .objects
                .iter()
                .map(|obj| obj.transform(camera_matrix))
                .collect(),
            camera: self.camera,
        }
    }
}
