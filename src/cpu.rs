use std::ops::Add;
use crate::datatypes::datatypes::*;
use crate::display::Display;
use crate::instruction::*;
use crate::ram::Ram;
use crate::registers::*;
use crate::stack::Stack;
use crate::timers::Timers;
use crate::util::*;

const REGISTER_COUNT: usize = 16;

pub(crate) struct Cpu {
    registers: [Register<VRegisterMarker, Byte>; REGISTER_COUNT],
    i: Register<IRegisterMarker, Address>,
    stack: Stack,
    ram: Ram,
    pc: Register<PCRegisterMarker, Address>,
    vf: Register<FRegisterMarker, Byte>,
    timers: Timers,
    display: Display,
}

impl Cpu {
    pub(crate) fn new(ram: Ram) -> Cpu {
        let registers: [Register<VRegisterMarker, Byte>; 16] = [Register::new(0); 16];
        let i = Register::new(0);
        let stack = Stack::new();
        let pc = Register::new(0);
        let vf = Register::new(0);
        let timers = Timers::new();
        let display = Display::new();
        Cpu {
            registers,
            i,
            stack,
            ram,
            pc,
            vf,
            timers,
            display,
        }
    }
    
    pub fn cycle(&mut self) {
        let opcode = self.fetch();
        let instruction = self.decode(opcode);
        self.execute(instruction);
    }

    fn fetch(&mut self) -> u16 {
        let word = self.ram.read_word(self.pc.value());
        self.pc.increment();
        word
    }

    fn decode(&mut self, opcode: u16) -> Instruction {
        match opcode {
            0x00E0 => Instruction::CLS,
            0x00EE => Instruction::RET,
            0x0000..=0x0FFF => Instruction::SYS(Address::new(Address::mask(opcode))),
            0x1000..=0x1FFF => Instruction::JP(Address::new(Address::mask(opcode))),
            0x2000..=0x2FFF => Instruction::CALL(Address::new(Address::mask(opcode))),
            0x3000..=0x3FFF => {
                let reg = get_hex_digit_usize(opcode, 1, 1);
                let val = get_hex_digit_u8(opcode, 2, 2);
                Instruction::SE(reg, Byte(val))
            }
            0x4000..=0x4FFF => {
                let reg = get_hex_digit_usize(opcode, 1, 1);
                let val = get_hex_digit_u8(opcode, 2, 2);
                Instruction::SNE(reg, Byte(val))
            }
            0x5000..=0x5FFF => {
                let reg1 = get_hex_digit_usize(opcode, 1, 1);
                let reg2 = get_hex_digit_usize(opcode, 2, 1);
                Instruction::SEV(reg1, reg2)
            }
            0x6000..=0x6FFF => {
                let reg = get_hex_digit_usize(opcode, 1, 1);
                let val = get_hex_digit_u8(opcode, 2, 2);
                Instruction::LD(reg, Byte(val))
            }
            0x7000..=0x7FFF => {
                let reg = get_hex_digit_usize(opcode, 1, 1);
                let val = get_hex_digit_u8(opcode, 2, 2);
                Instruction::ADD(reg, Byte(val))
            }
            0x8000..=0x8FFF => {
                let reg1 = get_hex_digit_usize(opcode, 1, 1);
                let reg2 = get_hex_digit_usize(opcode, 2, 1);
                match get_hex_digit(opcode, 3, 1) {
                    0x0 => Instruction::LDV(reg1, reg2),
                    0x1 => Instruction::OR(reg1, reg2),
                    0x2 => Instruction::AND(reg1, reg2),
                    0x3 => Instruction::XOR(reg1, reg2),
                    0x4 => Instruction::ADDV(reg1, reg2),
                    0x5 => Instruction::SUB(reg1, reg2),
                    0x6 => Instruction::SHR(reg1, reg2),
                    0x7 => Instruction::SUBN(reg1, reg2),
                    0xE => Instruction::SHL(reg1, reg2),
                    _ => unreachable!("Invalid opcode starting with 8"),
                }
            }
            0x9000..=0x9FFF => {
                let reg1 = get_hex_digit_usize(opcode, 1, 1);
                let reg2 = get_hex_digit_usize(opcode, 2, 1);
                Instruction::SNEV(reg1, reg2)
            }
            0xA000..=0xAFFF => Instruction::LDI(Address::new(Address::mask(opcode))),
            0xB000..=0xBFFF => Instruction::JPV0(Address::new(Address::mask(opcode))),
            0xC000..=0xCFFF => {
                let reg = get_hex_digit_usize(opcode, 1, 1);
                let val = get_hex_digit_u8(opcode, 2, 2);
                Instruction::RND(reg, Byte(val))
            }
            0xD000..=0xDFFF => {
                let reg1 = get_hex_digit_usize(opcode, 1, 1);
                let reg2 = get_hex_digit_usize(opcode, 2, 1);
                let nibble = get_hex_digit_u8(opcode, 3, 1);
                Instruction::DRW(reg1, reg2, Nibble(nibble))
            }
            0xE000..=0xEFFF => {
                let reg = get_hex_digit_usize(opcode, 1, 1);
                match get_hex_digit(opcode, 2, 2) {
                    0x9E => Instruction::SKP(reg),
                    0xA1 => Instruction::SKNP(reg),
                    _ => unreachable!("Invalid opcode"),
                }
            }
            0xF000..=0xFFFF => {
                let reg = get_hex_digit_usize(opcode, 1, 1);
                match get_hex_digit(opcode, 2, 2) {
                    0x07 => Instruction::LDDT(reg),
                    0x0A => Instruction::LDK(reg),
                    0x15 => Instruction::LDDTV(reg),
                    0x18 => Instruction::LDST(reg),
                    0x1E => Instruction::ADDI(reg),
                    0x29 => Instruction::LDF(reg),
                    0x33 => Instruction::LDB(reg),
                    0x55 => Instruction::LDIV(reg),
                    0x65 => Instruction::LDVI(reg),
                    _ => unreachable!("Invalid opcode starting with F"),
                }
            }
            _ => unreachable!("Invalid opcode, opcode: {:#X} not matched", opcode),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::SYS(addr) => {
                self.pc.jump(addr);
            }
            Instruction::CLS => {
                self.display.clear();
            }
            Instruction::RET => {
                let addr = self.stack.pop().expect("Stack underflow");
                self.pc.jump(addr);
            }
            Instruction::JP(addr) => {
                self.pc.jump(addr);
            }
            Instruction::CALL(addr) => {
                self.stack.push(*self.pc.value());
                self.pc.jump(addr);
            }
            Instruction::SE(reg, byte) => {
                if self.registers[reg].value() == &byte {
                    self.pc.increment();
                }
            }
            Instruction::SNE(reg, byte) => {
                if self.registers[reg].value() != &byte {
                    self.pc.increment();
                }
            }
            Instruction::SEV(reg1, reg2) => {
                if self.registers[reg1].value() == self.registers[reg2].value() {
                    self.pc.increment();
                }
            }
            Instruction::LD(reg, byte) => {
                self.registers[reg].load(byte);
            }
            Instruction::ADD(reg, byte) => match self.registers[reg].value().add(byte) {
                Ok(value) => {
                    self.registers[reg].load(value);
                    self.vf.set();
                }
                Err(value) => {
                    self.registers[reg].load(value);
                    self.vf.unset();
                }
            },
            Instruction::LDV(reg1, reg2) => {
                let value = self.registers[reg2].value();
                self.registers[reg1].load(*value);
            }
            Instruction::OR(reg1, reg2) => {
                let value =
                    self.registers[reg1].value().as_u8() | self.registers[reg2].value().as_u8();
                self.registers[reg1].load(Byte(value));
            }
            Instruction::AND(reg1, reg2) => {
                let value =
                    self.registers[reg1].value().as_u8() & self.registers[reg2].value().as_u8();
                self.registers[reg1].load(Byte(value));
            }
            Instruction::XOR(reg1, reg2) => {
                let value =
                    self.registers[reg1].value().as_u8() ^ self.registers[reg2].value().as_u8();
                self.registers[reg1].load(Byte(value));
            }
            Instruction::ADDV(reg1, reg2) => {
                let value =
                    self.registers[reg1].value().as_u8() + self.registers[reg2].value().as_u8();
                self.registers[reg1].load(Byte(value));
                if value > 0xFF {
                    self.vf.set();
                } else {
                    self.vf.unset();
                };
            }
            Instruction::SUB(reg1, reg2) => {
                let res = self.registers[reg1]
                    .value()
                    .sub(*self.registers[reg2].value());
                match res {
                    Ok(value) => {
                        self.registers[reg1].load(value);
                        self.vf.set();
                    }
                    Err(value) => {
                        self.registers[reg1].load(value);
                        self.vf.unset();
                    }
                }
            }
            Instruction::SHR(reg1, reg2) => {
                if self.registers[reg1].value().as_u8() & 0x1 == 1 {
                    self.vf.set();
                } else {
                    self.vf.unset();
                }
                let value = self.registers[reg2].value().as_u8() >> 1;
                self.registers[reg1].load(Byte(value));
            }
            Instruction::SUBN(reg1, reg2) => {
                let res = self.registers[reg2]
                    .value()
                    .sub(*self.registers[reg1].value());
                match res {
                    Ok(value) => {
                        self.registers[reg1].load(value);
                        self.vf.set();
                    }
                    Err(value) => {
                        self.registers[reg1].load(value);
                        self.vf.unset();
                    }
                }
            }
            Instruction::SHL(reg1, reg2) => {
                if self.registers[reg1].value().as_u8() & 0x80 == 1 {
                    self.vf.set();
                } else {
                    self.vf.unset();
                }
                let value = self.registers[reg2].value().as_u8() << 1;
                self.registers[reg1].load(Byte(value));
            }
            Instruction::SNEV(reg1, reg2) => {
                if self.registers[reg1].value() != self.registers[reg2].value() {
                    self.pc.increment();
                }
            }
            Instruction::LDI(addr) => {
                self.i.load(addr);
            }
            Instruction::JPV0(addr) => {
                let jump_to = addr.as_u16() + self.registers[0].value().as_u8() as u16;
                self.pc.jump(Address::new(jump_to));
            }
            Instruction::RND(reg, byte) => {
                let random_byte = rand::random::<u8>();
                let value = random_byte & byte.as_u8();
                self.registers[reg].load(Byte(value));
            }
            Instruction::DRW(reg1, reg2, nibble) => {
                // TODO: Check if it works
                let x = self.registers[reg1].value().as_usize();
                let y = self.registers[reg2].value().as_usize();

                let mut sprites = vec![0; nibble.0 as usize];
                for i in 0..nibble.0 {
                    let sprite = self.ram.read(self.i.value() + i);
                    sprites[i as usize] = sprite.as_u8();
                }
                let collision = self.display.draw_sprite(x, y, &sprites);
                if collision != 0 {
                    self.vf.set();
                } else {
                    self.vf.unset();
                }
            }
            Instruction::SKP(reg) => {
                // TODO: Skip if key is pressed
                unimplemented!("SKP instruction not implemented");
            }
            Instruction::SKNP(reg) => {
                // TODO: Skip if key is not pressed
                unimplemented!("SKNP instruction not implemented");
            }
            Instruction::LDDT(reg) => {
                self.registers[reg].load(Byte(self.timers.get_delay_timer()));
            }
            Instruction::LDK(reg) => {
                // TODO: Wait for keypress, store in reg
                unimplemented!("LDK instruction not implemented");
            }
            Instruction::LDDTV(reg) => {
                let val = self.registers[reg].value();
                self.timers.set_delay_timer(val.as_u8());
            }
            Instruction::LDST(reg) => {
                let val = self.registers[reg].value();
                self.timers.set_sound_timer(val.as_u8());
            }
            Instruction::ADDI(reg) => {
                let addr = self.i.value().as_u16() + self.registers[reg].value().as_u8() as u16;
                self.i.load(Address::new(addr));
            }
            Instruction::LDF(reg) => {
                // TODO: I = location of sprite for digit Vx
                unimplemented!("LDF instruction not implemented");
            }
            Instruction::LDB(reg) => {
                let value = self.registers[reg].value().as_u8();
                let bcd = [Byte(value / 100), Byte((value / 10) % 10), Byte(value % 10)];
                for (i, &byte) in bcd.iter().enumerate() {
                    self.ram.load(self.i.value() + i, byte);
                }
            }
            Instruction::LDIV(reg) => {
                for i in 0..=reg {
                    self.ram.load(self.i.value() + i, *self.registers[i].value());
                }
            }
            Instruction::LDVI(reg) => {
                for i in 0..=reg {
                    self.registers[i].load(self.ram.read(self.i.value() + i));
                }
            }
            // Super chip-48 instructions
            Instruction::SCU(nibble) => unimplemented!(),
            Instruction::SCR => unimplemented!(),
            Instruction::SCL => unimplemented!(),
            Instruction::EXIT => unimplemented!(),
            Instruction::LOW => unimplemented!(),
            Instruction::HIGH => unimplemented!(),
            Instruction::DRW0(reg1, reg2) => unimplemented!(),
            Instruction::LDHF(reg) => unimplemented!(),
            Instruction::LDR(reg) => unimplemented!(),
            Instruction::LDRV(reg) => unimplemented!(),
        }
    }
}
