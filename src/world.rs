use crate::{gameObj::GameObj, renderer::Renderer, utils::getTime};

pub struct World {
    pub renderer: Renderer,
    pub objects: Vec<Box<dyn GameObj>>,
}

impl World {
    pub fn new(renderer: Renderer, objects: Vec<Box<dyn GameObj>>) -> World {
        World { renderer, objects }
    }
    pub fn addObj(self: &mut Self, object: Box<dyn GameObj>) {
        self.objects.push(object)
    }
    pub fn drawAll(&mut self) {
        let start1 = getTime();
        for i in &mut self.objects {
            i.draw(&mut self.renderer);
        }
        let end1 = getTime();
        //println!("Time:{}", end1 - start1);
    }
    pub fn drawAllAtIndex(&mut self, index: usize) {
        let start1 = getTime();
        for i in 0..self.objects.len() {
            if i >= index {
                self.objects[i].draw(&mut self.renderer);
            }
        }
        let end1 = getTime();
        //println!("Time:{}", end1 - start1);
    }
}
