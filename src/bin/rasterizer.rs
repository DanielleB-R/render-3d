use glam::{DMat3, DMat4, DQuat, DVec3, EulerRot, IVec2};
use image::{ImageBuffer, RgbImage};
use serde::Deserialize;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::mem;

use render_3d::camera::Viewport;
use render_3d::canvas::{LineCanvas, SymmetricCanvas};
use render_3d::utils::interpolate;

#[derive(Debug, Clone, Copy)]
struct ShadedVertex {
    position: IVec2,
    intensity: f64,
}

impl ShadedVertex {
    fn new(x: i32, y: i32, h: f64) -> Self {
        Self {
            position: IVec2::new(x, y),
            intensity: h,
        }
    }

    fn x(&self) -> i32 {
        self.position[0]
    }

    fn y(&self) -> i32 {
        self.position[1]
    }

    fn h(&self) -> f64 {
        self.intensity
    }
}

trait TriangleCanvas {
    fn draw_wireframe_triangle(&mut self, p0: IVec2, p1: IVec2, p2: IVec2, color: DVec3);
    fn draw_filled_triangle(&mut self, p0: IVec2, p1: IVec2, p2: IVec2, color: DVec3);
    fn draw_shaded_triangle(
        &mut self,
        p0: ShadedVertex,
        p1: ShadedVertex,
        p2: ShadedVertex,
        color: DVec3,
    );
}

impl TriangleCanvas for RgbImage {
    fn draw_wireframe_triangle(&mut self, p0: IVec2, p1: IVec2, p2: IVec2, color: DVec3) {
        self.draw_line(p0, p1, color);
        self.draw_line(p1, p2, color);
        self.draw_line(p2, p0, color);
    }

    fn draw_filled_triangle(&mut self, mut p0: IVec2, mut p1: IVec2, mut p2: IVec2, color: DVec3) {
        // sort the points by y coordinate
        if p1[1] < p0[1] {
            mem::swap(&mut p1, &mut p0);
        }
        if p2[1] < p0[1] {
            mem::swap(&mut p2, &mut p0);
        }
        if p2[1] < p1[1] {
            mem::swap(&mut p2, &mut p1);
        }

        let mut x01 = interpolate(p0[1], p0[0] as f64, p1[1], p1[0] as f64);
        let mut x12 = interpolate(p1[1], p1[0] as f64, p2[1], p2[0] as f64);
        let x02 = interpolate(p0[1], p0[0] as f64, p2[1], p2[0] as f64);

        // The calculation duplicates the x-coordinate for y1
        x01.pop();
        x01.append(&mut x12);

        let m = x01.len() / 2;
        let (x_left, x_right) = if x02[m] < x01[m] {
            (x02, x01)
        } else {
            (x01, x02)
        };

        for y in p0[1]..=p2[1] {
            let i = (y - p0[1]) as usize;
            let x0 = x_left[i] as i32;
            let x1 = x_right[i] as i32;
            for x in x0..=x1 {
                self.put_canvas_pixel(x, y, color);
            }
        }
    }

    fn draw_shaded_triangle(
        &mut self,
        mut p0: ShadedVertex,
        mut p1: ShadedVertex,
        mut p2: ShadedVertex,
        color: DVec3,
    ) {
        if p1.y() < p0.y() {
            mem::swap(&mut p1, &mut p0);
        }
        if p2.y() < p0.y() {
            mem::swap(&mut p2, &mut p0);
        }
        if p2.y() < p1.y() {
            mem::swap(&mut p2, &mut p1);
        }

        let mut x01 = interpolate(p0.y(), p0.x() as f64, p1.y(), p1.x() as f64);
        let mut h01 = interpolate(p0.y(), p0.h(), p1.y(), p1.h());

        let mut x12 = interpolate(p1.y(), p1.x() as f64, p2.y(), p2.x() as f64);
        let mut h12 = interpolate(p1.y(), p1.h(), p2.y(), p2.h());

        let x02 = interpolate(p0.y(), p0.x() as f64, p2.y(), p2.x() as f64);
        let h02 = interpolate(p0.y(), p0.h(), p2.y(), p2.h());

        x01.pop();
        x01.append(&mut x12);

        h01.pop();
        h01.append(&mut h12);

        let m = x01.len() / 2;
        let (x_left, h_left, x_right, h_right) = if x02[m] < x01[m] {
            (x02, h02, x01, h01)
        } else {
            (x01, h01, x02, h02)
        };

        for y in p0.y()..=p2.y() {
            let i = (y - p0.y()) as usize;
            let x_l = x_left[i].floor() as i32;
            let x_r = x_right[i].floor() as i32;

            let h_segment = interpolate(x_l, h_left[i], x_r, h_right[i]);

            for x in x_l..=x_r {
                let j = (x - x_l) as usize;
                self.put_canvas_pixel(x, y, color * h_segment[j]);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct Triangle {
    vertices: [usize; 3],
    color: DVec3,
}

impl Triangle {
    fn render(&self, canvas: &mut RgbImage, projected: &[IVec2]) {
        canvas.draw_wireframe_triangle(
            projected[self.vertices[0]],
            projected[self.vertices[1]],
            projected[self.vertices[2]],
            self.color,
        );
    }
}

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
