use crate::{
    collisionBox::CollisionBoxTypes,
    gameObj::{self, IGameObj},
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
                                let baseI = gameobjs[i].getColBoxMut().points[0].toVec();
                                let velI = gameobjs[i].getVelocity();
                                let basesJ = gameobjs[j].getColBoxMut().points[0].toVec();
                                let velJ = gameobjs[j].getVelocity();

                                let baseI = baseI.subtract(&basesJ);
                                let velI = velI.reverse();

                                let bx = baseI.x;
                                let by = baseI.y;
                                let rx = velJ.x;
                                let ry = velJ.y;
                                let sx = velI.x;
                                let sy = velI.y;

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

                                let sGes = b;
                                let xGes = bx - s * sx;

                                /*          let m1 = gameobjs[i].getMass();
                                let m2 = gameobjs[j].getMass();
                                let u1 = gameobjs[i].getVelocity();
                                let u2 = gameobjs[i].getVelocity();
                                let velJ = gameobjs[j].getVelocity();
                                let theta = gameobjs[i].getVelocity().angleTo(&Vector2::newI(1, 0));
                                let velINewX = (2.0 * m1 * u1.x * theta.to_radians().cos()
                                    - (m1 - m2) * u2.x * (theta.to_radians().cos()))
                                    / (m1 + m2);
                                let velINewY = (2.0 * m1 * u1.y * theta.to_radians().cos()
                                    - (m1 - m2) * u2.y * (theta.to_radians().cos()))
                                    / (m1 + m2);
                                println!("theta{}", theta);
                                println!("theta{}", theta.to_radians());
                                println!("theta{}", theta.to_radians().cos());
                                println!("Vec:{}{}", velINewX, velINewY);
                                gameobjs[i].setVelocity(Vector2::new(velINewX, velINewY)); */
                            }
                        },
                        CollisionBoxTypes::Line => match gameobjs[j].getColBox().colType {
                            CollisionBoxTypes::AABB => {}
                            CollisionBoxTypes::Circle => gameobjs[j].setVelocity(velJ.reverse()),
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

    pub fn checkCollisions(&self, gameobjs: &mut Vec<Box<dyn IGameObj>>) {
        for i in 0..gameobjs.len() {
            for j in 0..gameobjs.len() {
                if (i != j) {
                    if (gameobjs[i].getColBox().collides(gameobjs[j].getColBox())) {
                        let idJ = gameobjs[j].getColBox().getId();
                        let idI = gameobjs[i].getColBox().getId();
                        gameobjs[i].getColBoxMut().setToCollidedWith(idJ);
                        gameobjs[j].getColBoxMut().setToCollidedWith(idI);

                        let velI = gameobjs[i].getVelocity();
                        let velJ = gameobjs[j].getVelocity();

                        //HandleCollisions
                        match gameobjs[i].getColBox().colType {
                            CollisionBoxTypes::AABB => {}
                            CollisionBoxTypes::Circle => match gameobjs[j].getColBox().colType {
                                CollisionBoxTypes::AABB => {}
                                CollisionBoxTypes::Circle => {
                                    gameobjs[i].setVelocity(velJ);
                                    gameobjs[j].setVelocity(velI);
                                }
                                CollisionBoxTypes::Line => {
                                    gameobjs[i].setVelocity(velI.reverse());
                                }
                            },
                            CollisionBoxTypes::Line => match gameobjs[j].getColBox().colType {
                                CollisionBoxTypes::AABB => {}
                                CollisionBoxTypes::Circle => {
                                    gameobjs[j].setVelocity(velJ.reverse())
                                }
                                CollisionBoxTypes::Line => {}
                            },
                        }
                    }
                }
            }
        }
        for mut i in gameobjs {
            i.getColBoxMut().clearCollidedWith();
        }
    }
}
