use crate::{
    collisionBox::CollisionBox,
    colours::Colour,
    constants::{DEL_HEIGHT_L, DEL_HEIGHT_R, DEL_WIDTH_L, DEL_WIDTH_R},
    gameObj::{BehaviourMap, GameObj, IGameObj},
    point::Point,
    utils::rotatePoint,
    vector2::Vector2,
};

#[derive(Debug)]
pub struct Ellipsis {
    gameObj: GameObj,
    distance: f64,
    colBox: CollisionBox,
}

impl Ellipsis {
    pub fn new(
        p1: Point,
        p2: Point,
        distance: f64,
        colour: Colour,
        id: String,
        colClass: String,
        collidesWith: Vec<String>,
    ) -> Ellipsis {
        return Ellipsis {
            gameObj: GameObj {
                rotation: 0.0,
                centre: p1.centreTo(&p2),
                colour,
                points: vec![p1, p2],
                filled: false,
                id: id.clone(),
                velocity: Vector2::newI(0, 0),
                mass: distance,
                movesLeft: 0,
                behaviourMap: BehaviourMap::new(),
            },
            distance,
            colBox: CollisionBox::new(
                crate::collisionBox::CollisionBoxTypes::Ellipsis,
                vec![p1, p2],
                vec![distance],
                id.clone(),
                colClass,
                collidesWith,
            ),
        };
    }
}

impl IGameObj for Ellipsis {
    fn draw(&self, renderer: &mut crate::renderer::Renderer) {
        if self.gameObj.filled {
            renderer.fillEllipsis(
                &self.gameObj.points[0],
                &self.gameObj.points[1],
                self.distance as i32,
                self.gameObj.colour.rgba,
            );
        } else {
            renderer.drawEllipsis(
                &self.gameObj.points[0],
                &self.gameObj.points[1],
                self.distance as i32,
                self.gameObj.colour.rgba,
            )
        }
    }
    fn moveI(self: &mut Self, x: i64, y: i64) {
        self.gameObj.moveI(x, y);
    }

    fn getColour(&mut self) -> &mut Colour {
        return self.gameObj.getColour();
    }

    fn setColour(&mut self, colour: Colour) {
        self.gameObj.setColour(colour);
    }

    fn rotate(&mut self, rad: f64) {
        self.gameObj.rotate(rad);
    }

    fn setRotation(&mut self, rad: f64) {
        self.gameObj.setRotation(rad);
    }

    fn setFilled(&mut self, val: bool) {
        self.gameObj.setFilled(val);
    }

    fn setCentre(&mut self, centre: Point) {
        self.gameObj.centre = centre;
    }

    fn readyForTrial(&self) -> bool {
        self.gameObj.points.clone().into_iter().all(|x| {
            return x.x + self.distance < DEL_WIDTH_L
                || x.x + self.distance > DEL_WIDTH_R
                || x.y + self.distance < DEL_HEIGHT_L
                || x.y + self.distance > DEL_HEIGHT_R;
        })
    }

    fn getColBox(&self) -> &CollisionBox {
        return &self.colBox;
    }

    fn getColBoxMut(&mut self) -> &mut CollisionBox {
        return &mut self.colBox;
    }

    fn setVelocity(&mut self, v: Vector2) {
        self.gameObj.setVelocity(v)
    }

    fn getVelocity(&self) -> Vector2 {
        return self.gameObj.getVelocity();
    }

    fn mMove(&mut self) {
        self.gameObj.mMove();
    }

    fn getID(&self) -> String {
        return self.gameObj.getId();
    }

    fn getMovesLeft(&self) -> i32 {
        return self.gameObj.getMovesLeft();
    }

    fn setMovesLeft(&mut self, val: i32) {
        self.gameObj.setMovesLeft(val)
    }

    fn getMass(&self) -> f64 {
        return self.gameObj.getMass();
    }

    fn setBehaviourMap(&mut self, map: BehaviourMap) {
        self.gameObj.setBehaviourMap(map);
    }

    fn applyBehaviour(&mut self) {
        self.gameObj.applyBehaviour();
        self.colBox.points = self.gameObj.points.clone();
    }

    fn moveF(&mut self, x: f64, y: f64) {
        self.moveI(x as i64, y as i64)
    }
}
