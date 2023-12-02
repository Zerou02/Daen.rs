use std::{f64::consts::PI, vec};

use crate::{
    collisionBox::{CollisionBox, CollisionBoxTypes},
    colours::Colour,
    gameObj::{GameObj, IGameObj},
    point::Point,
    utils::rotatePoint,
    vector2::Vector2,
};

pub struct Line {
    gameObj: GameObj,
    colBox: CollisionBox,
}

impl Line {
    pub fn new(p1: Point, p2: Point, colour: Colour, id: u64) -> Line {
        let mut retVal = Line {
            gameObj: GameObj {
                rotation: 0.0,
                centre: Point::newI(0, 0),
                colour,
                points: vec![p1, p2],
                filled: false,
                mass: 9999.9,
                id,
                movesLeft: 0,
                velocity: Vector2::newI(0, 0),
            },
            colBox: CollisionBox::new(CollisionBoxTypes::Line, vec![p1, p2], vec![], id),
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
        self.colBox.moveI(x, y);
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

    fn setCentre(&mut self, centre: Point) {
        self.gameObj.centre = centre;
    }

    fn readyForTrial(&self) -> bool {
        return self.gameObj.readyForTrial();
    }

    fn getColBox(&self) -> &CollisionBox {
        return &self.colBox;
    }
    fn getColBoxMut(&mut self) -> &mut CollisionBox {
        return &mut self.colBox;
    }

    fn getVelocity(&self) -> Vector2 {
        return self.gameObj.getVelocity();
    }

    fn setVelocity(&mut self, v: Vector2) {
        self.gameObj.setVelocity(v)
    }

    fn mMove(&mut self) {
        let v = self.gameObj.velocity;
        self.gameObj.mMove();
        self.colBox.moveF(v.x, v.y);
    }

    fn getID(&self) -> u64 {
        return self.gameObj.getId();
    }

    fn getMovesLeft(&self) -> i32 {
        return self.gameObj.getMovesLeft();
    }

    fn setMovesLeft(&mut self, val: i32) {
        self.gameObj.setMovesLeft(val)
    }

    fn getMass(&self) -> f64 {
        return self.gameObj.getMass();
    }
}
