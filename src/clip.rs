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
    // We call this with the one positive vertex as `a`
    fn clip_one_positive(&self, a: DVec3, b: DVec3, c: DVec3, plane: Plane) -> Vec<Self> {
        vec![Self {
            v0: a,
            v1: plane.intersection(a, b),
            v2: plane.intersection(a, c),
            color: self.color,
        }]
    }

    // We call this with the one negative vertex as `c`
    fn clip_one_negative(&self, a: DVec3, b: DVec3, c: DVec3, plane: Plane) -> Vec<Self> {
        let ap = plane.intersection(a, c);
        let bp = plane.intersection(b, c);

        vec![
            Self {
                v0: a,
                v1: b,
                v2: ap,
                color: self.color,
            },
            Self {
                v0: ap,
                v1: b,
                v2: bp,
                color: self.color,
            },
        ]
    }

    fn clip_against_plane(&self, plane: Plane) -> Vec<Self> {
        let d0 = plane.signed_distance(self.v0);
        let d1 = plane.signed_distance(self.v1);
        let d2 = plane.signed_distance(self.v2);

        match (d0.signum() as i32, d1.signum() as i32, d2.signum() as i32) {
            (1, 1, 1) => vec![*self],
            (-1, -1, -1) => vec![],
            (1, -1, -1) => self.clip_one_positive(self.v0, self.v1, self.v2, plane),
            (-1, 1, -1) => self.clip_one_positive(self.v1, self.v0, self.v2, plane),
            (-1, -1, 1) => self.clip_one_positive(self.v2, self.v0, self.v1, plane),
            (-1, 1, 1) => self.clip_one_negative(self.v1, self.v2, self.v0, plane),
            (1, -1, 1) => self.clip_one_negative(self.v0, self.v2, self.v1, plane),
            (1, 1, -1) => self.clip_one_negative(self.v0, self.v1, self.v2, plane),
            _ => unreachable!(),
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

        Some(Self {
            triangles: self
                .triangles
                .iter()
                .map(|t| t.clip_against_plane(plane))
                .flatten()
                .collect(),
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
