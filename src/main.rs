use image::{ImageBuffer, Rgb, RgbImage};

fn main() {
    let mut buffer: RgbImage = ImageBuffer::new(512, 512);

    for i in 0..512 {
        buffer.put_pixel(i, i, Rgb([255, 255, 255]))
    }

    buffer.save("test.png").unwrap();
}
