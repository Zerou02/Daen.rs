use crate::{
    ellipsis,
    point::{self, Point},
    utils::screenToCartesianY,
    vector2::Vector2,
};

#[derive(Debug)]
pub enum CollisionBoxTypes {
    AABB,
    Circle,
    Line,
    Ellipsis,
}
#[derive(Debug)]
pub struct CollisionBox {
    pub colType: CollisionBoxTypes,
    pub points: Vec<Point>,
    pub values: Vec<f64>,
    id: String,
    pub collidedWith: Vec<String>,
    pub collisionClass: String,
    pub collidesWith: Vec<String>,
}

impl CollisionBox {
    pub fn new(
        colType: CollisionBoxTypes,
        points: Vec<Point>,
        values: Vec<f64>,
        id: String,
        colClass: String,
        collidesWith: Vec<String>,
    ) -> CollisionBox {
        return CollisionBox {
            colType,
            points,
            values,
            collidedWith: vec![],
            id,
            collisionClass: colClass,
            collidesWith,
        };
    }

    pub fn newCircle(
        points: Vec<Point>,
        values: Vec<f64>,
        id: String,
        colClass: String,
        collidesWith: Vec<String>,
    ) -> CollisionBox {
        return CollisionBox {
            colType: CollisionBoxTypes::Circle,
            points,
            values,
            collidedWith: vec![],
            id,
            collidesWith,
            collisionClass: colClass,
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

    fn testEllipsisCircleCollision(
        &self,
        leftEllipsisPoint: Point,
        rightEllipsisPoint: Point,
        circleCentre: Point,
        ellipsisDist: f64,
        circleRadius: f64,
    ) -> bool {
        let mut lineVec: Vector2;
        let lineBasePoint: Vector2;
        //TODO: Steigung Inf beachten
        if (leftEllipsisPoint.distanceTo(&circleCentre)
            < rightEllipsisPoint.distanceTo(&circleCentre))
        {
            lineBasePoint = leftEllipsisPoint.toVec();
        } else {
            lineBasePoint = rightEllipsisPoint.toVec();
        }
        lineVec = circleCentre.toVec().subtract(&lineBasePoint);
        lineVec.normalize();
        let mut collides = false;
        let dx = (circleCentre.x - lineBasePoint.x);
        let amountSteps = dx.abs();
        let isNegative = if (dx < 0.0) { -1.0 } else { 1.0 };
        for i in 0..amountSteps as usize {
            let y = (amountSteps * lineVec.y) + lineBasePoint.y;
            let x = lineBasePoint.x + amountSteps * isNegative;
            let pointToCheck = Point::new(x, y);
            let isInEllipsis = (leftEllipsisPoint.distanceTo(&pointToCheck)
                + rightEllipsisPoint.distanceTo(&pointToCheck)
                - ellipsisDist)
                .abs()
                < 2.0;

            let isInCircle = circleCentre.distanceTo(&pointToCheck) < circleRadius;

            if (isInCircle && isInEllipsis) {
                collides = true;
                break;
            }
        }
        return collides;
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
                CollisionBoxTypes::Ellipsis => {
                    return self.testEllipsisCircleCollision(
                        colBox.points[0],
                        colBox.points[1],
                        self.points[0],
                        colBox.values[0],
                        self.values[0],
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
                CollisionBoxTypes::Ellipsis => {
                    return false;
                }
            },
            CollisionBoxTypes::Ellipsis => match colBox.colType {
                CollisionBoxTypes::AABB => {
                    return false;
                }
                CollisionBoxTypes::Circle => {
                    return false;
                }
                CollisionBoxTypes::Ellipsis => {
                    return false;
                }
                CollisionBoxTypes::Line => {
                    return false;
                }
                CollisionBoxTypes::Ellipsis => {
                    return self.testEllipsisCircleCollision(
                        self.points[0],
                        self.points[1],
                        colBox.points[0],
                        self.values[0],
                        colBox.values[0],
                    );
                }
            },
        };
        return true;
    }

    pub fn setToCollidedWith(&mut self, id: String) {
        self.collidedWith.push(id);
    }

    pub fn clearCollidedWith(&mut self) {
        self.collidedWith = vec![];
    }

    pub fn getId(&self) -> String {
        return self.id.clone();
    }
}
