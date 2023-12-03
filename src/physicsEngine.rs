use crate::{
    collisionBox::CollisionBoxTypes,
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
            let isZeroX = vel.x == 0.0;
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
            //     println!("yStep{}{}", xStep, yStep);
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
                    gameobjs[i].getColBoxMut().setToCollidedWith(j as u64);
                    gameobjs[j].getColBoxMut().setToCollidedWith(i as u64);

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
                                //   println!("velJ{:?}", velJ);
                                gameobjs[i].setVelocity(velJ);
                                gameobjs[j].setVelocity(velI);
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
    ) -> (bool, u64) {
        let mut collided = false;
        let mut id = 1;
        for i in gameobjs {
            if (obj1.getID() != i.getID()) {
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
        let velJ = lineVec;
        let baseI = circle.getColBoxMut().points[0].toVec();
        let baseJ = lineBase;
        let bVec = baseJ.subtract(&baseI);
        let mut b = Matrix::new(2, 2);
        b.addVec(0, bVec);
        let mut matrix = Matrix::new(2, 2);
        matrix.addVec(0, velI);
        matrix.addVec(1, velJ.reverse());
        let angle = velI.angleTo(&velJ);
        let result = self.gaussianElimination(&mut matrix, &mut b);
        let mut sp = Point::new(
            baseI.x + velI.x * result.getEntry(0, 0),
            baseI.y + velI.y * result.getEntry(0, 0),
        );
        if (baseIsSp) {
            sp = lineBase.toPoint();
        }
        let mut normalizedAngle = if (angle) < 90.0 { angle } else { 180.0 - angle };
        let origNormalizedAngle = normalizedAngle;
        let rotatedFirst = rotatePoint(&centre, normalizedAngle.to_radians(), &sp);
        let newVelFirst = sp.vecTo(rotatedFirst);
        let nP = rotatedFirst.toVec().subtract(&baseJ);
        let isMultiple = (nP.x / velJ.x) * velJ.y == nP.y;
        if (isMultiple) {
            normalizedAngle = 2.0 * normalizedAngle + 180.0;
        } else {
            normalizedAngle *= 2.0
        }
        println!("angel{}", angle);
        println!("altAnge{}", normalizedAngle);
        let newRotated = rotatePoint(&centre, normalizedAngle.to_radians(), &sp);
        let mut newVel = rotatePoint(
            &circle.getVelocity().toPoint(),
            2.0 * origNormalizedAngle.to_radians(),
            &Point::newI(0, 0),
        );
        println!("newVel{:?}", newVel);
        //let mut newVel = sp.vecTo(newRotated);
        //newVel.normalize();
        return newVel.toVec();
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
