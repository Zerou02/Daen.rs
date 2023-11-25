use crate::{
    collisionBox::CollisionBox,
    colours::Colour,
    constants::{DEL_HEIGHT_L, DEL_HEIGHT_R, DEL_WIDTH_L, DEL_WIDTH_R},
    gameObj::{GameObj, IGameObj},
    point::Point,
    utils::rotatePoint,
    vector2::Vector2,
};

pub struct Ellipsis {
    gameObj: GameObj,
    distance: f64,
    colBox: CollisionBox,
}

impl Ellipsis {
    pub fn new(p1: Point, p2: Point, distance: f64, colour: Colour, id: u64) -> Ellipsis {
        return Ellipsis {
            gameObj: GameObj {
                rotation: 0.0,
                centre: p1.centreTo(&p2),
                colour,
                points: vec![p1, p2],
                filled: false,
                id,
                velocity: Vector2::newI(0, 0),
            },
            distance,
            colBox: CollisionBox::new(
                crate::collisionBox::CollisionBoxTypes::AABB,
                vec![],
                vec![],
                id,
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

    fn getID(&self) -> u64 {
        return self.gameObj.getId();
    }
}
