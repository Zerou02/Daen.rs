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
        self.rotation += rad;
        self.rotation %= 360.0;
        let newPoint1 = rotatePoint(&self.p1, rad, &self.centre);
        let newPoint2 = rotatePoint(&self.p2, rad, &self.centre);
        self.p1 = newPoint1;
        self.p2 = newPoint2;
    }

    fn setRotation(&mut self, rad: f64) {
        self.rotate(-self.rotation);
        //        let base1 = rotatePoint(&self.p1, -self.rotation, &self.centre);
        //      let base2 = rotatePoint(&self.p2, -self.rotation, &self.centre);
        self.rotation = rad;
        //    let new1 = rotatePoint(&base1, -self.rotation, &self.centre);
        //  let new2 = rotatePoint(&base2, -self.rotation, &self.centre);
        self.rotate(self.rotation);
    }

    fn setFilled(&mut self, val: bool) {
        todo!()
    }
}
