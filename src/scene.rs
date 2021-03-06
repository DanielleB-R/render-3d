use crate::camera::Viewport;
use crate::scene_definition::{
    CameraDefinition, InstanceDefinition, ModelDefinition, RotationDefinition, SceneDefinition,
    TransformDefinition, TriangleDefinition,
};
use glam::{DMat4, DQuat, DVec3, EulerRot};
use serde::Deserialize;
use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v0: DVec3,
    pub v1: DVec3,
    pub v2: DVec3,
    pub color: DVec3,
}

impl From<(TriangleDefinition, &Vec<DVec3>)> for Triangle {
    fn from((triangle, vertices): (TriangleDefinition, &Vec<DVec3>)) -> Self {
        Self {
            v0: vertices[triangle.vertices[0]],
            v1: vertices[triangle.vertices[1]],
            v2: vertices[triangle.vertices[2]],
            color: triangle.color,
        }
    }
}

impl From<RotationDefinition> for DQuat {
    fn from(rotation: RotationDefinition) -> Self {
        Self::from_euler(
            EulerRot::XYZ,
            rotation.x * PI / 180.0,
            rotation.y * PI / 180.0,
            rotation.z * PI / 180.0,
        )
    }
}

impl From<TransformDefinition> for DMat4 {
    fn from(other: TransformDefinition) -> Self {
        Self::from_scale_rotation_translation(
            DVec3::splat(other.scale),
            other.rotation.into(),
            other.translation,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub triangles: Vec<Triangle>,
    pub transform: DMat4,
    pub bounding_center: DVec3,
    pub bounding_radius: f64,
}

impl From<(InstanceDefinition, &HashMap<String, ModelDefinition>)> for Object {
    fn from((instance, models): (InstanceDefinition, &HashMap<String, ModelDefinition>)) -> Self {
        let model = models.get(&instance.model).unwrap();

        let mut bounding_center: DVec3 = model.vertices.iter().sum();
        bounding_center /= model.vertices.len() as f64;
        let bounding_radius: f64 = model
            .vertices
            .iter()
            .map(|v| (*v - bounding_center).length_squared())
            .reduce(f64::max)
            .unwrap()
            .sqrt();

        let transform: DMat4 = instance.transform.into();

        Self {
            triangles: model
                .triangles
                .iter()
                .map(|t| Triangle::from((*t, &model.vertices)))
                .collect(),
            transform,
            bounding_center,
            bounding_radius,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub transform: DMat4,
    pub viewport: Viewport,
}

impl From<CameraDefinition> for Camera {
    fn from(other: CameraDefinition) -> Self {
        Self {
            transform: other.transform.into(),
            viewport: other.viewport,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "SceneDefinition")]
pub struct Scene {
    pub objects: Vec<Object>,
    pub camera: Camera,
}

impl From<SceneDefinition> for Scene {
    fn from(other: SceneDefinition) -> Self {
        Self {
            objects: other
                .instances
                .into_iter()
                .map(|instance| (instance, &other.models).into())
                .collect(),
            camera: other.camera.into(),
        }
    }
}
