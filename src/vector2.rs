use crate::point::Point;

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

    pub fn getLength(&self) -> f64 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }

    pub fn reverse(&self) -> Vector2 {
        return Vector2 {
            x: self.x * -1.0,
            y: self.y * -1.0,
        };
    }

    pub fn scalarProduct(&self, otherVec: &Vector2) -> f64 {
        return self.x * otherVec.x + self.y * otherVec.y;
    }

    pub fn angleTo(&self, otherVec: &Vector2) -> f64 {
        return (self.scalarProduct(otherVec) / (self.getLength() * otherVec.getLength()))
            .acos()
            .to_degrees();
    }

    pub fn subtract(&self, otherVec: &Vector2) -> Vector2 {
        return Vector2::new(self.x - otherVec.x, self.y - otherVec.y);
    }

    pub fn toPoint(&self) -> Point {
        return Point::new(self.x, self.y);
    }

    pub fn normalize(&mut self) {
        let isNegative = if (self.x < 0.0) { -1.0 } else { 1.0 };
        self.y = self.y / self.x * isNegative;
        self.x = self.x / self.x * isNegative;
    }
}
