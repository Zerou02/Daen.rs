use crate::{
    colours::Colour,
    gameObj::{GameObj, IGameObj},
    point::Point,
    utils::rotatePoint,
};

pub struct Ellipsis {
    gameObj: GameObj,
    distance: f64,
}

impl Ellipsis {
    pub fn new(p1: Point, p2: Point, distance: f64, colour: Colour, id: u64) -> Ellipsis {
        return Ellipsis {
            gameObj: GameObj {
                rotation: 0.0,
                centre: p1.centreTo(&p2),
                colour,
                points: vec![p1, p2],
                filled: false,
                id,
            },
            distance,
        };
    }
}

impl IGameObj for Ellipsis {
    fn draw(&self, renderer: &mut crate::renderer::Renderer) {
        if self.gameObj.filled {
            renderer.fillEllipsis(
                &self.gameObj.points[0],
                &self.gameObj.points[1],
                self.distance as i32,
                self.gameObj.colour.rgba,
            );
        } else {
            renderer.drawEllipsis(
                &self.gameObj.points[0],
                &self.gameObj.points[1],
                self.distance as i32,
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
        self.gameObj.rotate(rad);
    }

    fn setRotation(&mut self, rad: f64) {
        self.gameObj.setRotation(rad);
    }

    fn setFilled(&mut self, val: bool) {
        self.gameObj.setFilled(val);
    }
}
