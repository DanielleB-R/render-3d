mod camera;
mod canvas;

use crate::camera::Camera;
use crate::canvas::SymmetricCanvas;
use glam::DVec3;
use image::{ImageBuffer, RgbImage};
use serde::Deserialize;

type Color = DVec3;

fn reflect_ray(ray: DVec3, normal: DVec3) -> DVec3 {
    2.0 * normal.dot(ray) * normal - ray
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "type")]
enum Light {
    Ambient { intensity: f64 },
    Directional { intensity: f64, direction: DVec3 },
    Point { intensity: f64, position: DVec3 },
}

impl Light {
    fn compute_lighting(
        &self,
        point: DVec3,
        normal: DVec3,
        view: DVec3,
        specular: i32,
        scene: &Scene,
    ) -> f64 {
        if let Self::Ambient { intensity } = self {
            return *intensity;
        }

        let (l, intensity, max_t) = match self {
            Self::Directional {
                intensity,
                direction,
            } => (*direction, intensity, f64::INFINITY),
            Self::Point {
                intensity,
                position,
            } => (*position - point, intensity, 1.0),
            _ => unreachable!(),
        };

        let (shadow_sphere, _) = scene.closest_intersection(point, l, 0.001, max_t);
        if shadow_sphere.is_some() {
            return 0.0;
        }

        let ndotl = normal.dot(l);
        let diffuse = if ndotl >= 0.0 {
            intensity * ndotl / (normal.length() * l.length())
        } else {
            0.0
        };

        let specular_intensity = if specular != -1 {
            let r = reflect_ray(l, normal);
            let rdotv = r.dot(view);
            if rdotv > 0.0 {
                intensity * (rdotv / (r.length() * view.length())).powi(specular)
            } else {
                0.0
            }
        } else {
            0.0
        };

        diffuse + specular_intensity
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct Sphere {
    center: DVec3,
    radius: f64,
    color: Color,
    specular: i32,
    #[serde(default)]
    reflective: f64,
}

impl Sphere {
    fn intersect_ray_sphere(&self, origin: DVec3, d: DVec3) -> (f64, f64) {
        let co = origin - self.center;

        let a = d.dot(d);
        let b = 2.0 * d.dot(co);
        let c = co.dot(co) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return (f64::INFINITY, f64::INFINITY);
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

        (t1, t2)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Scene {
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
    background: Color,
    camera: Camera,
}

impl Scene {
    fn light_point(&self, point: DVec3, normal: DVec3, view: DVec3, specular: i32) -> f64 {
        self.lights
            .iter()
            .map(|light| light.compute_lighting(point, normal, view, specular, self))
            .sum()
    }

    fn closest_intersection(
        &self,
        origin: DVec3,
        direction: DVec3,
        min_t: f64,
        max_t: f64,
    ) -> (Option<Sphere>, f64) {
        let mut closest_t = f64::INFINITY;
        let mut closest_sphere = None;

        for sphere in &self.spheres {
            let (t1, t2) = sphere.intersect_ray_sphere(origin, direction);
            if t1 <= max_t && t1 >= min_t && t1 < closest_t {
                closest_t = t1;
                closest_sphere = Some(*sphere);
            }
            if t2 <= max_t && t2 >= min_t && t2 < closest_t {
                closest_t = t2;
                closest_sphere = Some(*sphere);
            }
        }
        (closest_sphere, closest_t)
    }

    fn trace_ray(
        &self,
        origin: DVec3,
        direction: DVec3,
        min_t: f64,
        max_t: f64,
        recursion_depth: u8,
    ) -> Color {
        let (closest_sphere, closest_t) =
            self.closest_intersection(origin, direction, min_t, max_t);

        if closest_sphere.is_none() {
            return self.background;
        }
        let sphere = closest_sphere.unwrap();

        let point = origin + closest_t * direction;
        let normal = (point - sphere.center).normalize();
        let intensity = self.light_point(point, normal, -direction, sphere.specular);
        let local_color = sphere.color * intensity;

        if sphere.reflective <= 0.0 || recursion_depth == 0 {
            return local_color;
        }
        let r = reflect_ray(-direction, normal);
        let reflected_color = self.trace_ray(point, r, 0.001, f64::INFINITY, recursion_depth - 1);

        local_color * (1.0 - sphere.reflective) + reflected_color * sphere.reflective
    }
}

fn main() {
    let mut buffer: RgbImage = ImageBuffer::new(512, 512);

    let scene: Scene = serde_yaml::from_slice(&std::fs::read("scene.yaml").unwrap()).unwrap();

    for cx in -256..256 {
        for cy in -256..256 {
            let direction = scene.camera.viewport.direction_from_canvas(&buffer, cx, cy);
            let color = scene.trace_ray(scene.camera.position, direction, 1.0, f64::INFINITY, 3);
            buffer.put_canvas_pixel(cx, cy, color);
        }
    }

    buffer.save("test.png").unwrap();
}
