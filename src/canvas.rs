use crate::utils::{interpolate, map_triangle_attribute};
use glam::{DVec3, IVec2};
use image::{ImageBuffer, RgbImage};
use std::mem;

#[derive(Debug, Clone)]
pub struct Canvas {
    image: RgbImage,
    width: u32,
    half_width: i32,
    height: u32,
    half_height: i32,
    depth_buffer: Vec<f64>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            image: ImageBuffer::new(width, height),
            width,
            half_width: (width / 2) as i32,
            height,
            half_height: (height / 2) as i32,
            depth_buffer: vec![0.0; (width * height) as usize],
        }
    }

    fn put_pixel(&mut self, cx: i32, cy: i32, color: DVec3) {
        let x = self.half_width + cx;
        if x < 0 || x >= self.width as i32 {
            return;
        }
        let y = self.half_height - cy - 1;
        if y < 0 || y >= self.height as i32 {
            return;
        }
        let color_data = color
            .clamp(DVec3::splat(0.0), DVec3::splat(255.0))
            .to_array()
            .map(|f| f as u8);
        self.image.put_pixel(x as u32, y as u32, color_data.into());
    }

    pub fn put_depth_pixel(&mut self, cx: i32, cy: i32, depth: f64, color: DVec3) {
        let x = self.half_width + cx;
        if x < 0 || x >= self.width as i32 {
            return;
        }
        let y = self.half_height - cy - 1;
        if y < 0 || y >= self.height as i32 {
            return;
        }
        let buffer_index = (x as u32 + self.width * (y as u32)) as usize;
        if depth < self.depth_buffer[buffer_index] {
            return;
        }
        self.depth_buffer[buffer_index] = depth;
        let color_data = color
            .clamp(DVec3::splat(0.0), DVec3::splat(255.0))
            .to_array()
            .map(|f| f as u8);
        self.image.put_pixel(x as u32, y as u32, color_data.into());
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn draw_line(&mut self, p0: IVec2, p1: IVec2, color: DVec3) {
        if (p1[0] - p0[0]).abs() > (p1[1] - p0[1]).abs() {
            // this line is more horizontal than vertical
            let (pl, pr) = if p0[0] < p1[0] { (p0, p1) } else { (p1, p0) };

            let ys = interpolate(pl[0], pl[1] as f64, pr[0], pr[1] as f64);

            for x in pl[0]..=pr[0] {
                self.put_pixel(x, ys[(x - pl[0]) as usize].floor() as i32, color);
            }
        } else {
            // this line is more vertical than horizontal
            let (pl, pr) = if p0[1] < p1[1] { (p0, p1) } else { (p1, p0) };

            let xs = interpolate(pl[1], pl[0] as f64, pr[1], pr[0] as f64);

            for y in pl[1]..=pr[1] {
                self.put_pixel(xs[(y - pl[1]) as usize].floor() as i32, y, color);
            }
        }
    }

    pub fn draw_wireframe_triangle(&mut self, p0: IVec2, p1: IVec2, p2: IVec2, color: DVec3) {
        self.draw_line(p0, p1, color);
        self.draw_line(p1, p2, color);
        self.draw_line(p2, p0, color);
    }

    pub fn draw_filled_triangle(
        &mut self,
        mut p0: IVec2,
        mut p1: IVec2,
        mut p2: IVec2,
        color: DVec3,
    ) {
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

        let (x012, x02) = map_triangle_attribute(
            (p0[1], p0[0] as f64),
            (p1[1], p1[0] as f64),
            (p2[1], p2[0] as f64),
        );

        let m = x012.len() / 2;
        let (x_left, x_right) = if x02[m] < x012[m] {
            (x02, x012)
        } else {
            (x012, x02)
        };

        for y in p0[1]..=p2[1] {
            let i = (y - p0[1]) as usize;
            let x0 = x_left[i] as i32;
            let x1 = x_right[i] as i32;
            for x in x0..=x1 {
                self.put_pixel(x, y, color);
            }
        }
    }

    pub fn save(&self, path: impl AsRef<std::path::Path>) -> image::ImageResult<()> {
        self.image.save(path)
    }
}
