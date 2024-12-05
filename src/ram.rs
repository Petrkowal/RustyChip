use crate::datatypes::datatypes::*;

const MEMORY_SIZE: usize = 4096;
pub struct Ram {
    memory: [Byte; MEMORY_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            memory: [Byte(0); MEMORY_SIZE],
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
}