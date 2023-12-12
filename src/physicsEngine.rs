use std::f32::consts::E;

use crate::{
    collisionBox::CollisionBoxTypes,
    ellipsis,
    gameObj::{self, IGameObj},
    matrix::{self, Matrix},
    point::Point,
    utils::rotatePoint,
    vector2::Vector2,
};

pub struct PhysicsEngine {
    gravity: f64,
}

impl PhysicsEngine {
    pub fn new() -> PhysicsEngine {
        return PhysicsEngine { gravity: 6.0 };
    }

    pub fn applyPhysics(&self, gameobjs: &mut Vec<Box<dyn IGameObj>>) {
        for i in 0..gameobjs.len() {
            let vel = gameobjs[i].getVelocity();
            let isZeroX = vel.x.round() == 0.0;
            gameobjs[i].setMovesLeft(if (isZeroX) {
                vel.y.abs().round()
            } else {
                vel.x.abs().round()
            } as i32);
            let mut yStep = vel.y / (vel.x).abs();
            let mut xStep = if (vel.x < 0.0) { -1.0 } else { 1.0 };
            if (isZeroX) {
                xStep = 0.0;
                yStep = if (vel.y < 0.0) { -1.0 } else { 1.0 }
            }
            while gameobjs[i].getMovesLeft() > 0 {
                gameobjs[i].moveF(xStep, yStep);
                let left = gameobjs[i].getMovesLeft() - 1;
                gameobjs[i].setMovesLeft(left);
                let colRetVal = self.doesObjCollide(&gameobjs[i], gameobjs);
                if (colRetVal.0) {
                    gameobjs[i].moveF(-xStep, -yStep);
                    gameobjs[i].setMovesLeft(0);

                    let mut j = 0;
                    for k in 0..gameobjs.len() {
                        if colRetVal.1 == gameobjs[k].getID() {
                            j = k;
                            break;
                        }
                    }
                    let jId = gameobjs[j].getColBoxMut().getId();
                    let iId = gameobjs[i].getColBoxMut().getId();
                    gameobjs[i].getColBoxMut().setToCollidedWith(jId);
                    gameobjs[j].getColBoxMut().setToCollidedWith(iId);

                    let velI = gameobjs[i].getVelocity();
                    let velJ = gameobjs[j].getVelocity();
                    //handle Collision
                    match gameobjs[i].getColBox().colType {
                        CollisionBoxTypes::AABB => {}
                        CollisionBoxTypes::Circle => match gameobjs[j].getColBox().colType {
                            CollisionBoxTypes::AABB => {}
                            CollisionBoxTypes::Circle => {
                                let secondPoint = gameobjs[j].getColBoxMut().points[0];
                                let sp = gameobjs[i].getColBoxMut().points[0]
                                    .centreTo(&secondPoint)
                                    .toVec();
                                let lineVec = sp.subtract(&secondPoint.toVec());
                                let velI = self.handleCollisionLineCircle(
                                    &mut gameobjs[i],
                                    sp,
                                    lineVec,
                                    true,
                                );
                                let velJ = self.handleCollisionLineCircle(
                                    &mut gameobjs[j],
                                    sp,
                                    lineVec,
                                    true,
                                );
                                gameobjs[i].setVelocity(velI);
                                gameobjs[j].setVelocity(velJ);
                            }
                            CollisionBoxTypes::Line => {
                                let lineBase = &gameobjs[j].getColBox().points[0].toVec();
                                let lineVec =
                                    gameobjs[j].getColBox().points[1].toVec().subtract(lineBase);
                                let newVel = self.handleCollisionLineCircle(
                                    &mut gameobjs[i],
                                    lineBase.clone(),
                                    lineVec,
                                    false,
                                );
                                gameobjs[i].setVelocity(newVel);
                            }
                            CollisionBoxTypes::Ellipsis => {
                                let points = &gameobjs[j].getColBoxMut().points.clone();
                                let eDist = gameobjs[j].getColBoxMut().values[0];
                                let (lineBase, lineVec) = self.handleCircleEllipsisCollision(
                                    &mut gameobjs[i],
                                    points[0],
                                    points[1],
                                    eDist,
                                );
                                let newVel = self.handleCollisionLineCircle(
                                    &mut gameobjs[i],
                                    lineBase,
                                    lineVec,
                                    false,
                                );
                                gameobjs[i].setVelocity(newVel);
                            }
                        },
                        CollisionBoxTypes::Line => match gameobjs[j].getColBox().colType {
                            CollisionBoxTypes::AABB => {}
                            CollisionBoxTypes::Circle => {
                                let lineBase = &gameobjs[i].getColBox().points[0].toVec();
                                let lineVec =
                                    gameobjs[i].getColBox().points[1].toVec().subtract(lineBase);
                                let newVel = self.handleCollisionLineCircle(
                                    &mut gameobjs[j],
                                    lineBase.clone(),
                                    lineVec,
                                    false,
                                );
                                gameobjs[j].setVelocity(newVel);
                            }
                            CollisionBoxTypes::Line => {}
                            CollisionBoxTypes::Ellipsis => todo!(),
                        },
                        CollisionBoxTypes::Ellipsis => match gameobjs[j].getColBox().colType {
                            CollisionBoxTypes::Circle => {
                                let points = &gameobjs[i].getColBoxMut().points.clone();
                                let eDist = gameobjs[i].getColBoxMut().values[0];
                                let (lineBase, lineVec) = self.handleCircleEllipsisCollision(
                                    &mut gameobjs[j],
                                    points[0],
                                    points[1],
                                    eDist,
                                );
                                let newVel = self.handleCollisionLineCircle(
                                    &mut gameobjs[j],
                                    lineBase,
                                    lineVec,
                                    false,
                                );
                                gameobjs[j].setVelocity(newVel);
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
        for x in gameobjs {
            x.getColBoxMut().clearCollidedWith();
        }
        //self.checkCollisions(&mut gameobjs);

        //gameobj.moveF(0.0, self.gravity);
    }

    /**
     * (hasCollided,id of otherObj)
     */
    pub fn doesObjCollide(
        &self,
        obj1: &Box<dyn IGameObj>,
        gameobjs: &Vec<Box<dyn IGameObj>>,
    ) -> (bool, String) {
        let mut collided = false;
        let mut id: String = "".to_owned();
        for i in gameobjs {
            if (obj1.getID() != i.getID()
                && obj1
                    .getColBox()
                    .collidesWith
                    .contains(&i.getColBox().collisionClass))
            {
                if (obj1.getColBox().collides(i.getColBox())) {
                    collided = true;
                    id = i.getID();
                    break;
                }
            }
        }
        return (collided, id);
    }

    pub fn handleCollisionLineCircle(
        &self,
        circle: &mut Box<dyn IGameObj>,
        lineBase: Vector2,
        lineVec: Vector2,
        baseIsSp: bool,
    ) -> Vector2 {
        let centre = circle.getColBoxMut().points[0];
        let velI = circle.getVelocity();
        let mut velJ = lineVec;
        if (velI.y < 0.0) {
            velJ.x = -1.0 * lineVec.x;
        }
        if (velI.x > 0.0) {
            velJ.y = -1.0 * lineVec.y;
        }
        let baseI = circle.getColBoxMut().points[0].toVec();
        let baseJ = lineBase;
        let bVec = baseJ.subtract(&baseI);
        let mut b = Matrix::new(2, 2);
        b.addVec(0, bVec);
        let mut matrix = Matrix::new(2, 2);
        matrix.addVec(0, velI);
        matrix.addVec(1, velJ.reverse());
        let angle = velI.angleTo(&velJ).round();
        let result = self.gaussianElimination(&mut matrix, &mut b);
        let mut sp = Point::new(
            baseI.x + velI.x * result.getEntry(0, 0),
            baseI.y + velI.y * result.getEntry(0, 0),
        );
        if (baseIsSp) {
            sp = lineBase.toPoint();
        }
        let newVel = rotatePoint(
            &circle.getVelocity().toPoint(),
            2.0 * angle.to_radians(),
            &Point::newI(0, 0),
        );
        return newVel.toVec();
    }

    pub fn handleCircleEllipsisCollision(
        &self,
        circle: &mut Box<dyn IGameObj>,
        leftFocalPoint: Point,
        rightFocalPoint: Point,
        eDist: f64,
    ) -> (Vector2, Vector2) {
        let mut sp: Point = Point::newI(0, 0);
        let mut circleVel = circle.getVelocity();
        let radius = circle.getColBoxMut().values[0];
        let circleCentre = circle.getColBoxMut().points[0];
        circleVel.normalize();
        for x in -radius as i32..radius as i32 {
            let pY = (x as f64 * circleVel.y) + circleCentre.y;
            let pX = x as f64 * circleVel.x + circleCentre.x;
            let p = Point::new(pX, pY);
            let isSp = (leftFocalPoint.distanceTo(&p) + rightFocalPoint.distanceTo(&p) - eDist)
                .abs()
                < 0.1;
            if (isSp) {
                sp = p.clone();
            }
        }

        let mut currentPoint = sp.clone();
        let mut lastEntry = sp.clone();
        let directionMap = [
            [-1, 0],
            [-1, -1],
            [0, -1],
            [1, -1],
            [1, 0],
            [1, 1],
            [0, 1],
            [-1, 1],
        ];

        let mut distVec: Vec<f64> = Vec::with_capacity(8);
        //checkAllIndices
        for x in directionMap {
            let p = Point::new(currentPoint.x + x[0] as f64, currentPoint.y + x[1] as f64);
            distVec.push(
                ((p.distanceTo(&leftFocalPoint) + p.distanceTo(&rightFocalPoint)) - eDist as f64)
                    .abs(),
            );
        }
        //eval
        let mut index = 9999;
        let mut min = 99999.0;
        for (i, x) in distVec.iter().enumerate() {
            if (*x < min
                && !(directionMap[i][0] as f64 + currentPoint.x == lastEntry.x
                    && directionMap[i][1] as f64 + currentPoint.y == lastEntry.y))
            {
                min = *x;
                index = i;
            }
        }
        let mut index2 = 9999;
        min = 999999.0;
        for (i, x) in distVec.iter().enumerate() {
            if (*x <= min
                && !(directionMap[i][0] as f64 + currentPoint.x == lastEntry.x
                    && directionMap[i][1] as f64 + currentPoint.y == lastEntry.y)
                && i != index)
            {
                min = *x;
                index2 = i;
            }
        }
        let mut test = currentPoint.clone();
        currentPoint.movePoint(directionMap[index][0] as f64, directionMap[index][1] as f64);
        let cp2 = currentPoint.clone();
        test.movePoint(
            directionMap[index2][0] as f64,
            directionMap[index2][1] as f64,
        );

        let points = vec![currentPoint, sp, test];
        let mut minX = 99999.0;
        let mut maxX = 0.0;
        let mut maxY = 0.0;
        let mut minY = 9999.0;
        for p in points {
            if (p.x) < minX {
                minX = p.x;
            }
            if (p.x > maxX) {
                maxX = p.x;
            }
            if (p.y > maxY) {
                maxY = p.y;
            }
            if (p.y < minY) {
                minY = p.y;
            }
        }

        let lineBase = Point::new(minX, minY);
        let lineVec = Vector2::new(maxX, maxY).subtract(&lineBase.toVec());
        return (lineBase.toVec(), lineVec);
    }

    pub fn gaussianElimination(&self, a: &mut Matrix, b: &mut Matrix) -> Matrix {
        let j = 0;
        let n = a.getWidth() as i32;
        //TODO: Auf N-Dimensionalitaet erweitern
        if (a.getEntry(j, j) == 0.0) {
            let mut big = 0.0;
            let mut kRow = j;
            for k in (j + 1..=n - 1) {
                if (a.getEntry(k, j).abs() > big) {
                    big = a.getEntry(k, j).abs();
                    kRow = k;
                }
            }
            //swap rows j,kRow
            for l in j..=n - 1 {
                let dum = a.getEntry(j, l);
                a.setEntry(j, l, a.getEntry(kRow, l));
                a.setEntry(kRow, l, dum);
            }
            //swap j,k th e of b
            let dum = b.getEntry(j, 0);
            b.setEntry(j, 0, b.getEntry(kRow, 0));
            b.setEntry(kRow, 0, dum);
        }
        let pivot = a.getEntry(j, j);
        if (pivot == 0.0) {
            panic!()
        }
        for i in j + 1..=n - 1 {
            let mult = a.getEntry(i, j) / pivot;
            for l in j..=n - 1 {
                a.setEntry(i, l, a.getEntry(i, l) - mult * a.getEntry(j, l));
            }
            b.setEntry(i, 0, b.getEntry(i, 0) - mult * b.getEntry(j, 0));
        }
        return self.backSubstitution(a, b);
    }

    pub fn backSubstitution(&self, u: &mut Matrix, c: &mut Matrix) -> Matrix {
        let n = u.getWidth();
        let mut x = Matrix::new(1, n);
        for i in (0..=n - 1).rev() {
            let mut sum = 0.0;
            for j in i + 1..=n - 1 {
                sum = sum + x.getEntry(j, 0) * u.getEntry(i, j);
            }
            x.setEntry(i, 0, 1.0 / u.getEntry(i, i) * (c.getEntry(i, 0) - sum));
        }
        return x;
    }
}
