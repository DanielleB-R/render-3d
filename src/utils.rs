pub fn interpolate(i0: i32, d0: f64, i1: i32, d1: f64) -> Vec<f64> {
    let mut values = vec![];

    let a = (d1 - d0) / (i1 - i0) as f64;
    let mut d = d0;

    for _ in i0..=i1 {
        values.push(d);
        d += a;
    }

    values
}

type AttributePoint = (i32, f64);

pub fn map_triangle_attribute(
    p0: AttributePoint,
    p1: AttributePoint,
    p2: AttributePoint,
) -> (Vec<f64>, Vec<f64>) {
    let mut d01 = interpolate(p0.0, p0.1, p1.0, p1.1);
    let mut d12 = interpolate(p1.0, p1.1, p2.0, p2.1);
    let d02 = interpolate(p0.0, p0.1, p2.0, p2.1);

    d01.pop();
    d01.append(&mut d12);

    (d01, d02)
}
