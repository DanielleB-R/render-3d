use render_3d::canvas::Canvas;
use render_3d::scene::Scene;

fn main() {
    let mut canvas = Canvas::new(512, 512);

    let scene: Scene = serde_yaml::from_slice(&std::fs::read("cube.yaml").unwrap()).unwrap();

    scene.transform().clip().render(&mut canvas);

    canvas.save("raster.png").unwrap();
}
