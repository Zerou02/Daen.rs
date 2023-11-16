use crate::{gameObj::GameObj, point::Point, renderer::Renderer};

pub struct Square {
    pivot: Point,
    dim: Point,
    colour: u32,
}

impl Square {
    pub fn new(x: i32, y: i32, w: i32, h: i32, colour: u32) -> Square {
        return Square {
            pivot: Point { x, y },
            dim: Point { x: w, y: h },
            colour,
        };
    }
}

impl GameObj for Square {
    fn draw(&self, renderer: &mut Renderer) {
        renderer.fillSquare(&self.pivot, &self.dim, self.colour);
    }
    fn moveObj(self: &mut Self, x: i32, y: i32) {
        self.pivot.x += x;
        self.pivot.y += y;
    }

    fn getColour(&mut self) -> u32 {
        return self.colour;
    }

    fn setColour(&mut self, colour: u32) {
        self.colour = colour;
    }
}
