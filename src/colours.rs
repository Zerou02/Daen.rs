#[derive(Clone)]
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
