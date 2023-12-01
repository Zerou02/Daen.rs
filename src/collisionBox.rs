use crate::{
    point::{self, Point},
    utils::screenToCartesianY,
};

#[derive(Debug)]
pub enum CollisionBoxTypes {
    AABB,
    Circle,
    Line,
}

pub struct CollisionBox {
    pub colType: CollisionBoxTypes,
    pub points: Vec<Point>,
    pub values: Vec<f64>,
    id: u64,
    collidedWith: Vec<u64>,
}

impl CollisionBox {
    pub fn new(
        colType: CollisionBoxTypes,
        points: Vec<Point>,
        values: Vec<f64>,
        id: u64,
    ) -> CollisionBox {
        return CollisionBox {
            colType,
            points,
            values,
            collidedWith: vec![],
            id,
        };
    }

    pub fn newCircle(points: Vec<Point>, values: Vec<f64>, id: u64) -> CollisionBox {
        return CollisionBox {
            colType: CollisionBoxTypes::Circle,
            points,
            values,
            collidedWith: vec![],
            id,
        };
    }

    fn testCircleCollision(&self, p1: &Point, r1: f64, p2: &Point, r2: f64) -> bool {
        return p1.distanceTo(&p2) < (r1 + r2);
    }

    fn testCircleLineCollision(&self, cP: &Point, r: f64, lP1: &Point, lP2: &Point) -> bool {
        let dx = lP2.x - lP1.x;
        let dy = lP2.y - lP1.y;
        let n = -(dy * lP1.x) - (dx * screenToCartesianY(lP1.y));
        let y = screenToCartesianY(cP.y);
        let d = (dy * cP.x + dx * y + n).abs() / (dy * dy + dx * dx).sqrt();
        return d < r;
    }

    pub fn moveI(self: &mut Self, x: i64, y: i64) {
        self.moveF(x as f64, y as f64);
    }

    pub fn moveF(self: &mut Self, x: f64, y: f64) {
        for p in &mut self.points {
            p.x += x;
            p.y += y;
        }
    }

    pub fn setParameter(&mut self, points: Vec<Point>, values: Vec<f64>) {
        self.points = points;
        self.values = values;
    }

    pub fn collides(&self, colBox: &CollisionBox) -> bool {
        if (self.collidedWith.contains(&colBox.getId())) {
            return false;
        }
        match self.colType {
            CollisionBoxTypes::AABB => {
                return false;
            }
            CollisionBoxTypes::Circle => match colBox.colType {
                CollisionBoxTypes::AABB => {
                    return false;
                }
                CollisionBoxTypes::Circle => {
                    return self.testCircleCollision(
                        &self.points[0],
                        self.values[0],
                        &colBox.points[0],
                        colBox.values[0],
                    );
                }
                CollisionBoxTypes::Line => {
                    return self.testCircleLineCollision(
                        &self.points[0],
                        self.values[0],
                        &colBox.points[0],
                        &colBox.points[1],
                    );
                }
            },
            CollisionBoxTypes::Line => match colBox.colType {
                CollisionBoxTypes::AABB => {
                    return false;
                }
                CollisionBoxTypes::Line => {
                    return false;
                }
                CollisionBoxTypes::Circle => {
                    return self.testCircleLineCollision(
                        &colBox.points[0],
                        colBox.values[0],
                        &self.points[0],
                        &self.points[1],
                    );
                }
            },
        };
        return true;
    }

    pub fn setToCollidedWith(&mut self, id: u64) {
        self.collidedWith.push(id);
    }

    pub fn clearCollidedWith(&mut self) {
        self.collidedWith = vec![];
    }

    pub fn getId(&self) -> u64 {
        return self.id;
    }
}
