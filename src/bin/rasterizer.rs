use glam::{DMat3, DMat4, DQuat, DVec3, EulerRot, IVec2};
use image::{ImageBuffer, RgbImage};
use serde::Deserialize;
use std::collections::HashMap;
use std::f64::consts::PI;

use render_3d::camera::Viewport;
use render_3d::canvas::TriangleCanvas;
use render_3d::scene::Triangle;

// #[derive(Debug, Clone, Copy)]
// struct ShadedVertex {
//     position: IVec2,
//     intensity: f64,
// }

// impl ShadedVertex {
//     fn new(x: i32, y: i32, h: f64) -> Self {
//         Self {
//             position: IVec2::new(x, y),
//             intensity: h,
//         }
//     }

//     fn x(&self) -> i32 {
//         self.position[0]
//     }

//     fn y(&self) -> i32 {
//         self.position[1]
//     }

//     fn h(&self) -> f64 {
//         self.intensity
//     }
// }

// #[derive(Debug, Clone, Copy, Deserialize)]
// struct Triangle {
//     vertices: [usize; 3],
//     color: DVec3,
// }

// impl Triangle {
//     fn render(&self, canvas: &mut RgbImage, projected: &[IVec2]) {
//         canvas.draw_wireframe_triangle(
//             projected[self.vertices[0]],
//             projected[self.vertices[1]],
//             projected[self.vertices[2]],
//             self.color,
//         );
//     }
// }

#[derive(Debug, Clone, Deserialize)]
struct Object {
    vertices: Vec<DVec3>,
    triangles: Vec<Triangle>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct DegreeRotation {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default)]
    z: f64,
}

impl From<DegreeRotation> for DMat3 {
    fn from(rotation: DegreeRotation) -> Self {
        Self::from_euler(
            EulerRot::XYZ,
            rotation.x * PI / 180.0,
            rotation.y * PI / 180.0,
            rotation.z * PI / 180.0,
        )
    }
}

fn default_scale() -> f64 {
    1.0
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct DegreeTransform {
    #[serde(default = "default_scale")]
    scale: f64,
    #[serde(default)]
    rotation: DegreeRotation,
    #[serde(default)]
    translation: DVec3,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(from = "DegreeTransform")]
struct Transform {
    scale: f64,
    rotation: DMat3,
    translation: DVec3,
}

impl From<DegreeTransform> for Transform {
    fn from(other: DegreeTransform) -> Self {
        Self {
            scale: other.scale,
            rotation: other.rotation.into(),
            translation: other.translation,
        }
    }
}

impl From<Transform> for DMat4 {
    fn from(other: Transform) -> Self {
        Self::from_scale_rotation_translation(
            DVec3::splat(other.scale),
            DQuat::from_mat3(&other.rotation),
            other.translation,
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Instance {
    model: String,
    transform: Transform,
}

impl Instance {
    fn render(
        &self,
        canvas: &mut RgbImage,
        transform_matrix: &DMat4,
        viewport: &Viewport,
        models: &HashMap<String, Object>,
    ) {
        let model = models.get(&self.model).unwrap();

        let projected: Vec<_> = model
            .vertices
            .iter()
            .map(|v| transform_matrix.transform_point3(*v))
            .map(|v| viewport.project_vertex(canvas, v))
            .collect();

        for t in &model.triangles {
            t.render(canvas, &projected);
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Camera {
    transform: Transform,
}

#[derive(Debug, Clone, Deserialize)]
struct Scene {
    models: HashMap<String, Object>,
    instances: Vec<Instance>,
    camera: Camera,
}

impl Scene {
    fn render(&self, canvas: &mut RgbImage, viewport: &Viewport) {
        let camera_matrix = DMat4::from(self.camera.transform).inverse();
        for instance in &self.instances {
            let transform_matrix = camera_matrix * DMat4::from(instance.transform);
            instance.render(canvas, &transform_matrix, viewport, &self.models);
        }
    }
}

fn main() {
    let mut buffer: RgbImage = ImageBuffer::new(512, 512);
    let viewport: Viewport = Default::default();

    let scene: Scene = serde_yaml::from_slice(&std::fs::read("cube.yaml").unwrap()).unwrap();

    scene.render(&mut buffer, &viewport);

    buffer.save("raster.png").unwrap();
}
