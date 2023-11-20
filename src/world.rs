use crate::{gameObj::GameObj, renderer::Renderer};

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
        for i in &mut self.objects {
            i.draw(&mut self.renderer);
        }
    }
}
