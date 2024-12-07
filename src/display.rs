

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Display {
    display: [u8; 64 * 32],
}

impl Display {
    pub fn new() -> Display {
        Display {
            display: [0; 64 * 32],
        }
    }
}

impl Default for Display {
    fn default() -> Display {
        Display::new()
    }
}

impl Display {
    pub fn clear(&mut self) {
        self.display = [0; 64 * 32];
    }

    pub fn set_pixel(&mut self, x: usize, y: usize) {
        self.display[x + y * 64] = 1;
    }

    pub fn unset_pixel(&mut self, x: usize, y: usize) {
        self.display[x + y * 64] = 0;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.display[x + y * 64]
    }

    pub fn get_display(&self) -> &[u8; 64 * 32] {
        &self.display
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0;
        let x = x % 64;
        let y = y % 32;
        for (j, &sprite_byte) in sprite.iter().enumerate() {
            for i in 0..8 {
                let sprite_pixel = (sprite_byte >> (7 - i)) & 1;
                let x = x + i;
                let y = y + j;
                if x >= 64 || y >= 32 {
                    continue;
                }
                let pixel = self.get_pixel(x, y);
                collision |= pixel & sprite_pixel;
                self.display[x + y * 64] ^= sprite_pixel;
            }
        }

        collision
    }

    pub fn get_display_as_bytes(&self) -> Vec<u8> {
        self.display.to_vec()
    }

}
