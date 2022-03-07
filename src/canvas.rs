use crate::utils::interpolate;
use glam::{DVec3, IVec2};
use image::RgbImage;

// An extension trait to use the conventions in the book for putting pixels.
// This includes a coordinate system centered in the image, y-axis up, and
// treating colours as floating-point vectors rather than u8 arrays
pub trait SymmetricCanvas {
    fn put_canvas_pixel(&mut self, cx: i32, cy: i32, color: DVec3);
}

impl SymmetricCanvas for RgbImage {
    fn put_canvas_pixel(&mut self, cx: i32, cy: i32, color: DVec3) {
        let x = (self.width() / 2) as i32 + cx;
        if x < 0 || x >= self.width() as i32 {
            return;
        }
        let y = (self.height() / 2) as i32 - cy - 1;
        if y < 0 || y >= self.height() as i32 {
            return;
        }
        let color_data = color
            .clamp(DVec3::splat(0.0), DVec3::splat(255.0))
            .to_array()
            .map(|f| f as u8);
        self.put_pixel(x as u32, y as u32, color_data.into());
    }
}

pub trait LineCanvas: SymmetricCanvas {
    fn draw_line(&mut self, p0: IVec2, p1: IVec2, color: DVec3);
}

impl LineCanvas for RgbImage {
    fn draw_line(&mut self, p0: IVec2, p1: IVec2, color: DVec3) {
        if (p1[0] - p0[0]).abs() > (p1[1] - p0[1]).abs() {
            // this line is more horizontal than vertical
            let (pl, pr) = if p0[0] < p1[0] { (p0, p1) } else { (p1, p0) };

            let ys = interpolate(pl[0], pl[1] as f64, pr[0], pr[1] as f64);

            for x in pl[0]..=pr[0] {
                self.put_canvas_pixel(x, ys[(x - pl[0]) as usize].floor() as i32, color);
            }
        } else {
            // this line is more vertical than horizontal
            let (pl, pr) = if p0[1] < p1[1] { (p0, p1) } else { (p1, p0) };

            let xs = interpolate(pl[1], pl[0] as f64, pr[1], pr[0] as f64);

            for y in pl[1]..=pr[1] {
                self.put_canvas_pixel(xs[(y - pl[1]) as usize].floor() as i32, y, color);
            }
        }
    }
}
