pub fn get_hex_digit(opcode: u16, position: u8, amount: u8) -> u16 {
    if position < 0 || amount <= 0 || position + amount > 4 {
        panic!("Invalid position or amount");
    }
    let shift_amount = 4 * (4 - position - amount);
    (opcode >> shift_amount) & ((1 << (4 * amount)) - 1)
}

pub fn get_hex_digit_usize(opcode: u16, position: u8, amount: u8) -> usize {
    get_hex_digit(opcode, position, amount) as usize
}

pub fn get_hex_digit_u8(opcode: u16, position: u8, amount: u8) -> u8 {
    get_hex_digit(opcode, position, amount) as u8
}
