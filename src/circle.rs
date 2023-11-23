use crate::{
    colours::Colour,
    gameObj::{GameObj, IGameObj},
    point::Point,
};

pub struct Circle {
    gameObj: GameObj,
    r: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64, colour: Colour, id: u64) -> Circle {
        return Circle {
            gameObj: GameObj {
                rotation: 0.0,
                centre: Point::new(x, y),
                colour,
                points: vec![],
                filled: true,
                id,
            },
            r,
        };
    }
}

impl IGameObj for Circle {
    fn draw(&self, renderer: &mut crate::renderer::Renderer) {
        if self.gameObj.filled {
            renderer.fillCircle(
                &self.gameObj.centre,
                self.r as i32,
                self.gameObj.colour.rgba,
            );
        } else {
            renderer.drawCircle(
                &self.gameObj.centre,
                self.r as i32,
                self.gameObj.colour.rgba,
            )
        }
    }
    fn moveI(self: &mut Self, x: i64, y: i64) {
        self.gameObj.moveI(x, y);
    }

    fn getColour(&mut self) -> &mut Colour {
        return self.getColour();
    }

    fn setColour(&mut self, colour: Colour) {
        self.setColour(colour);
    }

    fn rotate(&mut self, rad: f64) {
        self.rotate(rad);
    }

    fn setRotation(&mut self, rad: f64) {
        self.rotate(rad);
    }

    fn setFilled(&mut self, val: bool) {
        self.setFilled(val);
    }
}
