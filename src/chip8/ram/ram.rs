use crate::chip8::datatypes::datatypes::*;
use crate::chip8::io::digits::Digits;
use std::fs::File;
use std::io::Read;

const MEMORY_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ram {
    memory: [Byte; MEMORY_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        let mut ram: Ram = Ram {
            memory: [Byte(0); MEMORY_SIZE],
        };

        Ram::load_font(&mut ram);

        ram
    }

    fn load_font(ram: &mut Ram) {
        for i in 0..16 {
            let sprite = Digits::sprite(&Digits::from_usize(i));
            for j in 0..5 {
                ram.load(Address(0x50 + i as u16 * 5 + j as u16), Byte(sprite[j]));
            }
        }
    }

    pub fn load(&mut self, address: Address, value: Byte) {
        self.memory[address.as_usize()] = value;
    }

    pub fn read(&self, address: Address) -> Byte {
        self.memory[address.as_usize()]
    }

    pub fn read_word(&self, address: &Address) -> u16 {
        let first = self.memory[address.as_usize()].0 as u16;
        let second = self.memory[address.as_usize() + 1].0 as u16;
        (first << 8) | second
    }

    pub fn read_word_as_bytes(&self, address: &Address) -> (Byte, Byte) {
        let first = self.memory[address.as_usize()];
        let second = self.memory[address.as_usize() + 1];
        (first, second)
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        let mut file = File::open(rom_path).expect("Failed to open file");
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");

        for i in 0..buffer.len() {
            self.load(Address(0x200 + i as u16), Byte(buffer[i]));
        }
    }
}
