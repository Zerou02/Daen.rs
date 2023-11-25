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

    pub fn applyPhysics(&mut self, gameobjs: &mut Vec<Box<dyn IGameObj>>) {
        self.checkCollisions(gameobjs);

        //gameobj.moveF(0.0, self.gravity);
    }

    pub fn checkCollisions(&mut self, gameobjs: &mut Vec<Box<dyn IGameObj>>) {
        for i in 0..gameobjs.len() {
            for j in 0..gameobjs.len() {
                if (i != j) {
                    if (gameobjs[i].getColBox().collides(gameobjs[j].getColBox())) {
                        println!("COL");
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
        for i in gameobjs {
            i.getColBoxMut().clearCollidedWith();
        }
    }
}
