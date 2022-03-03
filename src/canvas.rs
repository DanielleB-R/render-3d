use glam::DVec3;

// An extension trait to use the conventions in the book for putting pixels.
// This includes a coordinate system centered in the image, y-axis up, and
// treating colours as floating-point vectors rather than u8 arrays
pub trait SymmetricCanvas {
    fn put_canvas_pixel(&mut self, cx: i32, cy: i32, color: DVec3);
}

impl SymmetricCanvas for image::RgbImage {
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
