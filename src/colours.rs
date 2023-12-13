use rand::prelude::*;

#[derive(Debug)]
pub enum ColourType {
    BLACK,
    WHITE,
    RED,
    GREEN,
    BLUE,
    CYAN,
    YELLOW,
    ORANGE,
    PINK,
    PURPLE,
    BROWN,
}

pub fn getColourVal(colourType: ColourType) -> u32 {
    return match colourType {
        ColourType::BLACK => 0x00000000,
        ColourType::WHITE => 0xffffffff,
        ColourType::RED => 0xff0000aa,
        ColourType::GREEN => 0x00ff00aa,
        ColourType::BLUE => 0x0000ffaa,
        _ => panic!("IMPLEMENT ME"),
    };
}
#[derive(Debug, Clone)]
pub struct Colour {
    pub rgba: [u8; 4],
    pub hsva: [u8; 4],
    pub ranges: Vec<[u8; 4]>,
    pub currRangePtr: usize,
}

impl Colour {
    pub fn new() -> Colour {
        return Colour {
            rgba: [0, 0, 0, 0],
            hsva: [0, 0, 0, 0],
            ranges: vec![],
            currRangePtr: 0,
        };
    }

    pub fn setRanges(mut self, ranges: Vec<[u8; 4]>) -> Self {
        self.ranges = ranges;
        self.currRangePtr = 0;
        return self;
    }

    pub fn createFromVal(&mut self, h: u8, s: u8, v: u8, a: u8) {
        self.hsva = [h, s, v, a];
        self.convertHSVAToRGBA();
    }

    pub fn createFromH(&mut self, hue: u8) {
        let s = 255;
        let v = 127;
        let a = 255;
        self.hsva = [hue, s, v, a];
        self.convertHSVAToRGBA();
    }

    pub fn getU8Val(&self, h: i32) -> u8 {
        return (h as f64 / 360.0 * 255.0) as u8;
    }

    pub fn createFromString(&mut self, str: ColourType) {
        return match str {
            ColourType::BLUE => self.createFromH(self.getU8Val(246)),
            ColourType::GREEN => self.createFromH(self.getU8Val(113)),
            ColourType::RED => self.createFromH(255),
            ColourType::CYAN => self.createFromH(self.getU8Val(178)),
            ColourType::YELLOW => self.createFromH(self.getU8Val(61)),
            ColourType::ORANGE => self.createFromH(self.getU8Val(27)),
            ColourType::BLACK => self.createFromVal(0, 0, 0, 0),
            ColourType::WHITE => self.createFromVal(0, 0, 255, 255),
            ColourType::PINK => self.createFromH(self.getU8Val(291)),
            ColourType::PURPLE => self.createFromH(self.getU8Val(263)),
            ColourType::BROWN => {
                self.createFromVal(self.getU8Val(31), 255, (29.0 / 100.0 * 255.0) as u8, 255)
            }
        };
    }

    pub fn createRandHSVA(&mut self) {
        let rng = thread_rng();
        let h = thread_rng().gen_range(0..=255);
        self.createFromH(h);
    }

    pub fn setHSVA(mut self, val: [u8; 4]) -> Colour {
        self.hsva = val;
        self.convertHSVAToRGBA();
        return self;
    }

    pub fn convertHSVAToRGBA(&mut self) -> &mut Colour {
        let h = self.hsva[0] as f64 / 255.0 * 360.0;
        let s = self.hsva[1] as f64 / 255.0;
        let v = self.hsva[2] as f64 / 255.0;
        let a = self.hsva[3] as f64;

        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let [r, g, b, newA] = match h as u32 {
            0..=59 => [c, x, m, a],
            60..=119 => [x, c, m, a],
            120..=179 => [m, c, x, a],
            180..=239 => [m, x, c, a],
            240..=299 => [x, m, c, a],
            300..=360 => [c, m, x, a],
            _ => panic!("Colour too powerful to display"),
        };
        let newR = ((r + m) * 255.0) as u8;
        let newG = ((g + m) * 255.0) as u8;
        let newB = ((b + m) * 255.0) as u8;
        self.rgba = [newR, newG, newB, newA as u8];
        return self;
    }

    pub fn increaseHSVA(&mut self, val: i32) -> &mut Self {
        let mut newH = self.hsva[0] as i32 + val;
        while newH > 255 {
            newH -= 255;
        }
        if newH < 0 {
            newH = 255;
        }
        self.hsva[0] = newH as u8;
        self.convertHSVAToRGBA();
        return self;
    }

    pub fn increaseRange(&mut self, val: i32) {
        if (self.ranges.len() == 0) {
            self.increaseHSVA(val);
            return;
        }
        let nextRange = if (self.currRangePtr == self.ranges.len() - 1) {
            self.ranges[0].clone()
        } else {
            self.ranges[self.currRangePtr + 1].clone()
        };
        let currRange = self.ranges[self.currRangePtr].clone();
        let mut differenceVec: [i16; 4] = [0, 0, 0, 0];
        for i in 0..4 {
            let difference: i16 = (nextRange[i] as i16 - currRange[i] as i16);
            differenceVec[i] = if (difference != 0) {
                difference / difference
            } else {
                0
            };
        }
        let mut amountSame = 0;
        for i in 0..4 {
            self.hsva[i] = (differenceVec[i] + self.hsva[i] as i16) as u8;
            if (self.hsva[i] == self.ranges[self.currRangePtr][i]) {
                amountSame += 1;
            }
        }

        if (amountSame == 4) {
            self.currRangePtr = if (self.currRangePtr == self.ranges.len()) {
                0
            } else {
                self.currRangePtr + 1
            };
        }
        self.convertHSVAToRGBA();
    }
}
