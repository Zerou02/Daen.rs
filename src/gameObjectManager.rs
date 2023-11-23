use crate::{
    circle::Circle, colours::Colour, gameObj::IGameObj, point::Point, square::Square,
    triangle::Triangle,
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
        return &mut self.gameObj[id as usize];
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
}
