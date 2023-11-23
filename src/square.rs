use std::ops::Range;

use winit::platform::x11;

use crate::{
    colours::Colour,
    gameObj::{GameObj, IGameObj},
    point::Point,
    renderer::Renderer,
    utils::rotatePoint,
};

pub struct Square {
    gameObj: GameObj,
}

impl Square {
    pub fn new(p: Point, w: f64, h: f64, colour: Colour) -> Square {
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
                id: 0,
            },
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

    fn moveObj(self: &mut Self, x: f64, y: f64) {
        self.gameObj.moveObj(x, y);
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
}
