use glam::{DVec3, IVec2};
use image::{ImageBuffer, RgbImage};

use render_3d::canvas::SymmetricCanvas;

fn interpolate(i0: i32, d0: f64, i1: i32, d1: f64) -> Vec<f64> {
    let mut values = vec![];

    let a = (d1 - d0) / (i1 - i0) as f64;
    let mut d = d0;

    for _ in i0..=i1 {
        values.push(d);
        d += a;
    }

    values
}

trait LineCanvas {
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

fn main() {
    let mut buffer: RgbImage = ImageBuffer::new(512, 512);

    // for cx in -256..256 {
    //     for cy in -256..256 {
    //         buffer.put_canvas_pixel(cx, cy, DVec3::new(0.0, 255.0, 0.0));
    //     }
    // }
    buffer.draw_line(
        IVec2::new(-50, -200),
        IVec2::new(60, 240),
        DVec3::new(255.0, 255.0, 255.0),
    );

    buffer.save("raster.png").unwrap();
}
