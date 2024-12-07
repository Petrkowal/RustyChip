use crate::chip8::datatypes::datatypes::*;
use crate::chip8::io::display::Display;
use crate::chip8::cpu::instruction::Instruction;
use crate::chip8::io::keyboard::Keyboard;
use crate::chip8::ram::Ram;
use crate::chip8::cpu::registers::*;
use crate::chip8::cpu::stack::Stack;
use crate::chip8::cpu::timers::Timers;
use crate::chip8::util::util::*;
use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;

const REGISTER_COUNT: usize = 16;

enum State {
    Running,
    WaitingForKey(Instruction),
}
pub struct Cpu {
    registers: [Register<VRegisterMarker, Byte>; 16],
    i: Register<IRegisterMarker, Address>,
    stack: Stack,
    pc: Register<PCRegisterMarker, Address>,
    timers: Timers,
    display: Rc<RefCell<Display>>,
    ram: Rc<RefCell<Ram>>,
    keyboard: Rc<RefCell<Keyboard>>,
    state: State,
}

impl Cpu {
    pub fn new(display: Rc<RefCell<Display>>, ram: Rc<RefCell<Ram>>, keyboard: Rc<RefCell<Keyboard>>) -> Cpu {
        let registers = [Register::new(0); REGISTER_COUNT];
        let i = Register::new(0);
        let stack = Stack::new();
        let mut pc = Register::new(0);
        pc.jump(Address(0x200));
        let timers = Timers::new();
        Cpu {
            registers,
            i,
            stack,
            pc,
            timers,
            display,
            ram,
            keyboard,
            state: State::Running,
        }
    }

    pub fn cycle(&mut self) {
        match self.state {
            State::Running => {
                let opcode = self.fetch();
                let instruction = self.decode(opcode);
                self.execute(instruction);
            },
            State::WaitingForKey(instruction) => {
                self.execute(instruction);
            },
        }
    }

    pub fn update_timers(&mut self) {
        self.timers.update();
    }
    
    pub fn should_beep(&self) -> bool {
        self.timers.get_sound_timer() > 0
    }

    fn fetch(&mut self) -> u16 {
        let word = self.ram.borrow().read_word(self.pc.value());
        self.pc.increment();
        word
    }

    fn set_vf(&mut self) {
        self.registers[0xF].load(Byte(1));
    }

    fn unset_vf(&mut self) {
        self.registers[0xF].load(Byte(0));
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
            0xA000..=0xAFFF => Instruction::LDI(Address::new(opcode)),
            0xB000..=0xBFFF => {
                let reg = get_hex_digit_usize(opcode, 1, 1);
                let addr = Address::new(Address::mask(opcode));
                Instruction::JPVX(reg, addr)
            }
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
                self.display.borrow_mut().clear();
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
                    self.set_vf();
                }
                Err(value) => {
                    self.registers[reg].load(value);
                    self.unset_vf();
                }
            },
            Instruction::LDV(reg1, reg2) => {
                let value = *self.registers[reg2].value();
                self.registers[reg1].load(value);
            }
            Instruction::OR(reg1, reg2) => {
                let value =
                    self.registers[reg1].value().as_u8() | self.registers[reg2].value().as_u8();
                self.registers[reg1].load(Byte(value));
                self.unset_vf();
            }
            Instruction::AND(reg1, reg2) => {
                let value =
                    self.registers[reg1].value().as_u8() & self.registers[reg2].value().as_u8();
                self.registers[reg1].load(Byte(value));
                self.unset_vf();
            }
            Instruction::XOR(reg1, reg2) => {
                let value =
                    self.registers[reg1].value().as_u8() ^ self.registers[reg2].value().as_u8();
                self.registers[reg1].load(Byte(value));
                self.unset_vf();
            }
            Instruction::ADDV(reg1, reg2) => {
                let res = self.registers[reg1]
                    .value()
                    .add(*self.registers[reg2].value());
                match res {
                    Ok(value) => {
                        self.registers[reg1].load(value);
                        self.unset_vf();
                    }
                    Err(value) => {
                        self.registers[reg1].load(value);
                        self.set_vf();
                    }
                }
            }
            Instruction::SUB(reg1, reg2) => {
                let res = self.registers[reg1]
                    .value()
                    .sub(*self.registers[reg2].value());
                match res {
                    Ok(value) => {
                        self.registers[reg1].load(value);
                        self.set_vf();
                    }
                    Err(value) => {
                        self.registers[reg1].load(value);
                        self.unset_vf();
                    }
                }
            }
            Instruction::SHR(reg1, reg2) => {
                let digit = self.registers[reg1].value().as_u8() & 0x1;
                let value;
                if true { // Chip-8
                    value = self.registers[reg2].value().as_u8() >> 1; // Reg 1 or reg 2? Wtf
                }
                else { // Chip-48
                    value = self.registers[reg1].value().as_u8() >> 1;
                }
                self.registers[reg1].load(Byte(value));
                if digit == 1 {
                    self.set_vf();
                } else {
                    self.unset_vf();
                }
            }
            Instruction::SUBN(reg1, reg2) => {
                let res = self.registers[reg2]
                    .value()
                    .sub(*self.registers[reg1].value());
                match res {
                    Ok(value) => {
                        self.registers[reg1].load(value);
                        self.set_vf();
                    }
                    Err(value) => {
                        self.registers[reg1].load(value);
                        self.unset_vf();
                    }
                }
            }
            Instruction::SHL(reg1, reg2) => {
                let digit = self.registers[reg1].value().as_u8() >> 7;
                let value;
                if true { // Chip-8
                    value = self.registers[reg2].value().as_u8() << 1;
                }
                else { // Chip-48
                    value = self.registers[reg1].value().as_u8() << 1;
                }
                self.registers[reg1].load(Byte(value));
                if digit == 1 {
                    self.set_vf();
                } else {
                    self.unset_vf();
                }
            }
            Instruction::SNEV(reg1, reg2) => {
                if self.registers[reg1].value() != self.registers[reg2].value() {
                    self.pc.increment();
                }
            }
            Instruction::LDI(addr) => {
                self.i.load(addr);
            }
            Instruction::JPVX(reg, addr) => {
                let jmp_addr;
                if true { // Chip-8
                    jmp_addr = Address(addr.0 + self.registers[0].value().as_u8() as u16);
                }
                else { // Super chip-48
                    jmp_addr = Address(addr.0 + self.registers[reg].value().as_u8() as u16);
                }
                self.pc.jump(jmp_addr);
            }
            Instruction::RND(reg, byte) => {
                let random_byte = rand::random::<u8>();
                let value = random_byte & byte.as_u8();
                self.registers[reg].load(Byte(value));
            }
            Instruction::DRW(reg1, reg2, nibble) => {
                let x = self.registers[reg1].value().as_usize();
                let y = self.registers[reg2].value().as_usize();
                let n = nibble.0 as usize;

                let mut sprites: Vec<Byte> = Vec::new();
                for i in 0..n {
                    sprites.push(self.ram.borrow().read(self.i.value() + i));
                }

                let mut collision = false;
                let sprites: &[u8] = &sprites.iter().map(|byte| byte.0).collect::<Vec<u8>>();
                if self.display.borrow_mut().draw_sprite(x, y, sprites) != 0 {
                    collision = true;
                }

                if collision {
                    self.set_vf();
                } else {
                    self.unset_vf();
                }
            }
            Instruction::SKP(reg) => {
                if self
                    .keyboard
                    .borrow()
                    .is_pressed(self.registers[reg].value().as_u8())
                {
                    self.pc.increment();
                }
            }
            Instruction::SKNP(reg) => {
                if !self
                    .keyboard
                    .borrow()
                    .is_pressed(self.registers[reg].value().as_u8())
                {
                    self.pc.increment();
                }
            }
            Instruction::LDDT(reg) => {
                self.registers[reg].load(Byte(self.timers.get_delay_timer()));
            }
            Instruction::LDK(reg) => {

                let kboard = self.keyboard.borrow();
                for i in 0..16 {
                    if kboard.is_pressed(i) {
                        self.registers[reg].load(Byte(i));
                        self.state = State::Running;
                        return;
                    }
                }

                self.state = State::WaitingForKey(Instruction::LDK(reg));
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
                let digit = self.registers[reg].value().as_u8();
                let addr = Address(0x50 + digit as u16 * 5);
                self.i.load(addr);
            }
            Instruction::LDB(reg) => {
                let value = self.registers[reg].value().as_u8();
                let bcd = [Byte(value / 100), Byte((value / 10) % 10), Byte(value % 10)];
                for (i, &byte) in bcd.iter().enumerate() {
                    self.ram.borrow_mut().load(self.i.value() + i, byte);
                }
            }
            Instruction::LDIV(reg) => {
                for i in 0..=reg {
                    self.ram
                        .borrow_mut()
                        .load(*self.i.value(), *self.registers[i].value());
                    self.i.increment();
                }
            }
            Instruction::LDVI(reg) => {
                for i in 0..=reg {
                    self.registers[i].load(self.ram.borrow().read(*self.i.value()));
                    self.i.increment();
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
