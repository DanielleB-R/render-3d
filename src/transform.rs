use crate::scene::{Object, Scene, Triangle};
use glam::DMat4;

impl Triangle {
    pub fn transform(&self, transform_matrix: &DMat4) -> Self {
        Self {
            v0: transform_matrix.transform_point3(self.v0),
            v1: transform_matrix.transform_point3(self.v1),
            v2: transform_matrix.transform_point3(self.v2),
            color: self.color,
        }
    }
}

impl Object {
    pub fn transform(&self, camera_matrix: DMat4) -> Self {
        let transform_matrix = camera_matrix * self.transform;
        Self {
            triangles: self
                .triangles
                .iter()
                .map(|t| t.transform(&transform_matrix))
                .collect(),
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
