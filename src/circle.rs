use crate::{gameObj::GameObj, point::Point};

pub struct Circle {
    centre: Point,
    r: i32,
    colour: u32,
    filled: bool,
}

impl Circle {
    pub fn new(x: i32, y: i32, r: i32, colour: u32) -> Circle {
        return Circle {
            centre: Point { x, y },
            r,
            colour,
            filled: false,
        };
    }
}

impl GameObj for Circle {
    fn draw(&self, renderer: &mut crate::renderer::Renderer) {
        if self.filled {
            renderer.fillCircle(self.centre.x, self.centre.y, self.r, self.colour);
        } else {
            renderer.drawCircle(&self.centre, self.r, self.colour)
        }
    }
    fn moveObj(self: &mut Self, x: i32, y: i32) {
        self.centre.x += x;
        self.centre.y += y;
    }

    fn rotate(self: &mut Self, deg: f64) {}
    fn getColour(&mut self) -> u32 {
        return self.colour;
    }

    fn setColour(&mut self, colour: u32) {
        self.colour = colour;
    }
}
