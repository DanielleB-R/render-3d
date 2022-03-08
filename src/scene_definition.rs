use glam::DVec3;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct TriangleDefinition {
    pub vertices: [usize; 3],
    pub color: DVec3,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelDefinition {
    pub vertices: Vec<DVec3>,
    pub triangles: Vec<TriangleDefinition>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct RotationDefinition {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub z: f64,
}

fn default_scale() -> f64 {
    1.0
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct TransformDefinition {
    #[serde(default = "default_scale")]
    pub scale: f64,
    #[serde(default)]
    pub rotation: RotationDefinition,
    #[serde(default)]
    pub translation: DVec3,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstanceDefinition {
    pub model: String,
    pub transform: TransformDefinition,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct CameraDefinition {
    pub transform: TransformDefinition,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SceneDefinition {
    pub models: HashMap<String, ModelDefinition>,
    pub instances: Vec<InstanceDefinition>,
    pub camera: CameraDefinition,
}
