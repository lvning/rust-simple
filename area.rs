pub trait NormalShape {
    fn clac_shape_area(&self) -> f64;
}
fn calc_area<T: NormalShape>(shape: &T) {
    println!("Calculate area is: {}", shape.clac_shape_area());
}

struct Rectangle {
    length: f64,
    width: f64,
}

impl NormalShape for Rectangle {
    fn clac_shape_area(&self) -> f64 {
        &self.width * &self.length
    }
}

struct Triangle {
    base: f64,
    height: f64,
}

impl NormalShape for Triangle {
    fn clac_shape_area(&self) -> f64 {
        (&self.base * &self.height) / 2.0
    }
}

struct Circle {
    radius: f64,
}

impl NormalShape for Circle {
    fn clac_shape_area(&self) -> f64 {
        3.14 * &self.radius * &self.radius
    }
}

fn main() {
    calc_area(&Rectangle {
        length: 10.5,
        width: 10.0,
    });
    calc_area(&Triangle {
        base: 12.5,
        height: 10.0,
    });
    calc_area(&Circle { radius: 8.5 })
}
