use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Byte(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Address(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nibble(pub u8);

impl Byte {
    pub fn new(value: u8) -> Byte {
        Byte(value)
    }

    pub fn as_u8(self) -> u8 {
        self.0
    }
    
    pub fn as_usize(self) -> usize {
        self.0 as usize
    }

    pub fn add(self, rhs: Byte) -> Result<Byte, Byte> {
        if let Some(result) = self.0.checked_add(rhs.0) {
            Ok(Byte(result))
        } else {
            Err(Byte((self.0.wrapping_add(rhs.0)) & 0xFF))
        }
    }
    
    pub fn sub(self, rhs: Byte) -> Result<Byte, Byte> {
        if let Some(result) = self.0.checked_sub(rhs.0) {
            Ok(Byte(result))
        } else {
            Err(Byte((self.0.wrapping_sub(rhs.0)) & 0xFF))
        }
    }
    
}

impl Default for Byte {
    fn default() -> Byte {
        Byte(0)
    }
}

impl Address {
    pub fn new(value: u16) -> Address {
        Address(Address::mask(value))
    }

    pub fn as_u16(self) -> u16 {
        Address::mask(self.0)
    }
    
    pub fn as_usize(self) -> usize {
        Address::mask(self.0) as usize
    }

    pub fn mask(mask: u16) -> u16 {
        mask & 0xFFF
    }
}

impl Default for Address {
    fn default() -> Address {
        Address(0)
    }
}

impl Nibble {
    pub fn new(value: u8) -> Nibble {
        Nibble(value & 0xF)
    }

    pub fn as_u8(self) -> u8 {
        self.0 & 0xF
    }
}

impl Default for Nibble {
    fn default() -> Nibble {
        Nibble(0)
    }
}


impl Add<u8> for &Address {
    type Output = Address;

    fn add(self, rhs: u8) -> Self::Output {
        Address::new(self.as_u16() + rhs as u16)
    }
}

impl Add<usize> for &Address {
    type Output = Address;

    fn add(self, rhs: usize) -> Self::Output {
        Address::new((self.as_usize() + rhs) as u16)
    }
}
