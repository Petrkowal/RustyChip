
mod chip8;

// 6377
fn main() {
    println!("Hello, chip-8!");

    let mut chip8 = chip8::Chip8::new("rom/Pong.ch8".to_string());
    chip8.run();

}
