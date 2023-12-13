use rand::Rng;
use serde_json::{map::Values, Number, Value};

use crate::{
    circle::Circle,
    colours::Colour,
    ellipsis::Ellipsis,
    gameObj::{BehaviourMap, IGameObj},
    line::Line,
    point::Point,
    square::Square,
    triangle::Triangle,
    vector2::Vector2,
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

    pub fn getGameObj(&mut self, id: String) -> &mut Box<dyn IGameObj> {
        let mut index = 0;
        for i in 0..self.gameObj.len() {
            if self.gameObj[i].getID() == id {
                index = i;
            }
        }
        return &mut self.gameObj[index as usize];
    }

    pub fn createTriangle(
        &mut self,
        p1: Point,
        p2: Point,
        p3: Point,
        colour: Colour,
        id: String,
        colClass: String,
        collidesWith: Vec<String>,
    ) {
        self.currentId += 1;
        self.addGameObj(Box::new(Triangle::new(
            p1,
            p2,
            p3,
            colour,
            id,
            colClass,
            collidesWith,
        )));
    }

    pub fn createSquare(
        &mut self,
        p1: Point,
        dim: Point,
        colour: Colour,
        id: String,
        colClass: String,
        collidesWith: Vec<String>,
    ) {
        self.currentId += 1;
        self.addGameObj(Box::new(Square::new(
            p1,
            dim.x,
            dim.y,
            colour,
            id,
            colClass,
            collidesWith,
        )));
    }

    pub fn extractNumbers(&self, str: &str) -> Vec<f64> {
        let mut retVec: Vec<f64> = vec![];
        for x in str.replace(" ", "").replace("\"", "").split(",") {
            retVec.push(x.parse::<f64>().unwrap());
        }
        return retVec;
    }

    pub fn removeEscapesSeq(&self, str: &str) -> String {
        return (str.replace("\"", ""));
    }
    pub fn parseConfig(&mut self, config: Vec<Value>) {
        for x in config {
            let entryType = &x["type"];

            let vel = match x.get("behaviours") {
                None => Vector2::newI(0, 0),
                Some(a) => Vector2::newI(1, 1),
            };
            let map = self.parseBehaviours(&x);
            let mut gameObj: Box<dyn IGameObj> = match x.get("type") {
                None => panic!("WE need a type to work properly"),
                Some(t) => match self.removeEscapesSeq(t.as_str().unwrap()).as_str() {
                    "Circle" => Box::new(self.parseCircleFromJSON(&x)),
                    "Ellipse" => Box::new(self.parseEllipseFromJSON(&x)),
                    "Line" => Box::new(self.parseLineFromJSON(&x)),
                    "Square" => Box::new(self.parseSquareFromJSON(&x)),
                    "Triangle" => Box::new(self.parseTriangleFromJSON(&x)),
                    _ => {
                        panic!("Wrong, misspelled or forgotten type")
                    }
                },
            };
            gameObj.setBehaviourMap(map);
            self.addGameObj(gameObj);
        }
    }

    pub fn parseVec2(&self, str: &Value, key: &str) -> Vector2 {
        match str.get(key) {
            None => Vector2::newI(0, 0),
            Some(a) => {
                let n = self.extractNumbers(a.as_str().unwrap());
                Vector2::new(n[0], n[1])
            }
        }
    }

    pub fn parseFloat(&self, str: &Value, key: &str) -> f64 {
        match str.get(key) {
            None => 0.0,
            Some(a) => {
                let n = self.extractNumbers(a.as_str().unwrap());
                n[0]
            }
        }
    }

    pub fn parseString(&self, str: &Value, key: &str) -> String {
        match str.get(key) {
            None => "".to_owned(),
            Some(a) => self.removeEscapesSeq(&a.as_str().unwrap()),
        }
    }

    pub fn parseStrVec(&self, str: &Value, key: &str) -> Vec<String> {
        match str.get(key) {
            None => vec![],
            Some(a) => {
                let b = a.clone();
                let mut retVec: Vec<String> = vec![];
                for x in b.as_str().unwrap().split(",") {
                    retVec.push(self.removeEscapesSeq(x));
                }
                return retVec;
            }
        }
    }

    pub fn parseColour(&self, str: &Value, key: &str) -> Colour {
        let mut c = Colour::new();
        match str.get(key) {
            None => {
                c = Colour::new();
            }
            Some(a) => match self.removeEscapesSeq(a.as_str().clone().unwrap()).as_str() {
                "green" => {
                    c.createFromString(crate::colours::ColourType::GREEN);
                }
                "red" => c.createFromString(crate::colours::ColourType::RED),
                "blue" => c.createFromString(crate::colours::ColourType::BLUE),
                "cyan" => c.createFromString(crate::colours::ColourType::CYAN),
                "yellow" => c.createFromString(crate::colours::ColourType::YELLOW),
                "orange" => c.createFromString(crate::colours::ColourType::ORANGE),
                "white" => c.createFromString(crate::colours::ColourType::WHITE),
                "black" => c.createFromString(crate::colours::ColourType::BLACK),
                "pink" => c.createFromString(crate::colours::ColourType::PINK),
                "purple" => c.createFromString(crate::colours::ColourType::PURPLE),
                "brown" => c.createFromString(crate::colours::ColourType::BROWN),
                _ => {
                    let c = Colour::new();
                }
            },
        };
        return c;
    }

    fn getHsvaVal(&self, colour: &str) -> [u8; 4] {
        let mut c = Colour::new();
        match colour {
            "green" => {
                c.createFromString(crate::colours::ColourType::GREEN);
            }
            "red" => c.createFromString(crate::colours::ColourType::RED),
            "blue" => c.createFromString(crate::colours::ColourType::BLUE),
            "cyan" => c.createFromString(crate::colours::ColourType::CYAN),
            "yellow" => c.createFromString(crate::colours::ColourType::YELLOW),
            "orange" => c.createFromString(crate::colours::ColourType::ORANGE),
            "white" => c.createFromString(crate::colours::ColourType::WHITE),
            "black" => c.createFromString(crate::colours::ColourType::BLACK),
            "pink" => c.createFromString(crate::colours::ColourType::PINK),
            "purple" => c.createFromString(crate::colours::ColourType::PURPLE),
            "brown" => c.createFromString(crate::colours::ColourType::BROWN),
            _ => {
                let c = Colour::new();
            }
        };
        return c.hsva;
    }
    pub fn parseColourRange(&self, str: &Value, key: &str) -> Vec<[u8; 4]> {
        let mut retVec: Vec<[u8; 4]> = vec![];
        match str.get(key) {
            None => {}
            Some(a) => {
                for x in str[key].as_str().unwrap().split(",") {
                    retVec.push(self.getHsvaVal(x))
                }
            }
        }
        return retVec;
    }

    pub fn parseBoolean(&self, str: &Value, key: &str) -> bool {
        match str.get(key) {
            None => false,
            Some(a) => {
                return a.as_bool().unwrap();
            }
        }
    }

    pub fn parseTriangleFromJSON(&self, str: &Value) -> Triangle {
        let id = self.parseString(str, "id");
        let p1: Vector2 = self.parseVec2(str, "p1");
        let p2: Vector2 = self.parseVec2(str, "p2");
        let p3: Vector2 = self.parseVec2(str, "p3");
        let vel: Vector2 = self.parseVec2(str, "velocity");
        let rotation = self.parseFloat(str, "rotation");
        let radius = self.parseFloat(str, "radius");
        let collisionClass = self.parseString(str, "collisionClass");
        let collidesWith = self.parseStrVec(str, "collidesWith");
        let colour = self.parseColour(str, "colour");
        let filled = self.parseBoolean(str, "filled");
        let ranges = self.parseColourRange(str, "range");
        let mut c = Triangle::new(
            p1.toPoint(),
            p2.toPoint(),
            p3.toPoint(),
            colour,
            id,
            collisionClass,
            collidesWith,
        );
        let newColour = c.getColour().clone().setRanges(ranges);
        c.setColour(newColour);
        c.setRotation(rotation);
        c.setVelocity(vel);
        c.setFilled(filled);
        return c;
    }

    pub fn parseSquareFromJSON(&self, str: &Value) -> Square {
        let id = self.parseString(str, "id");
        let pivot: Vector2 = self.parseVec2(str, "pivot");
        let width = self.parseFloat(str, "w");
        let height = self.parseFloat(str, "h");
        let vel: Vector2 = self.parseVec2(str, "velocity");
        let rotation = self.parseFloat(str, "rotation");
        let radius = self.parseFloat(str, "radius");
        let collisionClass = self.parseString(str, "collisionClass");
        let collidesWith = self.parseStrVec(str, "collidesWith");
        let colour = self.parseColour(str, "colour");
        let filled = self.parseBoolean(str, "filled");
        let ranges = self.parseColourRange(str, "range");
        let mut c = Square::new(
            pivot.toPoint(),
            width,
            height,
            colour,
            id,
            collisionClass,
            collidesWith,
        );
        let newColour = c.getColour().clone().setRanges(ranges);
        c.setColour(newColour);
        c.setRotation(rotation);
        c.setVelocity(vel);
        c.setFilled(filled);
        return c;
    }

    pub fn parseLineFromJSON(&self, str: &Value) -> Line {
        let id = self.parseString(str, "id");
        let p1: Vector2 = self.parseVec2(str, "p1");
        let p2: Vector2 = self.parseVec2(str, "p2");
        let vel: Vector2 = self.parseVec2(str, "velocity");
        let rotation = self.parseFloat(str, "rotation");
        let radius = self.parseFloat(str, "radius");
        let collisionClass = self.parseString(str, "collisionClass");
        let collidesWith = self.parseStrVec(str, "collidesWith");
        let colour = self.parseColour(str, "colour");
        let filled = self.parseBoolean(str, "filled");
        let ranges = self.parseColourRange(str, "range");
        let mut c = Line::new(
            p1.toPoint(),
            p2.toPoint(),
            colour,
            id,
            collisionClass,
            collidesWith,
        );
        let newColour = c.getColour().clone().setRanges(ranges);
        c.setColour(newColour);
        c.setRotation(rotation);
        c.setVelocity(vel);
        c.setFilled(filled);
        return c;
    }

    pub fn parseEllipseFromJSON(&self, str: &Value) -> Ellipsis {
        let id = self.parseString(str, "id");
        let lp: Vector2 = self.parseVec2(str, "lp");
        let rp: Vector2 = self.parseVec2(str, "rp");
        let vel: Vector2 = self.parseVec2(str, "velocity");
        let rotation = self.parseFloat(str, "rotation");
        let radius = self.parseFloat(str, "radius");
        let collisionClass = self.parseString(str, "collisionClass");
        let collidesWith = self.parseStrVec(str, "collidesWith");
        let colour = self.parseColour(str, "colour");
        let filled = self.parseBoolean(str, "filled");
        let ranges = self.parseColourRange(str, "range");
        let mut c = Ellipsis::new(
            lp.toPoint(),
            rp.toPoint(),
            radius,
            colour,
            id,
            collisionClass,
            collidesWith,
        );
        let newColour = c.getColour().clone().setRanges(ranges);
        c.setColour(newColour);
        c.setRotation(rotation);
        c.setVelocity(vel);
        c.setFilled(filled);
        return c;
    }

    pub fn parseCircleFromJSON(&self, str: &Value) -> Circle {
        let id = self.parseString(str, "id");
        let centre: Vector2 = self.parseVec2(str, "centre");
        let vel: Vector2 = self.parseVec2(str, "velocity");

        let rotation = self.parseFloat(str, "rotation");
        let radius = self.parseFloat(str, "radius");
        let collisionClass = self.parseString(str, "collisionClass");
        let collidesWith = self.parseStrVec(str, "collidesWith");
        let colour = self.parseColour(str, "colour");
        let filled = self.parseBoolean(str, "filled");
        let range = self.parseColourRange(str, "range");
        let mut c = Circle::new(
            centre.x,
            centre.y,
            radius,
            colour,
            id,
            collisionClass,
            collidesWith,
        );
        let newColour = c.getColour().clone().setRanges(range);
        c.setColour(newColour);
        c.setRotation(rotation);
        c.setFilled(filled);
        c.setVelocity(vel);
        return c;
    }

    pub fn parseBehaviours(&self, str: &Value) -> BehaviourMap {
        match str.get("behaviours") {
            None => BehaviourMap::new(),
            Some(map) => {
                let vel = self.parseVec2(map, "velocity");
                let h = self.parseFloat(map, "colour");
                let pos = self.parseVec2(map, "position");
                let rot: f64 = self.parseFloat(map, "rotation");
                BehaviourMap::newWithParam(vel, h as u8, pos, rot)
            }
        }
    }

    pub fn parseCircle(
        &mut self,
        id: String,
        centre: Point,
        radius: f64,
        colClass: String,
        collidesWith: Vec<String>,
        rotation: f64,
        colour: Colour,
        behaviourMap: BehaviourMap,
    ) {
        let mut c = Circle::new(
            centre.x,
            centre.y,
            radius,
            colour,
            id,
            colClass,
            collidesWith,
        );
        c.setBehaviourMap(behaviourMap);
        self.addGameObj(Box::new(c));
    }

    pub fn createCircle(
        &mut self,
        centre: Point,
        r: f64,
        colour: Colour,
        id: String,
        colClass: String,
        collidesWith: Vec<String>,
    ) {
        self.addGameObj(Box::new(Circle::new(
            centre.x,
            centre.y,
            r,
            colour,
            id,
            colClass,
            collidesWith,
        )));
    }

    pub fn createCircleS(
        &mut self,
        x: i32,
        y: i32,
        r: f64,
        colour: Colour,
        id: String,
        colClass: String,
        collidesWith: Vec<String>,
    ) {
        self.createCircle(Point::newI(x, y), r, colour, id, colClass, collidesWith)
    }

    pub fn createEllipsis(
        &mut self,
        p1: Point,
        p2: Point,
        dist: f64,
        colour: Colour,
        id: String,
        colClass: String,
        collidesWith: Vec<String>,
    ) {
        self.currentId += 1;
        self.addGameObj(Box::new(Ellipsis::new(
            p1,
            p2,
            dist,
            colour,
            id,
            colClass,
            collidesWith,
        )))
    }

    pub fn createLine(
        &mut self,
        p: Point,
        p2: Point,
        colour: Colour,
        id: String,
        colClass: String,
        collidesWith: Vec<String>,
    ) {
        self.currentId += 1;
        self.addGameObj(Box::new(Line::new(
            p,
            p2,
            colour,
            id,
            colClass,
            collidesWith,
        )));
    }

    pub fn createRandCircle(
        &mut self,
        xRange: (i32, i32),
        yRange: (i32, i32),
        rRange: (i32, i32),
        velRangeX: (i32, i32),
        velRangeY: (i32, i32),
        id: String,
        colClass: String,
        collidesWith: Vec<String>,
    ) {
        self.currentId += 1;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(xRange.0..=xRange.1) as f64;
        let y = rng.gen_range(yRange.0..=yRange.1) as f64;
        let r = rng.gen_range(rRange.0..=rRange.1) as f64;
        let velX = rng.gen_range(velRangeX.0..=velRangeX.1);
        let velY = rng.gen_range(velRangeY.0..=velRangeY.1);
        let mut colour = Colour::new();
        colour.createRandHSVA();
        let mut circle = Circle::new(x, y, r, colour, id, colClass, collidesWith);
        circle.setVelocity(Vector2::newI(velX, velY));
        self.addGameObj(Box::new(circle));
    }
}
