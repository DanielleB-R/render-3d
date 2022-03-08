use crate::utils::interpolate;
use glam::{DVec3, IVec2};
use image::RgbImage;
use std::mem;

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

pub trait TriangleCanvas {
    fn draw_wireframe_triangle(&mut self, p0: IVec2, p1: IVec2, p2: IVec2, color: DVec3);
    fn draw_filled_triangle(&mut self, p0: IVec2, p1: IVec2, p2: IVec2, color: DVec3);
    // fn draw_shaded_triangle(
    //     &mut self,
    //     p0: ShadedVertex,
    //     p1: ShadedVertex,
    //     p2: ShadedVertex,
    //     color: DVec3,
    // );
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

    // fn draw_shaded_triangle(
    //     &mut self,
    //     mut p0: ShadedVertex,
    //     mut p1: ShadedVertex,
    //     mut p2: ShadedVertex,
    //     color: DVec3,
    // ) {
    //     if p1.y() < p0.y() {
    //         mem::swap(&mut p1, &mut p0);
    //     }
    //     if p2.y() < p0.y() {
    //         mem::swap(&mut p2, &mut p0);
    //     }
    //     if p2.y() < p1.y() {
    //         mem::swap(&mut p2, &mut p1);
    //     }

    //     let mut x01 = interpolate(p0.y(), p0.x() as f64, p1.y(), p1.x() as f64);
    //     let mut h01 = interpolate(p0.y(), p0.h(), p1.y(), p1.h());

    //     let mut x12 = interpolate(p1.y(), p1.x() as f64, p2.y(), p2.x() as f64);
    //     let mut h12 = interpolate(p1.y(), p1.h(), p2.y(), p2.h());

    //     let x02 = interpolate(p0.y(), p0.x() as f64, p2.y(), p2.x() as f64);
    //     let h02 = interpolate(p0.y(), p0.h(), p2.y(), p2.h());

    //     x01.pop();
    //     x01.append(&mut x12);

    //     h01.pop();
    //     h01.append(&mut h12);

    //     let m = x01.len() / 2;
    //     let (x_left, h_left, x_right, h_right) = if x02[m] < x01[m] {
    //         (x02, h02, x01, h01)
    //     } else {
    //         (x01, h01, x02, h02)
    //     };

    //     for y in p0.y()..=p2.y() {
    //         let i = (y - p0.y()) as usize;
    //         let x_l = x_left[i].floor() as i32;
    //         let x_r = x_right[i].floor() as i32;

    //         let h_segment = interpolate(x_l, h_left[i], x_r, h_right[i]);

    //         for x in x_l..=x_r {
    //             let j = (x - x_l) as usize;
    //             self.put_canvas_pixel(x, y, color * h_segment[j]);
    //         }
    //     }
    // }
}
