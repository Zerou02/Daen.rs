pub struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    colour: u32,
}

impl Line {
    pub fn new(x1: i32, x2: i32, y1: i32, y2: i32, colour: u32) -> Line {
        return Line {
            x1,
            x2,
            y1,
            y2,
            colour,
        };
    }
}

impl GameObj for Line {}
