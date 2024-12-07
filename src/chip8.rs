use crate::cpu::Cpu;
use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::ram::Ram;
use raylib::prelude::RaylibDraw;
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

pub struct Chip8 {
    cpu: Cpu,
    ram: Rc<RefCell<Ram>>,
    display: Rc<RefCell<Display>>,
    keyboard: Rc<RefCell<Keyboard>>,
    rl: (raylib::RaylibHandle, raylib::RaylibThread),
}

impl Chip8 {
    pub fn new(path: String) -> Chip8 {
        let display = Rc::new(RefCell::new(Display::new()));
        let keyboard = Rc::new(RefCell::new(Keyboard::new()));
        let ram = Rc::new(RefCell::new(Ram::new()));
        ram.borrow_mut().load_rom(&path);

        let (rl, thread) = raylib::init().size(640, 320).title("Chip-8").build();

        Chip8 {
            cpu: Cpu::new(display.clone(), ram.clone(), keyboard.clone()),
            ram,
            display,
            keyboard,
            rl: (rl, thread),
        }
    }

    pub fn run(&mut self) {
        let mut next_time_to_cycle = std::time::Instant::now();
        let mut next_time_to_timer = std::time::Instant::now();
        let timers_rate = 60;
        let cpu_rate = 500;
        while !self.rl.0.window_should_close() {
            if std::time::Instant::now() < next_time_to_cycle && std::time::Instant::now() < next_time_to_timer {
                std::thread::sleep(min(next_time_to_cycle.duration_since(std::time::Instant::now()), next_time_to_timer.duration_since(std::time::Instant::now())).into());
            }
            if std::time::Instant::now() >= next_time_to_cycle {
                next_time_to_cycle = std::time::Instant::now() + std::time::Duration::from_millis(1000 / cpu_rate);
                self.cpu.cycle();
                self.handle_input();
                self.draw();
            }
            if std::time::Instant::now() >= next_time_to_timer {
                next_time_to_timer = std::time::Instant::now() + std::time::Duration::from_millis(1000 / timers_rate);
                self.cpu.update_timers();
            }
        }
    }

    fn draw(&mut self) {
        let display = self.display.borrow();
        let display = display.get_display();
        let mut handle = self.rl.0.begin_drawing(&self.rl.1);
        for y in 0..32 {
            for x in 0..64 {
                if display[x + y * 64] == 1 {
                    handle.draw_rectangle(x as i32 * 10, y as i32 * 10, 10, 10, raylib::prelude::Color::WHITE);
                } else {
                    handle.draw_rectangle(x as i32 * 10, y as i32 * 10, 10, 10, raylib::prelude::Color::BLACK);
                }
            }
        }
        
        // beep
        // if self.cpu.should_beep() {
        //     
        // }
    }

    fn handle_input(&mut self) {
        let mut keyboard = self.keyboard.borrow_mut();
        
        self.rl.0.get_key_pressed().iter().for_each(|key| {
            keyboard.press_key(*key);
        });
        
        keyboard.get_raylib_keys().iter().for_each(|key| {
            if self.rl.0.is_key_down(*key) {
                keyboard.press_key(*key);
            } else {
                keyboard.release_key(*key);
            }
        });
    }
}
