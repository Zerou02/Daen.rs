use crate::{colours::Colour, gameObj::GameObj, point::Point, renderer::Renderer};

pub struct Square {
    pivot: Point,
    dim: Point,
    colour: Colour,
}

impl Square {
    pub fn new(x: f64, y: f64, w: f64, h: f64, colour: Colour) -> Square {
        return Square {
            pivot: Point { x, y },
            dim: Point { x: w, y: h },
            colour,
        };
    }
}

impl GameObj for Square {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.fillSquare(&self.pivot, &self.dim, self.colour.rgba);
    }
    fn moveObj(self: &mut Self, x: f64, y: f64) {
        self.pivot.x += x;
        self.pivot.y += y;
    }

    fn getColour(&mut self) -> &mut Colour {
        return &mut self.colour;
    }

    fn setColour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    fn rotate(&mut self, deg: f64) {
        todo!()
    }

    fn setRotation(&mut self, deg: f64) {
        todo!()
    }
}
