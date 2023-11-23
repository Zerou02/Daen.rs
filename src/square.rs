use std::ops::Range;

use winit::platform::x11;

use crate::{
    colours::Colour, gameObj::GameObj, point::Point, renderer::Renderer, utils::rotatePoint,
};

pub struct Square {
    points: [Point; 4],
    centre: Point,
    dim: Point,
    colour: Colour,
    filled: bool,
}

impl Square {
    pub fn new(p: Point, w: f64, h: f64, colour: Colour) -> Square {
        return Square {
            points: [
                p,
                Point::new(p.x + w, p.y),
                Point::new(p.x + w, p.y + h),
                Point::new(p.x, p.y + h),
            ],
            centre: p.centreTo(&Point::new(p.x + w, p.y + h)),
            dim: Point { x: w, y: h },
            colour,
            filled: false,
        };
    }
}

impl GameObj for Square {
    fn draw(&self, renderer: &mut Renderer) {
        if (self.filled) {
            renderer.fillSquare(&self.points, self.colour.rgba);
        } else {
            renderer.drawSquare(&self.points, self.colour.rgba)
        }
    }
    fn moveObj(self: &mut Self, x: f64, y: f64) {
        todo!();
    }

    fn getColour(&mut self) -> &mut Colour {
        return &mut self.colour;
    }

    fn setColour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    fn rotate(&mut self, rad: f64) {
        let nP = Point::newI(0, 0);
        let mut newPoints: [Point; 4] = [nP, nP, nP, nP];
        for i in 0..self.points.len() {
            newPoints[i] = rotatePoint(&self.points[i], rad, &self.centre);
        }
        self.points = newPoints;
    }

    fn setRotation(&mut self, deg: f64) {
        todo!()
    }

    fn setFilled(&mut self, val: bool) {
        self.filled = val;
    }
}
