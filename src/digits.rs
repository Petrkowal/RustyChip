
pub enum Digits {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Digits {
    pub const fn sprite(&self) -> [u8; 5] {
        match self {
            Digits::Zero => [0xF0, 0x90, 0x90, 0x90, 0xF0],
            Digits::One => [0x20, 0x60, 0x20, 0x20, 0x70],
            Digits::Two => [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            Digits::Three => [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            Digits::Four => [0x90, 0x90, 0xF0, 0x10, 0x10],
            Digits::Five => [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            Digits::Six => [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            Digits::Seven => [0xF0, 0x10, 0x20, 0x40, 0x40],
            Digits::Eight => [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            Digits::Nine => [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            Digits::A => [0xF0, 0x90, 0xF0, 0x90, 0x90],
            Digits::B => [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            Digits::C => [0xF0, 0x80, 0x80, 0x80, 0xF0],
            Digits::D => [0xE0, 0x90, 0x90, 0x90, 0xE0],
            Digits::E => [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            Digits::F => [0xF0, 0x80, 0xF0, 0x80, 0x80],
        }
    }
    
    pub const fn from_usize(value: usize) -> Digits {
        match value {
            0 => Digits::Zero,
            1 => Digits::One,
            2 => Digits::Two,
            3 => Digits::Three,
            4 => Digits::Four,
            5 => Digits::Five,
            6 => Digits::Six,
            7 => Digits::Seven,
            8 => Digits::Eight,
            9 => Digits::Nine,
            10 => Digits::A,
            11 => Digits::B,
            12 => Digits::C,
            13 => Digits::D,
            14 => Digits::E,
            15 => Digits::F,
            _ => panic!("Invalid digit"),
        }
    }
    
}
