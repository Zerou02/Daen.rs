#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        return Vector2 { x, y };
    }

    pub fn newI(x: i32, y: i32) -> Vector2 {
        return Vector2 {
            x: x as f64,
            y: y as f64,
        };
    }

    pub fn reverse(&self) -> Vector2 {
        return Vector2 {
            x: self.x * -1.0,
            y: self.y * -1.0,
        };
    }
}
