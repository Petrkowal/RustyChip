use crate::chip8::cpu::Cpu;
use crate::chip8::io::display::Display;
use crate::chip8::io::keyboard::Keyboard;
use crate::chip8::ram::Ram;
use raylib::prelude::RaylibDraw;
use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;

// chip settings
pub struct ChipSettings {
    rom: String,
    size: (u32, u32),
    cpu_rate: u64,
    timers_rate: u64,
    colors: (u32, u32),
    swap_yz: bool,
    beep: bool,
}

impl ChipSettings {
    pub fn new(
        rom: String,
        size: (u32, u32),
        cpu_rate: u64,
        timers_rate: u64,
        colors: (u32, u32),
        swap_yz: bool,
        beep: bool,
    ) -> ChipSettings {
        let size = ChipSettings::fix_size(size);

        let colors = ChipSettings::fix_colors(colors.0, colors.1);

        if cpu_rate / timers_rate != 500 / 60 {
            log::warn!("CPU rate and timers rate are not in the correct ratio (500/60), this may cause issues.");
        }

        ChipSettings {
            rom,
            size,
            cpu_rate,
            timers_rate,
            colors,
            swap_yz,
            beep,
        }
    }

    fn fix_size(size: (u32, u32)) -> (u32, u32) {
        match (size.0, size.1) {
            (0, 0) => {
                (640, 320)
            }
            (width, height) if width != 0 && height != 0 => {
                if width / height != 2 {
                    log::warn!("Width and height are not in the correct ratio (64/32). This may cause some (visual) issues. Set just one of them or try to find a correct ratio.");
                }
                (width, height)
            }
            (width, 0) => {
                let width = width - width % 64;
                let height = width / 2;
                (width, height)
            }
            (0, height) => {
                let height = height - height % 32;
                let width = height * 2;
                (width, height)
            }
            _ => unreachable!("Rust is lying to me"),
        }
    }

    fn fix_colors(fg_color: u32, bg_color: u32) -> (u32, u32) {
        if fg_color == bg_color {
            log::warn!(
                "Foreground and background colors are the same. Enjoy your invisible screen!"
            );
        }
        (fg_color, bg_color)
    }
}

pub struct Chip8 {
    cpu: Cpu,
    ram: Rc<RefCell<Ram>>,
    display: Rc<RefCell<Display>>,
    keyboard: Rc<RefCell<Keyboard>>,
    rl: (raylib::RaylibHandle, raylib::RaylibThread),
    pixel_size: (i32, i32),
    settings: ChipSettings,
}

impl Chip8 {
    pub fn new(settings: ChipSettings) -> Chip8 {
        let display = Rc::new(RefCell::new(Display::new()));
        let keyboard = Rc::new(RefCell::new(Keyboard::new(settings.swap_yz)));
        let ram = Rc::new(RefCell::new(Ram::new()));
        ram.borrow_mut().load_rom(settings.rom.as_str());

        let cpu = Cpu::new(display.clone(), ram.clone(), keyboard.clone());

        // make title the name of the rom (extracted from the path)
        let name = settings
            .rom
            .split('/')
            .last()
            .unwrap_or("")
            .split('.')
            .next()
            .unwrap_or("");
        let (rl, thread) = raylib::init()
            .size(settings.size.0 as i32, settings.size.1 as i32)
            .title(&format!("Rusty Chip-8: {}", name))
            .build();
        
        let pixel_size = (settings.size.0 as i32 / 64, settings.size.1 as i32 / 32);

        Chip8 {
            cpu,
            ram,
            display,
            keyboard,
            rl: (rl, thread),
            pixel_size,
            settings,
        }
    }

    pub fn run(&mut self) {
        let mut next_time_to_cycle = std::time::Instant::now();
        let mut next_time_to_timer = std::time::Instant::now();
        let timers_rate = self.settings.timers_rate;
        let cpu_rate = self.settings.cpu_rate;
        while !self.rl.0.window_should_close() {
            if std::time::Instant::now() < next_time_to_cycle
                && std::time::Instant::now() < next_time_to_timer
            {
                std::thread::sleep(
                    min(
                        next_time_to_cycle.duration_since(std::time::Instant::now()),
                        next_time_to_timer.duration_since(std::time::Instant::now()),
                    ),
                );
            }
            if std::time::Instant::now() >= next_time_to_cycle {
                next_time_to_cycle =
                    std::time::Instant::now() + std::time::Duration::from_millis(1000 / cpu_rate);
                self.cpu.cycle();
                self.handle_input();
                self.draw();
            }
            if std::time::Instant::now() >= next_time_to_timer {
                next_time_to_timer = std::time::Instant::now()
                    + std::time::Duration::from_millis(1000 / timers_rate);
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
                    handle.draw_rectangle(
                        x as i32 * self.pixel_size.0,
                        y as i32 * self.pixel_size.1,
                        self.pixel_size.0,
                        self.pixel_size.1,
                        raylib::prelude::Color::new(
                            (self.settings.colors.0 >> 16) as u8,
                            (self.settings.colors.0 >> 8) as u8,
                            self.settings.colors.0 as u8,
                            255,
                        ),
                    );
                } else {
                    handle.draw_rectangle(
                        x as i32 * self.pixel_size.0,
                        y as i32 * self.pixel_size.1,
                        self.pixel_size.0,
                        self.pixel_size.1,
                        raylib::prelude::Color::new(
                            (self.settings.colors.1 >> 16) as u8,
                            (self.settings.colors.1 >> 8) as u8,
                            self.settings.colors.1 as u8,
                            255,
                        ),
                    );
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
