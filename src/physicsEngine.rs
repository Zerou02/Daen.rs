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
                vel.y.abs().floor()
            } else {
                vel.x.abs().floor()
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
                                gameobjs[i].setVelocity(velJ);
                                gameobjs[j].setVelocity(velI);
                            }
                            CollisionBoxTypes::Line => {
                                let centre = gameobjs[i].getColBoxMut().points[0];
                                let velI = gameobjs[i].getVelocity();
                                let velJ = gameobjs[j].getColBoxMut().points[0]
                                    .toVec()
                                    .subtract(&gameobjs[j].getColBoxMut().points[1].toVec());
                                let baseI = gameobjs[i].getColBoxMut().points[0].toVec();
                                let baseJ = gameobjs[j].getColBoxMut().points[0].toVec();
                                let bVec = baseJ.subtract(&baseI);
                                let mut b = Matrix::new(2, 2);
                                b.addVec(0, bVec);
                                let mut matrix = Matrix::new(2, 2);
                                matrix.addVec(0, velI);
                                matrix.addVec(1, velJ.reverse());
                                let angle = velI.angleTo(&velJ);
                                println!("firstBase{:?}", baseI);
                                println!("secondBase{:?}", baseJ);

                                println!("firstVec {:?}", velI);
                                println!("secondVec{:?}", velJ);
                                let result = self.gaussianElimination(&mut matrix, &mut b);
                                let sp = Point::new(
                                    baseI.x + velI.x * result.getEntry(0, 0),
                                    baseI.y + velI.y * result.getEntry(0, 0),
                                );
                                println!("Point1:{:?}", sp);
                                println!(
                                    "Point2:{:?}",
                                    Point::new(
                                        baseJ.x + velJ.x * result.getEntry(1, 0),
                                        baseJ.y + velJ.y * result.getEntry(1, 0)
                                    )
                                );
                                let normalizedAngle = if (angle) <= 90.0 {
                                    180.0 - 2.0 * angle
                                } else {
                                    360.0 - 2.0 * angle

                                    //     180.0 - 2.0 * angle
                                };
                                let rotated =
                                    rotatePoint(&centre, normalizedAngle.to_radians(), &sp);
                                let newVel = sp.vecTo(rotated);
                                println!("sp{:?}", sp);
                                println!("centre{:?}", centre);

                                println!("rotated{:?}", rotated);
                                println!("newVel{:?}", newVel);
                                gameobjs[i].setVelocity(newVel);
                                println!("angle{}", angle);
                            }
                        },
                        CollisionBoxTypes::Line => match gameobjs[j].getColBox().colType {
                            CollisionBoxTypes::AABB => {}
                            CollisionBoxTypes::Circle => {
                                // gameobjs[j].setVelocity(velJ.reverse()),
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

    fn calculateIntersectPoint(
        &self,
        baseLeft: Vector2,
        baseRight: Vector2,
        vecLeft: Vector2,
        vecRight: Vector2,
    ) -> Point {
        let mut sGes = 0.0;
        let mut xGes = 0.0;
        let baseI = baseLeft;
        let velJ = vecLeft;
        let baseJ = baseRight;
        let velI = vecRight;

        println!("baseI{:?}", baseLeft);
        println!("baseRight{:?}", baseRight);
        println!("velLeft{:?}", vecLeft);
        println!("velRight{:?}", vecRight);

        let origBaseI = baseI;
        let origBaseJ = baseJ;
        let origVelJ = velJ;
        let origVelI = velI;
        let baseI = baseI.subtract(&baseJ);
        let velJ = velJ.reverse();
        let bx = baseI.x;
        let by = baseI.y;
        let rx = velJ.x;
        let ry = velJ.y;
        let sx = velI.x;
        let sy = velI.y;
        if (ry == 0.0) {
            sGes = by;
            sGes = sGes / sx;
            xGes = -bx + sy * sx;
            xGes = xGes / rx;
        } else if (sy == 0.0) {
            xGes = by;
            xGes = xGes / rx;
            sGes = bx - xGes * rx;
            sGes = sGes.abs();
        } else if (sx == 0.0) {
            xGes = bx;
            xGes = xGes / rx;
            sGes = by - xGes * ry;
            sGes = sGes / sy;
        } else if (rx == 0.0) {
            sGes = bx;
            sGes = sGes.abs();
            xGes = by - sGes * sy;
            xGes = xGes.abs();
        } else {
            let bx = bx / rx;
            let sx = sx / rx;
            let rx = rx / rx;
            let by = by / ry;
            let sy = sy / ry;
            let ry = ry / ry;
            let s = -sy + sx;
            let b = bx - by;
            let b = b / s;
            let s = s / s;
            sGes = b;
            xGes = bx - sGes * sx;
        }
        println!("sGes{}", sGes);
        println!("xGes{}", xGes);
        println!("xCoord{}", origBaseI.x + xGes * origVelJ.x);
        println!("yCoord{}", origBaseI.y + xGes * origVelJ.y);
        println!("xCoord{}", origBaseJ.x + sGes * origVelI.x);
        println!("yCoord{}", origBaseJ.y + sGes * origVelI.y);
        return Point::new(
            origBaseI.x + xGes * origVelJ.x,
            origBaseI.y + xGes * origVelJ.y,
        );
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
