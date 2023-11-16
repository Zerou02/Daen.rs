#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn distanceTo(self: &Self, point: &Point) -> f64 {
        let cSquared = (point.x as f64 - self.x as f64).abs()
            * (point.x as f64 - self.x as f64).abs()
            + (point.y as f64 - self.y as f64).abs() * (point.y as f64 - self.y as f64).abs();
        return cSquared.sqrt();
    }
}
