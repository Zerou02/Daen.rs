use crate::{colours::Colour, gameObj::GameObj, point::Point, utils::rotatePoint};

pub struct Ellipsis {
    leftFocalPoint: Point,
    rightFocalPoint: Point,
    colour: Colour,
    distance: f64,
    filled: bool,
}

impl Ellipsis {
    pub fn new(p1: Point, p2: Point, distance: f64, colour: Colour) -> Ellipsis {
        return Ellipsis {
            leftFocalPoint: p1,
            rightFocalPoint: p2,
            colour,
            distance,
            filled: false,
        };
    }
}

impl GameObj for Ellipsis {
    fn draw(&self, renderer: &mut crate::renderer::Renderer) {
        if self.filled {
            renderer.fillEllipsis(
                &self.leftFocalPoint,
                &self.rightFocalPoint,
                self.distance as i32,
                self.colour.rgba,
            );
        } else {
            renderer.drawEllipsis(
                &self.leftFocalPoint,
                &self.rightFocalPoint,
                self.distance as i32,
                self.colour.rgba,
            )
        }
    }
    fn moveObj(self: &mut Self, x: f64, y: f64) {
        self.leftFocalPoint.x += x;
        self.rightFocalPoint.x += x;
        self.leftFocalPoint.y += y;
        self.rightFocalPoint.y += y;
    }

    fn getColour(&mut self) -> &mut Colour {
        return &mut self.colour;
    }

    fn setColour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    fn rotate(&mut self, rad: f64) {
        let pivot = Point {
            x: (self.rightFocalPoint.x + self.leftFocalPoint.x) / 2.0,
            y: (self.rightFocalPoint.y + self.leftFocalPoint.y) / 2.0,
        };
        let newPoint1 = rotatePoint(&self.leftFocalPoint, rad, &pivot);
        let newPoint2 = rotatePoint(&self.rightFocalPoint, rad, &pivot);
        self.leftFocalPoint = newPoint1;
        self.rightFocalPoint = newPoint2;
    }

    fn setRotation(&mut self, deg: f64) {
        todo!()
    }
}
