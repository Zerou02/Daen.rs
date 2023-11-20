use std::f64::consts::PI;

use crate::{colours::Colour, gameObj::GameObj, point::Point, utils::rotatePoint};

pub struct Line {
    p1: Point,
    p2: Point,
    centre: Point,
    rotation: f64,
    colour: Colour,
}

impl Line {
    pub fn new(p1: Point, p2: Point, colour: Colour) -> Line {
        let mut retVal = Line {
            p1,
            p2,
            colour,
            centre: Point { x: 0.0, y: 0.0 },
            rotation: 0.0,
        };
        retVal.calculateCentrePoint();
        return retVal;
    }

    fn calculateCentrePoint(&mut self) {
        let x = (self.p2.x - self.p1.x) / 2.0 + self.p1.x;
        let y = (self.p2.y - self.p1.y) / 2.0 + self.p1.y;

        self.centre = Point { x, y }
    }
}

impl GameObj for Line {
    fn draw(&self, renderer: &mut crate::renderer::Renderer) {
        renderer.drawLine(&self.p1, &self.p2, self.colour.rgba)
    }

    fn moveObj(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        self.p1.y += y;
        self.p2.y += y;
    }

    fn getColour(&mut self) -> &mut Colour {
        return &mut self.colour;
    }

    fn setColour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    fn rotate(&mut self, rad: f64) {
        let pivot = Point {
            x: (self.p2.x + self.p1.x) / 2.0,
            y: (self.p2.y + self.p1.y) / 2.0,
        };
        let newPoint1 = rotatePoint(&self.p1, rad, &pivot);
        let newPoint2 = rotatePoint(&self.p2, rad, &pivot);
        self.p1 = newPoint1;
        self.p2 = newPoint2;
    }

    fn setRotation(&mut self, deg: f64) {
        self.rotate(deg - self.rotation);
    }
}
