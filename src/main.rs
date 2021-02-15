use std::convert::{From, Into};

#[derive(Debug, Default)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    fn new(x: f64, y: f64) -> Point2D {
        Self { x, y }
    }
}

impl Into<f64> for Point2D {
    fn into(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl From<&Point3D> for Point2D {
    fn from(p3: &Point3D) -> Self {
        Self { x: p3.x, y: p3.y }
    }
}

#[derive(Debug, Default)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Point3D {
        Self { x, y, z }
    }
}

impl Into<f64> for Point3D {
    fn into(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl From<&Point2D> for Point3D {
    fn from(p2: &Point2D) -> Self {
        Self {
            x: p2.x,
            y: p2.y,
            z: 0.0,
        }
    }
}

fn main() {
    let p2 = Point2D::new(3.0, 4.0);

    // One way to convert a Point2D to a Point3D.
    let p3 = Point3D::from(&p2);
    println!("p2 to 3D = {:?}", p3);

    // Another way to convert a Point2D to a Point3D.
    // Parens are required around the reference because the
    // dot operator for the call to "into" has higher precedence.
    let p3: Point3D = (&p2).into();
    println!("p2 to 3D = {:?}", p3);

    let distance: f64 = p2.into();
    println!("p2 distance from origin = {}", distance);

    let p3 = Point3D::new(3.0, 4.0, 5.0);

    // One way to convert a Point3D to a Point2D.
    let p2 = Point2D::from(&p3);
    println!("p3 to 2D = {:?}", p2);

    // Another way to convert a Point3D to a Point2D.
    let p2: Point2D = (&p3).into();
    println!("p3 to 2D = {:?}", p2);

    let distance: f64 = p3.into();
    println!("p3 distance from origin = {}", distance);
}
