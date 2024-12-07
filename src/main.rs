use raylib::prelude::{Color, RaylibDraw};

mod cpu;
mod display;
mod keyboard;
mod timers;
mod datatypes;
mod registers;
mod util;
mod digits;
mod instruction;
mod stack;
mod ram;
mod chip8;

// 6377
fn main() {
    println!("Hello, chip-8!");

    let mut chip8 = chip8::Chip8::new("rom/6-keypad.ch8".to_string());
    chip8.run();

}
