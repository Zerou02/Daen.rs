use std::{f64::consts::PI, vec};

use crate::{
    colours::Colour,
    gameObj::{GameObj, IGameObj},
    point::Point,
    utils::rotatePoint,
};

pub struct Line {
    gameObj: GameObj,
}

impl Line {
    pub fn new(p1: Point, p2: Point, colour: Colour) -> Line {
        let mut retVal = Line {
            gameObj: GameObj {
                rotation: 0.0,
                centre: Point::newI(0, 0),
                colour,
                points: vec![p1, p2],
                filled: false,
                id: 0,
            },
        };
        retVal.gameObj.centre = retVal.calculateCentrePoint();
        return retVal;
    }

    fn calculateCentrePoint(&self) -> Point {
        let x = (self.gameObj.points[1].x + self.gameObj.points[0].x) / 2.0;
        let y = (self.gameObj.points[1].y + self.gameObj.points[0].y) / 2.0;

        return Point::new(x, y);
    }
}

impl IGameObj for Line {
    fn draw(&self, renderer: &mut crate::renderer::Renderer) {
        renderer.drawLine(
            &self.gameObj.points[0],
            &self.gameObj.points[1],
            self.gameObj.colour.rgba,
        )
    }

    fn moveI(self: &mut Self, x: i64, y: i64) {
        self.gameObj.moveI(x, y);
    }

    fn getColour(&mut self) -> &mut Colour {
        return self.gameObj.getColour();
    }

    fn setColour(&mut self, colour: Colour) {
        self.gameObj.setColour(colour);
    }

    fn rotate(&mut self, rad: f64) {
        self.gameObj.rotate(rad);
    }

    fn setRotation(&mut self, rad: f64) {
        self.gameObj.setRotation(rad);
    }

    fn setFilled(&mut self, val: bool) {
        self.gameObj.setFilled(val);
    }
}
