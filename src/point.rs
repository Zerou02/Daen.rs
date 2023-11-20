#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn newI(x: i32, y: i32) -> Point {
        return Point {
            x: x as f64,
            y: y as f64,
        };
    }
    pub fn distanceTo(self: &Self, point: &Point) -> f64 {
        let cSquared = (point.x as f64 - self.x as f64).abs()
            * (point.x as f64 - self.x as f64).abs()
            + (point.y as f64 - self.y as f64).abs() * (point.y as f64 - self.y as f64).abs();
        return cSquared.sqrt();
    }
}
