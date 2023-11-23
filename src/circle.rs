use crate::{colours::Colour, gameObj::GameObj, point::Point};

pub struct Circle {
    centre: Point,
    r: f64,
    colour: Colour,
    filled: bool,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64, colour: Colour) -> Circle {
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
            renderer.fillCircle(&self.centre, self.r as i32, self.colour.rgba);
        } else {
            renderer.drawCircle(&self.centre, self.r as i32, self.colour.rgba)
        }
    }
    fn moveObj(self: &mut Self, x: f64, y: f64) {
        self.centre.x += x;
        self.centre.y += y;
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

    fn setFilled(&mut self, val: bool) {
        todo!()
    }
}
