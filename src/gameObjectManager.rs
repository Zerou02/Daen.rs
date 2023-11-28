use rand::Rng;

use crate::{
    circle::Circle, colours::Colour, gameObj::IGameObj, line::Line, point::Point, square::Square,
    triangle::Triangle, vector2::Vector2,
};

pub struct GameObjManager {
    pub gameObj: Vec<Box<dyn IGameObj>>,
    currentId: u64,
}

pub enum ObjectTypes {
    Triangle,
    Circle,
    Square,
    Ellipsis,
}

impl GameObjManager {
    pub fn new() -> GameObjManager {
        return GameObjManager {
            gameObj: vec![],
            currentId: 0,
        };
    }

    pub fn addGameObj(&mut self, gameObj: Box<dyn IGameObj>) {
        self.gameObj.push(gameObj);
    }

    pub fn getGameObj(&mut self, id: u64) -> &mut Box<dyn IGameObj> {
        let mut index = 0;
        for i in 0..self.gameObj.len() {
            if self.gameObj[i].getID() == id {
                index = i;
            }
        }
        return &mut self.gameObj[index as usize];
    }

    pub fn createTriangle(&mut self, p1: Point, p2: Point, p3: Point, colour: Colour) {
        self.currentId += 1;
        self.addGameObj(Box::new(Triangle::new(p1, p2, p3, colour, self.currentId)));
    }

    pub fn createSquare(&mut self, p1: Point, dim: Point, colour: Colour) {
        self.currentId += 1;
        self.addGameObj(Box::new(Square::new(
            p1,
            dim.x,
            dim.y,
            colour,
            self.currentId,
        )));
    }

    pub fn createCircle(&mut self, centre: Point, r: f64, colour: Colour) {
        self.currentId += 1;
        self.addGameObj(Box::new(Circle::new(
            centre.x,
            centre.y,
            r,
            colour,
            self.currentId,
        )))
    }

    pub fn createCircleS(&mut self, x: i32, y: i32, r: f64, colour: Colour) {
        self.createCircle(Point::newI(x, y), r, colour)
    }

    pub fn createLine(&mut self, p: Point, p2: Point, colour: Colour) {
        self.currentId += 1;
        println!("ID{}", self.currentId);
        self.addGameObj(Box::new(Line::new(p, p2, colour, self.currentId)));
    }

    pub fn createRandCircle(
        &mut self,
        xRange: (i32, i32),
        yRange: (i32, i32),
        rRange: (i32, i32),
        velRangeX: (i32, i32),
        velRangeY: (i32, i32),
    ) {
        self.currentId += 1;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(xRange.0..=xRange.1) as f64;
        let y = rng.gen_range(yRange.0..=yRange.1) as f64;
        let r = rng.gen_range(rRange.0..=rRange.1) as f64;
        let velX = rng.gen_range(velRangeX.0..=velRangeX.1);
        let velY = rng.gen_range(velRangeY.0..=velRangeY.1);
        let colour = Colour::new().createRandHSVA();
        let mut circle = Circle::new(x, y, r, colour, self.currentId);
        circle.setVelocity(Vector2::newI(velX, velY));
        self.addGameObj(Box::new(circle));
    }
}
