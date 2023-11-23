use crate::{
    gameObj::{GameObj, IGameObj},
    gameObjectManager::GameObjManager,
    renderer::Renderer,
    utils::getTime,
};

pub struct World {
    pub renderer: Renderer,
    pub objectManager: GameObjManager,
}

impl World {
    pub fn new(renderer: Renderer) -> World {
        World {
            renderer,
            objectManager: GameObjManager::new(),
        }
    }
    pub fn addObj(self: &mut Self, object: Box<dyn IGameObj>) {
        self.objectManager.addGameObj(object);
    }
    pub fn drawAll(&mut self) {
        let start1 = getTime();
        for i in &self.objectManager.gameObj {
            i.draw(&mut self.renderer);
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
}
