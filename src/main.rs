use glam::Vec3;
use image::{ImageBuffer, Rgb, RgbImage};

type Color = Rgb<u8>;

trait SymmetricCanvas {
    fn put_canvas_pixel(&mut self, cx: i32, cy: i32, color: Color);
}

impl SymmetricCanvas for RgbImage {
    fn put_canvas_pixel(&mut self, cx: i32, cy: i32, color: Color) {
        let x = ((self.width() / 2) as i32 + cx) as u32;
        let y = ((self.height() / 2) as i32 - cy) as u32 - 1;
        self.put_pixel(x, y, color);
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

#[derive(Debug, Clone)]
struct Sphere {
    center: Vec3,
    radius: f32,
    color: Color,
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

struct Scene {
    spheres: Vec<Sphere>,
    background: Color,
}

impl Scene {
    fn trace_ray(&self, origin: Vec3, d: Vec3, min_t: f32, max_t: f32) -> Color {
        let mut closest_t = f32::INFINITY;
        let mut pixel_color = self.background;

        for sphere in &self.spheres {
            let (t1, t2) = sphere.intersect_ray_sphere(origin, d);
            if t1 <= max_t && t1 >= min_t && t1 < closest_t {
                closest_t = t1;
                pixel_color = sphere.color;
            }
            if t2 <= max_t && t2 >= min_t && t2 < closest_t {
                closest_t = t2;
                pixel_color = sphere.color;
            }
        }

        pixel_color
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

    let scene = Scene {
        background: Rgb([255, 255, 255]),
        spheres: vec![
            Sphere {
                center: Vec3::new(0.0, -1.0, 3.0),
                radius: 1.0,
                color: Rgb([255, 0, 0]),
            },
            Sphere {
                center: Vec3::new(2.0, 0.0, 4.0),
                radius: 1.0,
                color: Rgb([0, 0, 255]),
            },
            Sphere {
                center: Vec3::new(-2.0, 0.0, 4.0),
                radius: 1.0,
                color: Rgb([0, 255, 0]),
            },
        ],
    };

    for cx in -256..256 {
        for cy in -256..256 {
            let d = viewport.from_canvas(&buffer, cx, cy);
            let color = scene.trace_ray(origin, d, 1.0, f32::INFINITY);
            buffer.put_canvas_pixel(cx, cy, color);
        }
    }

    buffer.save("test.png").unwrap();
}
