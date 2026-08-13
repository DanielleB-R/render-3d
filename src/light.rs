use glam::DVec3;
use serde::Deserialize;

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
    fn compute_lighting(&self, point: DVec3, normal: DVec3, view: DVec3, specular: i32) -> f64 {
        if let Self::Ambient { intensity } = self {
            return *intensity;
        }

        let (l, intensity) = match self {
            Self::Directional {
                intensity,
                direction,
            } => (*direction, intensity),
            Self::Point {
                intensity,
                position,
            } => (*position - point, intensity),
            _ => unreachable!(),
        };

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
