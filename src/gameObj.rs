use crate::{colours::Colour, renderer::Renderer};

pub struct GameObj {
    rotation: f64,
}
pub trait GameObjT {
    fn draw(&self, renderer: &mut Renderer);
    fn moveObj(&mut self, x: f64, y: f64);
    fn getColour(&mut self) -> &mut Colour;
    fn setColour(&mut self, colour: Colour);
    fn rotate(&mut self, rad: f64);
    fn setRotation(&mut self, rad: f64);
    fn setFilled(&mut self, val: bool);
}
