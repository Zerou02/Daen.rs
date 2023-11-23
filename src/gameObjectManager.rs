use crate::gameObj::IGameObj;

pub struct GameObjManager {
    pub gameObj: Vec<Box<dyn IGameObj>>,
}

pub enum ObjectTypes {
    Triangle,
    Circle,
    Square,
    Ellipsis,
}

impl GameObjManager {
    pub fn new() -> GameObjManager {
        return GameObjManager { gameObj: vec![] };
    }

    pub fn addGameObj(&self, gameObj: Box<dyn IGameObj>) {
        self.gameObj.push(gameObj);
    }

    pub fn createTriangle(typee: ObjectTypes) {
        match typee {
            ObjectTypes::Triangle => todo!(),
            ObjectTypes::Circle => todo!(),
            ObjectTypes::Square => todo!(),
            ObjectTypes::Ellipsis => todo!(),
        }
    }
}
