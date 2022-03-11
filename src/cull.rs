use crate::scene::{Object, Scene, Triangle};

impl Triangle {
    fn is_front_facing(&self) -> bool {
        let normal = (self.v1 - self.v0).cross(self.v2 - self.v0);

        normal.dot(self.v0) <= 0.0
    }
}

impl Object {
    fn cull_back_faces(&self) -> Self {
        Self {
            triangles: self
                .triangles
                .iter()
                .filter(|t| t.is_front_facing())
                .copied()
                .collect(),
            transform: self.transform,
            bounding_center: self.bounding_center,
            bounding_radius: self.bounding_radius,
        }
    }
}

impl Scene {
    pub fn cull_back_faces(&self) -> Self {
        Self {
            objects: self.objects.iter().map(Object::cull_back_faces).collect(),
            camera: self.camera,
        }
    }
}
