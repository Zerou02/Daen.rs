use crate::{
    colours::getColourVal,
    gameObj::{GameObj, IGameObj},
    gameObjectManager::GameObjManager,
    physicsEngine::PhysicsEngine,
    renderer::Renderer,
    utils::getTime,
};

pub struct World {
    pub renderer: Renderer,
    pub objectManager: GameObjManager,
    physicsEngine: PhysicsEngine,
}

impl World {
    pub fn new(renderer: Renderer) -> World {
        World {
            renderer,
            objectManager: GameObjManager::new(),
            physicsEngine: PhysicsEngine::new(),
        }
    }
    pub fn addObj(self: &mut Self, object: Box<dyn IGameObj>) {
        self.objectManager.addGameObj(object);
    }
    pub fn drawAll(&mut self) {
        self.clear();

        let start1 = getTime();
        let mut indicesToRemove: Vec<usize> = vec![];
        self.physicsEngine
            .applyPhysics(&mut self.objectManager.gameObj);

        for i in 0..self.objectManager.gameObj.len() {
            let e = &mut self.objectManager.gameObj[i];
            if (e.readyForTrial()) {
                indicesToRemove.push(i);
            };
            e.draw(&mut self.renderer);
        }

        for i in indicesToRemove {
            self.objectManager.gameObj.remove(i);
        }
        let end1 = getTime();
        //println!("Time:{}", end1 - start1);
    }

    pub fn drawAllAtIndex(&mut self, index: usize) {
        let start1 = getTime();
        for i in 0..self.objectManager.gameObj.len() {
            if i >= index {
                self.objectManager.gameObj[i].draw(&mut self.renderer);
            }
        }
        let end1 = getTime();
        //println!("Time:{}", end1 - start1);
    }

    pub fn clear(&mut self) {
        self.renderer
            .clearBuf(getColourVal(crate::colours::ColourType::BLACK));
    }

    pub fn gObjMM(&mut self) -> &mut GameObjManager {
        return &mut self.objectManager;
    }
}
