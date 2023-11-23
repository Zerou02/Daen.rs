use core::num;

use crate::{colours::Colour, point::Point, renderer::Renderer, utils::rotatePoint};

pub struct GameObj {
    pub rotation: f64,
    pub centre: Point,
    pub colour: Colour,
    pub points: Vec<Point>,
    pub filled: bool,
    pub id: u64,
}

impl GameObj {
    pub fn moveI(self: &mut Self, x: i64, y: i64) {
        for p in &mut self.points {
            p.x += x as f64;
            p.y += y as f64;
        }
    }

    pub fn rotate(&mut self, rad: f64) {
        self.rotation += rad;
        self.rotation %= 360.0;
        for p in &mut self.points {
            *p = rotatePoint(&p, rad, &self.centre)
        }
    }

    pub fn getColour(&mut self) -> &mut Colour {
        return &mut self.colour;
    }

    pub fn setColour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    pub fn setFilled(&mut self, val: bool) {
        self.filled = val;
    }

    pub fn setRotation(&mut self, rad: f64) {
        self.rotate(-self.rotation);
        self.rotation = rad;
        self.rotate(self.rotation);
    }
}

pub trait IGameObj {
    fn draw(&self, renderer: &mut Renderer);
    fn moveI(&mut self, x: i64, y: i64);
    fn getColour(&mut self) -> &mut Colour;
    fn setColour(&mut self, colour: Colour);
    fn rotate(&mut self, rad: f64);
    fn setRotation(&mut self, rad: f64);
    fn setFilled(&mut self, val: bool);
}
