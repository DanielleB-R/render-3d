use glam::{DVec3, IVec2};
use image::{ImageBuffer, RgbImage};
use std::mem;

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

trait TriangleCanvas {
    fn draw_wireframe_triangle(&mut self, p0: IVec2, p1: IVec2, p2: IVec2, color: DVec3);
    fn draw_filled_triangle(&mut self, p0: IVec2, p1: IVec2, p2: IVec2, color: DVec3);
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
}

fn main() {
    let mut buffer: RgbImage = ImageBuffer::new(512, 512);

    buffer.draw_wireframe_triangle(
        IVec2::new(-200, -250),
        IVec2::new(200, 50),
        IVec2::new(20, 250),
        DVec3::new(255.0, 255.0, 255.0),
    );

    buffer.draw_filled_triangle(
        IVec2::new(-200, -250),
        IVec2::new(200, 50),
        IVec2::new(20, 250),
        DVec3::new(0.0, 255.0, 0.0),
    );

    buffer.save("raster.png").unwrap();
}
