use std::vec;

use crate::{
    collisionBox::{CollisionBox, CollisionBoxTypes},
    colours::Colour,
    constants::{DEL_HEIGHT_L, DEL_HEIGHT_R, DEL_WIDTH_R, WIDTH},
    gameObj::{GameObj, IGameObj},
    point::Point,
    vector2::Vector2,
};

pub struct Circle {
    gameObj: GameObj,
    r: f64,
    collisonBox: CollisionBox,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64, colour: Colour, id: u64) -> Circle {
        let centre = Point::new(x, y);
        return Circle {
            gameObj: GameObj {
                rotation: 0.0,
                centre,
                colour,
                points: vec![centre],
                filled: true,
                id,
                mass: r,
                movesLeft: 0,
                velocity: Vector2::newI(0, 0),
            },
            r,
            collisonBox: CollisionBox::newCircle(vec![centre], vec![r], id),
        };
    }
}

impl IGameObj for Circle {
    fn draw(&self, renderer: &mut crate::renderer::Renderer) {
        if self.gameObj.filled {
            renderer.fillCircle(
                &self.gameObj.points[0],
                self.r as i32,
                self.gameObj.colour.rgba,
            );
        } else {
            renderer.drawCircle(
                &self.gameObj.points[0],
                self.r as i32,
                self.gameObj.colour.rgba,
            )
        }
    }
    fn moveI(self: &mut Self, x: i64, y: i64) {
        self.gameObj.moveI(x, y);
        self.collisonBox.moveI(x, y);
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
        self.gameObj.rotate(rad);
    }

    fn setFilled(&mut self, val: bool) {
        self.gameObj.setFilled(val);
    }

    fn setCentre(&mut self, centre: Point) {
        self.gameObj.centre = centre;
    }

    fn readyForTrial(&self) -> bool {
        let x = self.gameObj.points[0].x;
        let y = self.gameObj.points[0].y;
        let r = self.r;
        return x + r > DEL_WIDTH_R
            || x + r < DEL_HEIGHT_L
            || y + r > DEL_HEIGHT_R
            || y + r < DEL_HEIGHT_L;
    }

    fn getColBox(&self) -> &CollisionBox {
        return &self.collisonBox;
    }
    fn getColBoxMut(&mut self) -> &mut CollisionBox {
        return &mut self.collisonBox;
    }

    fn setVelocity(&mut self, v: Vector2) {
        self.gameObj.setVelocity(v)
    }

    fn getVelocity(&self) -> Vector2 {
        return self.gameObj.getVelocity();
    }

    fn mMove(&mut self) {
        let v = self.gameObj.velocity;
        self.gameObj.mMove();
        self.collisonBox.moveF(v.x, v.y);
    }

    fn moveF(&mut self, x: f64, y: f64) {
        self.gameObj.moveF(x, y);
        self.collisonBox.moveF(x, y);
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
