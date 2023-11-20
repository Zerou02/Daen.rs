use crate::{colours::Colour, renderer::Renderer};

pub trait GameObj {
    fn draw(&self, renderer: &mut Renderer);
    fn moveObj(&mut self, x: f64, y: f64);
    fn getColour(&mut self) -> &mut Colour;
    fn setColour(&mut self, colour: Colour);
    fn rotate(&mut self, deg: f64);
    fn setRotation(&mut self, deg: f64);
}
