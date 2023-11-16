use crate::renderer::Renderer;

pub trait GameObj {
    fn draw(&self, renderer: &mut Renderer);
    fn moveObj(&mut self, x: i32, y: i32);
    fn getColour(&mut self) -> u32;
    fn setColour(&mut self, colour: u32);
}
