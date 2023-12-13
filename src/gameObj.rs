use core::num;

use pixels::wgpu::BufferUsages;
use rand::seq;
use std::fmt::Debug;

use crate::{
    collisionBox::CollisionBox,
    colours::Colour,
    constants::{DEL_HEIGHT_L, DEL_HEIGHT_R, DEL_WIDTH_L, DEL_WIDTH_R},
    point::Point,
    renderer::Renderer,
    utils::rotatePoint,
    vector2::Vector2,
};

#[derive(Debug)]
pub struct BehaviourMap {
    velocity: Vector2,
    rotation: f64,
    h: u8,
    position: Vector2,
}

impl BehaviourMap {
    pub fn new() -> BehaviourMap {
        return BehaviourMap {
            velocity: Vector2::newI(0, 0),
            h: 0,
            position: Vector2::newI(0, 0),
            rotation: 0.0,
        };
    }

    pub fn newWithParam(vel: Vector2, h: u8, pos: Vector2, rotation: f64) -> BehaviourMap {
        return BehaviourMap {
            velocity: vel,
            h,
            position: pos,
            rotation,
        };
    }
}

#[derive(Debug)]
pub struct GameObj {
    pub rotation: f64,
    pub centre: Point,
    pub colour: Colour,
    pub points: Vec<Point>,
    pub filled: bool,
    pub id: String,
    pub velocity: Vector2,
    pub movesLeft: i32,
    pub mass: f64,
    pub behaviourMap: BehaviourMap,
}

impl GameObj {
    pub fn moveI(self: &mut Self, x: i64, y: i64) {
        self.moveF(x as f64, y as f64);
    }

    pub fn moveF(&mut self, x: f64, y: f64) {
        for p in &mut self.points {
            p.x += x;
            p.y += y;
        }
        self.centre.x += x;
        self.centre.y += y;
    }

    pub fn rotate(&mut self, rad: f64) {
        self.rotation += rad * 0.001;
        self.rotation %= 360.0;
        for p in &mut self.points {
            *p = rotatePoint(&p, rad * 0.001, &self.centre)
        }
    }

    pub fn getColour(&mut self) -> &mut Colour {
        return &mut self.colour;
    }

    pub fn setColour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    pub fn setFilled(&mut self, val: bool) {
        self.filled = val;
    }

    pub fn setRotation(&mut self, rad: f64) {
        self.rotate(-self.rotation);
        self.rotation = rad;
        self.rotate(self.rotation);
    }

    pub fn readyForTrial(&self) -> bool {
        self.points.clone().into_iter().all(|x| {
            return x.x < DEL_WIDTH_L
                || x.x > DEL_WIDTH_R
                || x.y < DEL_HEIGHT_L
                || x.y > DEL_HEIGHT_R;
        })
    }

    pub fn setVelocity(&mut self, v: Vector2) {
        self.velocity = v;
    }

    pub fn mMove(&mut self) {
        let v = &self.velocity;
        self.moveF(v.x, v.y);
    }

    pub fn getVelocity(&self) -> Vector2 {
        return self.velocity;
    }

    pub fn getVelocityMut(&mut self) -> Vector2 {
        return self.velocity;
    }

    pub fn getId(&self) -> String {
        return self.id.clone();
    }

    pub fn getMovesLeft(&self) -> i32 {
        return self.movesLeft;
    }

    pub fn setMovesLeft(&mut self, val: i32) {
        self.movesLeft = val;
    }

    pub fn getMass(&self) -> f64 {
        return self.mass;
    }

    pub fn applyBehaviour(&mut self) {
        let posChange = self.behaviourMap.position;
        self.moveF(posChange.x, posChange.y);
        self.rotate(self.behaviourMap.rotation);
        self.setVelocity(self.getVelocity().add(self.behaviourMap.velocity));
        self.colour.increaseRange(self.behaviourMap.h.into());
    }

    pub fn setBehaviourMap(&mut self, map: BehaviourMap) {
        self.behaviourMap = map;
    }
}

pub trait IGameObj: Debug {
    fn draw(&self, renderer: &mut Renderer);
    fn moveI(&mut self, x: i64, y: i64);
    fn moveF(&mut self, x: f64, y: f64) {
        self.moveI(x as i64, y as i64)
    }
    fn getColour(&mut self) -> &mut Colour;
    fn setColour(&mut self, colour: Colour);
    fn rotate(&mut self, rad: f64);
    fn setRotation(&mut self, rad: f64);
    fn setFilled(&mut self, val: bool);
    fn setCentre(&mut self, centre: Point);
    fn readyForTrial(&self) -> bool;
    fn getColBox(&self) -> &CollisionBox;
    fn getColBoxMut(&mut self) -> &mut CollisionBox;
    fn setVelocity(&mut self, v: Vector2);
    fn getVelocity(&self) -> Vector2;
    fn getID(&self) -> String;
    fn mMove(&mut self);
    fn getMovesLeft(&self) -> i32;
    fn setMovesLeft(&mut self, val: i32);
    fn getMass(&self) -> f64;
    fn setBehaviourMap(&mut self, map: BehaviourMap);
    fn applyBehaviour(&mut self);
}
