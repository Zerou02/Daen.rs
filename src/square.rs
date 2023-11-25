use std::ops::Range;

use crate::{
    collisionBox::CollisionBox,
    colours::Colour,
    gameObj::{GameObj, IGameObj},
    point::Point,
    renderer::Renderer,
    utils::rotatePoint,
    vector2::Vector2,
};

pub struct Square {
    gameObj: GameObj,
    colBox: CollisionBox,
}

impl Square {
    pub fn new(p: Point, w: f64, h: f64, colour: Colour, id: u64) -> Square {
        let mut retS = Square {
            gameObj: GameObj {
                rotation: 0.0,
                centre: Point::newI(0, 0),
                colour,
                points: vec![
                    p,
                    Point::new(p.x + w, p.y),
                    Point::new(p.x + w, p.y + h),
                    Point::new(p.x, p.y + h),
                ],
                filled: false,
                id,
                velocity: Vector2::newI(0, 0),
            },
            colBox: CollisionBox::new(
                crate::collisionBox::CollisionBoxTypes::AABB,
                vec![],
                vec![],
                id,
            ),
        };
        let centrePoint = retS.gameObj.points[0].centreTo(&retS.gameObj.points[3]);
        retS.gameObj.centre = centrePoint;
        return retS;
    }
}

impl IGameObj for Square {
    fn draw(&self, renderer: &mut Renderer) {
        if (self.gameObj.filled) {
            renderer.fillSquare(&self.gameObj.points, self.gameObj.colour.rgba);
        } else {
            renderer.drawSquare(&self.gameObj.points, self.gameObj.colour.rgba);
        }
    }

    fn moveI(self: &mut Self, x: i64, y: i64) {
        self.gameObj.moveI(x, y);
    }

    fn getColour(&mut self) -> &mut Colour {
        return self.gameObj.getColour();
    }

    fn setColour(&mut self, colour: Colour) {
        self.gameObj.setColour(colour)
    }

    fn rotate(&mut self, rad: f64) {
        self.gameObj.rotate(rad)
    }

    fn setRotation(&mut self, rad: f64) {
        self.gameObj.setRotation(rad)
    }

    fn setFilled(&mut self, val: bool) {
        self.gameObj.setFilled(val)
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
        self.gameObj.mMove();
    }

    fn getID(&self) -> u64 {
        return self.gameObj.getId();
    }
}
