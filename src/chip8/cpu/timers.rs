
pub struct Timers {
    delay_timer: u8,
    sound_timer: u8,
}



impl Default for Timers {
    fn default() -> Timers {
        Timers::new()
    }
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            delay_timer: 0,
            sound_timer: 0,
        }
    }
    
    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn set_sound_timer(&mut self, value: u8) {
        self.sound_timer = value;
    }

    pub fn decrement(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}
