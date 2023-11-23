use crate::{colours::Colour, gameObj::GameObj, point::Point, utils::rotatePoint};

pub struct Triangle {
    points: [Point; 3],
    centre: Point,
    colour: Colour,
    rotation: f64,
    filled: bool,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point, colour: Colour) -> Triangle {
        let mut retVal = Triangle {
            points: [p1, p2, p3],
            centre: Point::newI(0, 0),
            colour,
            rotation: 0.0,
            filled: false,
        };
        retVal.calculateCentrePoint();
        return retVal;
    }

    pub fn calculateCentrePoint(&mut self) {
        let p1p2 = self.points[0].centreTo(&self.points[1]);
        let p1p3 = self.points[0].centreTo(&self.points[2]);
        self.centre = p1p2.centreTo(&p1p3);
    }
}

impl GameObj for Triangle {
    fn draw(&self, renderer: &mut crate::renderer::Renderer) {
        if (self.filled) {
            renderer.drawTriangle(
                &self.points[0],
                &self.points[1],
                &self.points[2],
                self.colour.rgba,
            );
        } else {
            renderer.fillTriangle(
                &self.points[0],
                &self.points[1],
                &self.points[2],
                self.colour.rgba,
            )
        }
    }

    fn moveObj(&mut self, x: f64, y: f64) {
        for mut p in self.points {
            p.x += x;
            p.y += y;
        }
    }

    fn getColour(&mut self) -> &mut Colour {
        return &mut self.colour;
    }

    fn setColour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    fn rotate(&mut self, rad: f64) {
        self.rotation += rad;
        let np = Point::newI(0, 0);
        let mut newPoints: [Point; 3] = [np, np, np];
        for i in 0..newPoints.len() {
            newPoints[i] = rotatePoint(&self.points[i], rad, &self.centre);
        }
        self.points = newPoints;
    }

    fn setRotation(&mut self, rad: f64) {
        todo!()
    }

    fn setFilled(&mut self, val: bool) {
        self.filled = val;
    }
}
