use glam::Vec3;
use image::{ImageBuffer, RgbImage};
use serde::Deserialize;

type Color = Vec3;

trait SymmetricCanvas {
    fn put_canvas_pixel(&mut self, cx: i32, cy: i32, color: Color);
}

impl SymmetricCanvas for RgbImage {
    fn put_canvas_pixel(&mut self, cx: i32, cy: i32, color: Color) {
        let x = ((self.width() / 2) as i32 + cx) as u32;
        let y = ((self.height() / 2) as i32 - cy) as u32 - 1;
        let color_data = color
            .clamp(Vec3::splat(0.0), Vec3::splat(255.0))
            .to_array()
            .map(|f| f as u8);
        self.put_pixel(x, y, color_data.into());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Viewport {
    width: f32,
    height: f32,
    distance: f32,
}

impl Viewport {
    pub fn from_canvas(&self, canvas: &RgbImage, cx: i32, cy: i32) -> Vec3 {
        Vec3::new(
            cx as f32 * (self.width) / (canvas.width() as f32),
            cy as f32 * (self.height) / (canvas.height() as f32),
            self.distance,
        )
    }
}

fn reflect_ray(ray: Vec3, normal: Vec3) -> Vec3 {
    2.0 * normal.dot(ray) * normal - ray
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(tag = "type")]
enum Light {
    Ambient { intensity: f32 },
    Directional { intensity: f32, direction: Vec3 },
    Point { intensity: f32, position: Vec3 },
}

impl Light {
    fn compute_lighting(
        &self,
        point: Vec3,
        normal: Vec3,
        view: Vec3,
        specular: i32,
        scene: &Scene,
    ) -> f32 {
        if let Self::Ambient { intensity } = self {
            return *intensity;
        }

        let (l, intensity, max_t) = match self {
            Self::Directional {
                intensity,
                direction,
            } => (*direction, intensity, f32::INFINITY),
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
    center: Vec3,
    radius: f32,
    color: Color,
    specular: i32,
    #[serde(default)]
    reflective: f32,
}

impl Sphere {
    fn intersect_ray_sphere(&self, origin: Vec3, d: Vec3) -> (f32, f32) {
        let co = origin - self.center;

        let a = d.dot(d);
        let b = 2.0 * d.dot(co);
        let c = co.dot(co) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return (f32::INFINITY, f32::INFINITY);
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
}

impl Scene {
    fn light_point(&self, point: Vec3, normal: Vec3, view: Vec3, specular: i32) -> f32 {
        self.lights
            .iter()
            .map(|light| light.compute_lighting(point, normal, view, specular, self))
            .sum()
    }

    fn closest_intersection(
        &self,
        origin: Vec3,
        direction: Vec3,
        min_t: f32,
        max_t: f32,
    ) -> (Option<Sphere>, f32) {
        let mut closest_t = f32::INFINITY;
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

    fn trace_ray(&self, origin: Vec3, direction: Vec3, min_t: f32, max_t: f32) -> Color {
        let (closest_sphere, closest_t) =
            self.closest_intersection(origin, direction, min_t, max_t);

        if closest_sphere.is_none() {
            return self.background;
        }
        let sphere = closest_sphere.unwrap();

        let point = origin + closest_t * direction;
        let normal = (point - sphere.center).normalize();
        let intensity = self.light_point(point, normal, -direction, sphere.specular);
        sphere.color * intensity
    }
}

fn main() {
    let mut buffer: RgbImage = ImageBuffer::new(512, 512);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let viewport = Viewport {
        width: 1.0,
        height: 1.0,
        distance: 1.0,
    };

    let scene: Scene = serde_yaml::from_slice(&std::fs::read("scene.yaml").unwrap()).unwrap();

    for cx in -256..256 {
        for cy in -256..256 {
            let d = viewport.from_canvas(&buffer, cx, cy);
            let color = scene.trace_ray(origin, d, 1.0, f32::INFINITY);
            buffer.put_canvas_pixel(cx, cy, color);
        }
    }

    buffer.save("test.png").unwrap();
}
