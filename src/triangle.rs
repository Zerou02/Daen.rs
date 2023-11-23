use crate::{
    colours::Colour,
    gameObj::{GameObj, IGameObj},
    point::Point,
    utils::rotatePoint,
};

pub struct Triangle {
    gameObj: GameObj,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point, colour: Colour, id: u64) -> Triangle {
        let mut retVal = Triangle {
            gameObj: GameObj {
                colour,
                points: vec![p1, p2, p3],
                centre: Point::newI(0, 0),
                rotation: 0.0,
                filled: false,
                id,
            },
        };
        retVal.calculateCentrePoint();
        return retVal;
    }

    pub fn calculateCentrePoint(&mut self) {
        let p1p2 = self.gameObj.points[0].centreTo(&self.gameObj.points[1]);
        let p1p3 = self.gameObj.points[0].centreTo(&self.gameObj.points[2]);
        self.gameObj.centre = p1p2.centreTo(&p1p3);
    }
}

impl IGameObj for Triangle {
    fn draw(&self, renderer: &mut crate::renderer::Renderer) {
        if (self.gameObj.filled) {
            renderer.drawTriangle(
                &self.gameObj.points[0],
                &self.gameObj.points[1],
                &self.gameObj.points[2],
                self.gameObj.colour.rgba,
            );
        } else {
            renderer.fillTriangle(
                &self.gameObj.points[0],
                &self.gameObj.points[1],
                &self.gameObj.points[2],
                self.gameObj.colour.rgba,
            )
        }
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
        self.gameObj.rotate(rad)
    }

    fn setRotation(&mut self, rad: f64) {
        self.gameObj.setRotation(rad)
    }

    fn setFilled(&mut self, val: bool) {
        self.gameObj.setFilled(val);
    }
}
