use image::{ImageBuffer, RgbImage};

use render_3d::camera::Viewport;
use render_3d::scene::Scene;

// #[derive(Debug, Clone, Copy)]
// struct ShadedVertex {
//     position: IVec2,
//     intensity: f64,
// }

// impl ShadedVertex {
//     fn new(x: i32, y: i32, h: f64) -> Self {
//         Self {
//             position: IVec2::new(x, y),
//             intensity: h,
//         }
//     }

//     fn x(&self) -> i32 {
//         self.position[0]
//     }

//     fn y(&self) -> i32 {
//         self.position[1]
//     }

//     fn h(&self) -> f64 {
//         self.intensity
//     }
// }

fn main() {
    let mut buffer: RgbImage = ImageBuffer::new(512, 512);
    let viewport: Viewport = Default::default();

    let scene: Scene = serde_yaml::from_slice(&std::fs::read("cube.yaml").unwrap()).unwrap();

    scene.render(&mut buffer, &viewport);

    buffer.save("raster.png").unwrap();
}
