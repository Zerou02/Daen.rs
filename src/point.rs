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
    pub fn new(x: f64, y: f64) -> Point {
        return Point { x, y };
    }
    pub fn distanceTo(self: &Self, point: &Point) -> f64 {
        let cSquared = (point.x as f64 - self.x as f64).abs()
            * (point.x as f64 - self.x as f64).abs()
            + (point.y as f64 - self.y as f64).abs() * (point.y as f64 - self.y as f64).abs();
        return cSquared.sqrt();
    }

    pub fn centreTo(&self, p: &Point) -> Point {
        let x = (p.x + self.x) / 2.0;
        let y = (p.y + self.y) / 2.0;
        return Point::new(x, y);
    }
    /**
     * Result(0) = leftPoint; Result(1) = rightPoint
     */
    pub fn orderedPoints(&self, p: &Point) -> (Point, Point) {
        let minX = self.x.min(p.x);
        let maxX = self.x.max(p.x);
        let yPoints = if (minX == self.x) {
            (self.y, p.y)
        } else {
            (p.y, self.y)
        };
        return (Point::new(minX, yPoints.0), Point::new(maxX, yPoints.1));
    }

    pub fn movePoint(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn equalTo(&mut self, p: &Point) -> bool {
        return self.x == p.x && self.y == p.y;
    }
}
