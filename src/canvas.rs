use glam::DVec3;

// An extension trait to use the conventions in the book for putting pixels.
// This includes a coordinate system centered in the image, y-axis up, and
// treating colours as floating-point vectors rather than u8 arrays
pub trait SymmetricCanvas {
    fn put_canvas_pixel(&mut self, cx: i32, cy: i32, color: DVec3);
}

impl SymmetricCanvas for image::RgbImage {
    fn put_canvas_pixel(&mut self, cx: i32, cy: i32, color: DVec3) {
        let x = ((self.width() / 2) as i32 + cx) as u32;
        let y = ((self.height() / 2) as i32 - cy) as u32 - 1;
        let color_data = color
            .clamp(DVec3::splat(0.0), DVec3::splat(255.0))
            .to_array()
            .map(|f| f as u8);
        self.put_pixel(x, y, color_data.into());
    }
}
