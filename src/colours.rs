use rand::prelude::*;

#[derive(Debug)]
pub enum ColourType {
    BLACK,
    WHITE,
    RED,
    GREEN,
    BLUE,
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

#[derive(Debug, Clone, Copy)]
pub struct Colour {
    pub rgba: [u8; 4],
    pub hsva: [u8; 4],
}

impl Colour {
    pub fn new() -> Colour {
        return Colour {
            rgba: [0, 0, 0, 0],
            hsva: [0, 0, 0, 0],
        };
    }
    pub fn createRandHSVA(mut self) -> Colour {
        let rng = thread_rng();
        let h = thread_rng().gen_range(0..=255);
        let s = thread_rng().gen_range(0..=255);
        let v = thread_rng().gen_range(0..=255);
        let a = thread_rng().gen_range(0..255);
        self.hsva = [h, s, v, a];
        self.convertHSVAToRGBA();
        return self;
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
}
