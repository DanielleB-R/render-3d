use glam::{DVec3, IVec2};
use image::{ImageBuffer, RgbImage};
use std::mem;

use render_3d::camera::Viewport;
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

fn main() {
    let mut buffer: RgbImage = ImageBuffer::new(512, 512);
    let viewport: Viewport = Default::default();

    // buffer.draw_wireframe_triangle(
    //     IVec2::new(-200, -250),
    //     IVec2::new(200, 50),
    //     IVec2::new(20, 250),
    //     DVec3::new(255.0, 255.0, 255.0),
    // );

    // buffer.draw_shaded_triangle(
    //     ShadedVertex::new(-200, -250, 0.0),
    //     ShadedVertex::new(200, 50, 0.5),
    //     ShadedVertex::new(20, 250, 1.0),
    //     DVec3::new(0.0, 255.0, 0.0),
    // );

    let vAf = DVec3::new(-2.0, -0.5, 5.0);
    let vBf = DVec3::new(-2.0, 0.5, 5.0);
    let vCf = DVec3::new(-1.0, 0.5, 5.0);
    let vDf = DVec3::new(-1.0, -0.5, 5.0);

    let vAb = DVec3::new(-2.0, -0.5, 6.0);
    let vBb = DVec3::new(-2.0, 0.5, 6.0);
    let vCb = DVec3::new(-1.0, 0.5, 6.0);
    let vDb = DVec3::new(-1.0, -0.5, 6.0);

    let blue = DVec3::new(0.0, 0.0, 255.0);
    let green = DVec3::new(0.0, 255.0, 0.0);
    let red = DVec3::new(255.0, 0.0, 0.0);

    buffer.draw_line(
        viewport.project_vertex(&buffer, vAf),
        viewport.project_vertex(&buffer, vBf),
        blue,
    );
    buffer.draw_line(
        viewport.project_vertex(&buffer, vBf),
        viewport.project_vertex(&buffer, vCf),
        blue,
    );
    buffer.draw_line(
        viewport.project_vertex(&buffer, vCf),
        viewport.project_vertex(&buffer, vDf),
        blue,
    );
    buffer.draw_line(
        viewport.project_vertex(&buffer, vDf),
        viewport.project_vertex(&buffer, vAf),
        blue,
    );

    buffer.draw_line(
        viewport.project_vertex(&buffer, vAb),
        viewport.project_vertex(&buffer, vBb),
        red,
    );
    buffer.draw_line(
        viewport.project_vertex(&buffer, vBb),
        viewport.project_vertex(&buffer, vCb),
        red,
    );
    buffer.draw_line(
        viewport.project_vertex(&buffer, vCb),
        viewport.project_vertex(&buffer, vDb),
        red,
    );
    buffer.draw_line(
        viewport.project_vertex(&buffer, vDb),
        viewport.project_vertex(&buffer, vAb),
        red,
    );

    buffer.draw_line(
        viewport.project_vertex(&buffer, vAb),
        viewport.project_vertex(&buffer, vAf),
        green,
    );
    buffer.draw_line(
        viewport.project_vertex(&buffer, vBb),
        viewport.project_vertex(&buffer, vBf),
        green,
    );
    buffer.draw_line(
        viewport.project_vertex(&buffer, vCb),
        viewport.project_vertex(&buffer, vCf),
        green,
    );
    buffer.draw_line(
        viewport.project_vertex(&buffer, vDb),
        viewport.project_vertex(&buffer, vDf),
        green,
    );

    buffer.save("raster.png").unwrap();
}
