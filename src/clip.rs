use crate::scene::{Camera, Object, Scene, Triangle};
use glam::DVec3;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    normal: DVec3,
    d: f64,
}

impl Plane {
    fn signed_distance(&self, vertex: DVec3) -> f64 {
        self.normal.dot(vertex) + self.d
    }

    fn intersection(&self, v0: DVec3, v1: DVec3) -> DVec3 {
        let t = (-self.d - self.normal.dot(v0)) / self.normal.dot(v1 - v0);
        v0 + t * (v1 - v0)
    }
}

impl Triangle {
    fn clip_against_plane(&self, plane: Plane, vertices: &mut Vec<DVec3>) -> Vec<Self> {
        let v0 = vertices[self.vertices[0]];
        let v1 = vertices[self.vertices[1]];
        let v2 = vertices[self.vertices[2]];

        let d0 = plane.signed_distance(v0);
        let d1 = plane.signed_distance(v1);
        let d2 = plane.signed_distance(v2);

        let sign = d0.signum() * d1.signum() * d2.signum();

        if sign > 0.0 {
            if d0 > 0.0 && d1 > 0.0 && d2 > 0.0 {
                // all positive, we're in the volume
                vec![*self]
            } else {
                // only one positive
                let (ia, a, b, c) = if d0 > 0.0 {
                    (self.vertices[0], v0, v1, v2)
                } else if d1 > 0.0 {
                    (self.vertices[1], v1, v0, v2)
                } else {
                    (self.vertices[2], v2, v0, v1)
                };

                let bp = plane.intersection(a, b);
                let cp = plane.intersection(a, c);

                let ibp = vertices.len();

                vertices.push(bp);
                vertices.push(cp);

                vec![Self {
                    vertices: [ia, ibp, ibp + 1],
                    color: self.color,
                }]
            }
        } else {
            if d0 < 0.0 && d1 < 0.0 && d2 < 0.0 {
                // all negative, we're out of the volume
                vec![]
            } else {
                // only one negative
                let (ia, ib, a, b, c) = if d0 < 0.0 {
                    (self.vertices[1], self.vertices[2], v1, v2, v0)
                } else if d1 < 0.0 {
                    (self.vertices[0], self.vertices[2], v0, v2, v1)
                } else {
                    (self.vertices[0], self.vertices[1], v0, v1, v2)
                };

                let ap = plane.intersection(a, c);
                let bp = plane.intersection(b, c);

                let iap = vertices.len();
                vertices.push(ap);
                vertices.push(bp);

                vec![
                    Self {
                        vertices: [ia, ib, iap],
                        color: self.color,
                    },
                    Self {
                        vertices: [iap, ib, iap + 1],
                        color: self.color,
                    },
                ]
            }
        }
    }
}

impl Object {
    fn clip_against_plane(&self, plane: Plane) -> Option<Self> {
        let d = plane.signed_distance(self.bounding_center);

        if d > self.bounding_radius {
            return Some(self.clone());
        } else if d < -self.bounding_radius {
            return None;
        }

        let mut vertices = self.vertices.clone();
        let triangles = self
            .triangles
            .iter()
            .map(|t| t.clip_against_plane(plane, &mut vertices))
            .flatten()
            .collect();

        Some(Self {
            vertices,
            triangles,
            transform: self.transform,
            bounding_center: self.bounding_center,
            bounding_radius: self.bounding_radius,
        })
    }

    fn clip(&self, planes: &[Plane]) -> Option<Self> {
        let mut object = self.clone();
        for plane in planes {
            match self.clip_against_plane(*plane) {
                Some(obj) => object = obj,
                None => return None,
            }
        }
        Some(object)
    }
}

impl Scene {
    pub fn clip(&self) -> Self {
        let planes = self.camera.clipping_planes();
        Self {
            objects: self
                .objects
                .iter()
                .filter_map(|obj| obj.clip(&planes))
                .collect(),
            camera: self.camera,
        }
    }
}

impl Camera {
    pub fn clipping_planes(&self) -> Vec<Plane> {
        vec![
            Plane {
                normal: DVec3::new(0.0, 0.0, 1.0),
                d: -self.viewport.distance,
            },
            Plane {
                normal: DVec3::new(1.0, 0.0, 1.0).normalize(),
                d: 0.0,
            },
            Plane {
                normal: DVec3::new(-1.0, 0.0, 1.0).normalize(),
                d: 0.0,
            },
            Plane {
                normal: DVec3::new(0.0, 1.0, 1.0).normalize(),
                d: 0.0,
            },
            Plane {
                normal: DVec3::new(0.0, -1.0, 1.0).normalize(),
                d: 0.0,
            },
        ]
    }
}
