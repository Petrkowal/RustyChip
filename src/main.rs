use crate::util::load_rom;

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

// 6377
fn main() {
    println!("Hello, chip-8!");

    // setup_display();
    // setup_input();
    // chip8_init();
    // chip8_load_game("games/INVADERS");
    //
    // while true {
    //     chip8_cycle();
    //     chip8_update_timers(); // Make sure to call this at 60Hz
    //     chip8_draw();
    //     chip8_input();
    // }


    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();
    // let window = video_subsystem
    //     .window("Chip-8", 640 * 2, 320 * 2)
    //     .position_centered()
    //     .build()
    //     .unwrap();
    // 
    // 
    // let mut event_pump = sdl_context.event_pump().unwrap();
    // 'running: loop {
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             sdl2::event::Event::Quit { .. } => break 'running,
    //             _ => {}
    //         }
    //     }
    // }
    
    let mut memory = ram::Ram::new();
    load_rom("rom/ibm_logo.ch8", &mut memory);
    
    let mut cpu = cpu::Cpu::new(memory);
    
    while true {
        cpu.cycle();
        
    }
    // input.iter().for_each(|x| println!("{:04x}", x));


}
